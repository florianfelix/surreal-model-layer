//! This is an example of a Surreal Model Layer
//!
//! The idea is to copy and adapt the model module.
//!
//! main is only used to test the things in the model module

#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::collections::HashMap;

use bigdecimal::BigDecimal;
use chrono::Local;
use model::datatypes::DataTypesForCreate;
use surrealdb::sql::Thing;
use ulid::Ulid;
use uuid::Uuid;

use crate::model::datatypes::{DataTypesBmc, EmbededStruct};
use crate::model::edge::EdgeBmc;
use crate::model::label::{LabelBmc, LabelForCreate, LabelForUpdate};
use crate::model::transaction::{TransactionBmc, TransactionForCreate, TransactionForUpdate};
use crate::model::user::UserBmc;
use crate::model::ModelManager;

pub use self::error::{Error, Result};

/// holds the model errors
mod error;

/// BackendModelControllers for custom tables. Start looking here.
///
/// In `surreal_store` are the base functions for the specific BMCs.
/// Basic CRUD should be possible with those.
/// Anything more complicated and/or custom for specific tables
/// should then be implemented in their own modules.
mod model;

/// call the test_... functions
#[tokio::main]
async fn main() -> Result<()> {
    let mm = ModelManager::new().await?;
    test_delete_tables(&mm).await?;
    test_create_schemaful(&mm).await?;

    // test_edges(&mm).await?;

    // test_datatypes(&mm).await?;

    test_users(&mm).await?;

    // test_labelbmc(&mm).await?;

    // test_transactionbmc(&mm).await?;

    Ok(())
}

/// start with deleting existing tables to begin afresh
async fn test_delete_tables(mm: &ModelManager) -> Result<()> {
    let srdb = mm.srdb().clone();
    let sql = "
    REMOVE TABLE transaction;
    REMOVE TABLE label;
    REMOVE TABLE user;
    REMOVE TABLE datatypes;
    REMOVE TABLE edge;
    ";
    let _res = srdb.query(sql).await?;
    // dbg!(_res);
    Ok(())
}

async fn test_create_schemaful(mm: &ModelManager) -> Result<()> {
    let srdb = mm.srdb().clone();

    let sql_ta_table = "
    BEGIN TRANSACTION;

    DEFINE TABLE transaction;
    DEFINE FIELD title ON TABLE transaction TYPE option<string>;
    DEFINE FIELD label ON TABLE transaction TYPE option<record>;
    DEFINE FIELD amount ON TABLE transaction TYPE float;

    COMMIT TRANSACTION;
    ";

    let schemaful_query = srdb.query(sql_ta_table).await?;
    // dbg!(schemaful_query);

    //PERMISSIONS FOR create, select, update, delete WHERE id = $auth.id

    let sql_user_table = r#"
    BEGIN TRANSACTION;

    DEFINE TABLE user SCHEMAFULL
	PERMISSIONS
    FOR update, delete WHERE id = $auth.id;

    DEFINE FIELD name ON user TYPE string;
    DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
    DEFINE FIELD password ON user TYPE string;

    DEFINE INDEX email ON user FIELDS email UNIQUE;

    DEFINE SCOPE user SESSION 24h
    SIGNUP (
		CREATE user CONTENT {
			name: $name,
			email: $email,
			password: crypto::argon2::generate($password)
		}
	)
    SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password, $password) );

    COMMIT TRANSACTION;
    "#;

    let schemaful_query = srdb.query(sql_user_table).await?;
    // dbg!(schemaful_query);

    Ok(())
}

async fn test_edges(mm: &ModelManager) -> Result<()> {
    // first create two records in two tables to connect to each other
    let username = "BobTheBuilder";
    let email = "bob@builder.com";
    let password = "clear_password";
    let user = UserBmc::create(mm, &username, email, password).await?;
    dbg!(&user);

    let tac = TransactionForCreate {
        title: "Purchase plot of land".into(),
        label: None,
        amount: 33.45,
    };
    let ta = TransactionBmc::create(&mm, tac.clone()).await?;
    let ta2 = TransactionBmc::create(&mm, tac).await?;
    dbg!(&ta);

    let user_id = user.id.clone();
    let ta_id = ta.id.clone();
    let ta_id2 = ta2.id.clone();

    // now connect the two records
    let conn = EdgeBmc::connect(mm, user_id.clone(), ta_id).await?;
    let conn = EdgeBmc::connect(mm, user_id.clone(), ta_id2).await?;
    // dbg!(&conn);

    // list all created edges
    let edges = EdgeBmc::list(mm).await?;
    dbg!(&edges);

    // list outgoing connections
    let outgoing = EdgeBmc::list_outgoing(mm).await?;
    dbg!(&outgoing);

    // list incoming connections
    let incoming = EdgeBmc::list_incoming(mm).await?;
    dbg!(&incoming);

    // list connected to id
    let ids = EdgeBmc::list_connected_ids_from_id(mm, user_id).await?;
    dbg!(&ids);

    // delete one end of the edge
    let deleted = TransactionBmc::delete(&mm, ta.id.id.to_raw()).await?;
    dbg!(&deleted);

    // list all remaining? edges
    let edges_remaining = EdgeBmc::list(mm).await?;
    dbg!(&edges_remaining);

    Ok(())
}

