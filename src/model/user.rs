/// `user` table with needed structs and functions
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::{sql::Thing, opt::auth::{Scope, Jwt, Root}};
use uuid;
use uuid::Uuid;

use crate::model::ModelManager;

#[allow(unused_imports)]
use crate::model::{Error, Result};

#[allow(unused_imports)]
use super::surreal_store::base_crud::{
    base_create_with_id, base_delete, base_get, base_list, base_update,
};
use super::surreal_store::{self, SurrealBmc};

// region: Types
#[derive(Debug, Clone, Deserialize)]
pub struct User {
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Thing,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserForCreate<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserForLogin {
    // pub id: Thing,
    pub email: String,
    pub password: String,
}

// endregion: Types

/// Unitstruct which holds all functions for the `user` table
pub struct UserBmc;

/// The name of the table in the database
impl SurrealBmc for UserBmc {
    const TABLE: &'static str = "user";
}

// region: UserBmc
impl UserBmc {
    pub async fn signup(mm: &ModelManager, name: &str, email: &str, password: &str) -> Result<Jwt> {
        let srdb = mm.srdb().clone();
        srdb.use_ns("test").use_db("test").await?;

        let credentials = UserForCreate {
            name: name,
            email: email,
            password: password,
        };

        let jwt = srdb.signup(Scope {
            namespace: "test",
            database: "test",
            scope: "user",
            params: credentials,
        })
        .await?;

        Ok(jwt)
    }

    pub async fn signin(mm: &ModelManager, email: &str, password: &str) -> Result<Jwt> {
        let srdb = mm.srdb().clone();


        srdb.use_ns("test").use_db("test").await?;

        let jwt = srdb.signin(Scope {
            namespace: "test",
            database: "test",
            scope: "user",
            params: UserForLogin {
                email: email.to_string(),
                password: password.to_string(),
            },
        }).await?;

        // dbg!(&jwt.as_insecure_token());

        Ok(jwt)
    }

    /// Create a new user with new_user_name as id in table user
    pub async fn create(
        mm: &ModelManager,
        user_name: &str,
        email: &str,
        password: &str,
    ) -> Result<User> {
        let signup_sql = format!(
            "
            CREATE user CONTENT {{
                name: '{}',
                email: '{}',
                password: crypto::argon2::generate('{}')
            }}
        ;
        ",
            user_name, email, password
        );

        let id: Thing = ("user".to_string(), email.to_string()).into();

        // println!("SIGNUP: {}", &signup_sql);

        let mut response = mm.srdb().clone().query(signup_sql).bind(("id", id)).await?;

        let user: Option<User> = response.take(0)?;

        if let Some(user) = user {
            return Ok(user);
        } else {
            return Err(Error::Store(surreal_store::Error::FailedToCreate));
        }
        // let res = base_create_with_id::<Self, _, _>(mm, new_user, &email).await?;

        // Ok(res)
    }

    pub async fn login(mm: &ModelManager, email: &str, password: &str) -> Result<User> {
        let signin_sql = format!(
            "
            SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password, $password)
        "
        );

        let mut response = mm
            .srdb()
            .clone()
            .query(signin_sql)
            .bind(("email", email.to_string()))
            .bind(("password", password.to_string()))
            .await?;

        let user: Option<Value> = response.take(0)?;
        // dbg!(&user);

        if let Some(user) = user {
            return Ok(serde_json::from_value(user).unwrap());
        } else {
            return Err(Error::Store(surreal_store::Error::FailedToCreate));
        }
    }
    /// Get a user by id (String)
    pub async fn get(mm: &ModelManager, id: String) -> Result<User> {
        let res = base_get::<Self, _>(mm, &id).await?;

        Ok(res)
    }

    /// List all users
    pub async fn list(mm: &ModelManager) -> Result<Vec<User>> {
        let res = base_list::<Self, _>(mm).await?;

        Ok(res)
    }

    // UPDATE BY ID
}

// endregion: UserBmc
