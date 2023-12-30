// region: Imports
#[allow(unused_imports)]
use super::surreal_store::base_crud::{
    base_create, base_create_with_id, base_create_with_ulid, base_delete, base_get, base_info,
    base_list, base_update,
};
use super::surreal_store::{
    general_crud::{general_create, general_list},
    SurrealBmc,
};
use crate::model::ModelManager;
#[allow(unused_imports)]
use crate::model::{Error, Result};

use bigdecimal::BigDecimal;
use chrono::{DateTime, FixedOffset, Local, NaiveDate};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use surrealdb::sql::Thing;
use ulid::Ulid;
use uuid::Uuid;
// endregion: Imports

// region: Types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbededStruct {
    pub last_name: String,
    pub age: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HThing {
    pub id: String,
    pub tb: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataTypes {
    #[serde(deserialize_with = "deserialize_thing_as_hthing")]
    id: HThing,
    // #[serde(rename(deserialize = "id"))]
    // #[serde(deserialize_with = "deserialize_thing_id_as_str")]
    // id: String,
    // #[serde(alias = "id")]
    // #[serde(deserialize_with = "deserialize_thing_tb_as_str", rename(deserialize = "id"))]
    // tb: String,
    // id: Thing,
    #[serde(flatten)]
    pub data: DataTypesForCreate,
}

fn deserialize_thing_as_hthing<'de, D>(deserializer: D) -> std::result::Result<HThing, D::Error>
where
    D: de::Deserializer<'de>,
{
    let t: Thing = de::Deserialize::deserialize(deserializer)?;
    println!("Got Thing to Deserialize {:?}", &t);
    Ok(HThing {
        id: t.id.to_raw(),
        tb: t.tb,
    })
}

fn deserialize_thing_id_as_str<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let t: Thing = de::Deserialize::deserialize(deserializer)?;
    println!("Got Thing to Deserialize {:?}", &t);
    Ok(t.id.to_raw())
}

fn deserialize_thing_tb_as_str<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let t: Thing = de::Deserialize::deserialize(deserializer)?;
    println!("Got Thing to Deserialize {:?}", &t);
    Ok(t.tb)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTypesForCreate {
    pub name: String,
    pub float: f64,
    pub integer: i64,
    pub embed: EmbededStruct,
    pub list: Vec<i32>,
    pub tuple: (i32, String),
    pub hash: HashMap<String, i32>,
    #[serde(with = "uuid::serde::compact")]
    pub uuid: Uuid,
    pub ulid: Ulid,
    pub json: Value,
    pub datetime: DateTime<Local>,
    pub naivedate: NaiveDate,
    pub datetimefixed: DateTime<FixedOffset>,
    pub record: Thing,
    pub record_string: String,
    pub money: BigDecimal,
    pub opt: Option<i32>,
    pub optnone: Option<i32>,
}

// endregion: Types

// region: DataTypesBmc
/// Holds the functions and the table name for this Model.
pub struct DataTypesBmc;

impl SurrealBmc for DataTypesBmc {
    const TABLE: &'static str = "datatypes";
}

impl DataTypesBmc {
    pub async fn create(mm: &ModelManager, data: DataTypesForCreate) -> Result<DataTypes> {
        let res = general_create::<_, _>(mm, Self::TABLE, data).await?;

        Ok(res)
    }

    pub async fn list(mm: &ModelManager) -> Result<Vec<DataTypes>> {
        let res = general_list::<_>(mm, Self::TABLE).await?;

        Ok(res)
    }

    // pub async fn info(mm: &ModelManager) -> Result<Value> {
    //     let res = base_info::<Self>(mm).await?;

    //     Ok(res)
    // }
}

// endregion: DataTypesBmc