// region: Playground functions
/// playground function to test the datatypes roundtrip
async fn test_datatypes(mm: &ModelManager) -> Result<()> {
    let embed = EmbededStruct {
        last_name: "Smith".into(),
        age: 32,
    };

    let jsonval = serde_json::to_value(&embed).unwrap();

    let record = Thing {
        tb: "tablename".to_string(),
        id: "recordid".into(),
    };

    // record parsed from raw
    let record: Thing = "tablename:['list', 'identifier']}".parse().unwrap();

    let data = DataTypesForCreate {
        name: "Test".into(),
        float: 1.33,
        integer: 42,
        embed: embed,
        list: vec![1, 2, 3],
        tuple: (1, "the comment".into()),
        hash: HashMap::from([("key".to_string(), 1)]),
        uuid: Uuid::new_v4(),
        ulid: Ulid::new(),
        json: jsonval,
        datetime: Local::now(),
        naivedate: Local::now().naive_local().date(),
        record: record.clone(),
        record_string: record.to_raw(),
        money: BigDecimal::from(((12.55 * 100.) as i64, 2)),
    };

    // let data = DataTypesForCreate::default();

    let created = DataTypesBmc::create(mm, data).await?;
    dbg!(created);

    let listed = DataTypesBmc::list(mm).await?;
    // dbg!(listed);

    let inf = DataTypesBmc::info(mm).await?;
    dbg!(inf);

    Ok(())
}

/// test UserBmc
async fn test_users(mm: &ModelManager) -> Result<()> {

    let jwt = UserBmc::signup(mm, "signupname", "signup@user.com", "clear_password").await?;
    let jwt = UserBmc::signup(mm, "signupname2", "signup2@user.com", "clear_password").await?;
    // let jwt = UserBmc::signin(mm, "signup@user.com", "clear_password").await?;


    let new_user = UserBmc::create(mm, "FirstUser", "first@user.com", "clear_password").await?;
    // let jwt = UserBmc::signin(mm, "first@user.com", "clear_password").await?;
    // dbg!(&new_user);

    // let signed_in = UserBmc::login(mm, new_user_email, new_user_password).await?;
    // dbg!(&signed_in);

    // let new_user = UserBmc::create(mm, "SecondUser", "second@user.com", "clear_password").await?;
    // dbg!(&new_user);

    // let signed_in = UserBmc::login(mm, new_user_email, new_user_password).await?;
    // dbg!(&signed_in);
    // let gotten = UserBmc::get(mm, new_user.id.id.to_string()).await?;
    // dbg!(gotten);

    let all_users = UserBmc::list(mm).await?;
    // dbg!(all_users);
    println!("ALL USERS:\n{:#?}", all_users);

    Ok(())
}

/// test TransactionBmc
async fn test_transactionbmc(mm: &ModelManager) -> Result<()> {
    // CREATE -- NORMAL RANDOM ID
    let ta1 = TransactionForCreate {
        title: "First Transaction".into(),
        label: None,
        // amount: 33.45,
        amount: 33.45,
    };
    let created = TransactionBmc::create(&mm, ta1).await?;
    dbg!(&created);

    // CREATE -- ULID
    let ta2 = TransactionForCreate {
        title: "Second Transaction".into(),
        label: None,
        amount: 55.45,
    };
    let created_ulid = TransactionBmc::create_ulid(&mm, ta2).await?;
    dbg!(&created_ulid);

    // CREATE -- ID
    let ta3 = TransactionForCreate {
        title: "Third Transaction".into(),
        label: Some(Thing {
            tb: "label".to_string(),
            id: "My first Label".into(),
        }),
        amount: 89.12,
    };
    let created_id = TransactionBmc::create_with_id(&mm, "Third Transaction".into(), ta3).await?;
    dbg!(&created_id);

    // GET -- BY ID
    let gotten = TransactionBmc::get(&mm, "Third Transaction".into()).await?;
    // dbg!(gotten);

    // UPDATE -- BY ID
    let taupdate = TransactionForUpdate {
        title: Some("Third Transaction Updated".to_string()),
        label: Some(Thing {
            tb: "label".to_string(),
            id: "The Second Label".into(),
        }),
        amount: Some(64.89),
    };
    let updated =
        TransactionBmc::update(&mm, created_ulid.clone().id.id.to_raw(), taupdate).await?;
    // dbg!(updated);

    // LIST -- ALL
    let listed = TransactionBmc::list(&mm).await?;
    dbg!(listed);

    // DELETE -- BY ID
    let delted = TransactionBmc::delete(&mm, created.id.id.to_raw()).await?;
    dbg!(delted);

    Ok(())
}

/// test LabelBmc
async fn test_labelbmc(mm: &ModelManager) -> Result<()> {
    // CREATE LABEL
    let created = LabelBmc::create(
        &mm,
        LabelForCreate {
            label_name: "My First Label".into(),
        },
    )
    .await?;
    dbg!(created);

    let created2 = LabelBmc::create_with_id(
        &mm,
        LabelForCreate {
            label_name: "My Second Label".into(),
        },
        "My Second Label".into(),
    )
    .await?;
    dbg!(created2);

    // // GET BY ID
    let gotten = LabelBmc::get(&mm, "My Second Label".into()).await?;
    dbg!(gotten);

    // // UPDATE BY ID
    let updated = LabelBmc::update(
        &mm,
        "My Second Label".into(),
        LabelForUpdate {
            label_name: "My Third Label".into(),
        },
    )
    .await?;
    dbg!(updated);

    // // DELETE BY ID
    let delted = LabelBmc::delete(&mm, "My Second Label".into()).await?;
    dbg!(delted);

    let labels = LabelBmc::list(&mm).await?;
    dbg!(labels);

    Ok(())
}

// endregion: Playground functions
