// region: Imports
#[allow(unused_imports)]
use super::surreal_store::base_crud::{
    base_create, base_create_with_id, base_create_with_ulid, base_delete, base_get, base_info,
    base_list, base_update,
};
use super::surreal_store::SurrealBmc;
use crate::model::ModelManager;
#[allow(unused_imports)]
use crate::model::{Error, Result};

use bigdecimal::BigDecimal;
use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};
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
pub struct DataTypes {
    id: Thing,
    #[serde(flatten)]
    pub data: DataTypesForCreate,
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
    pub record: Thing,
    pub record_string: String,
    pub money: BigDecimal,
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
        let res = base_create::<Self, _, _>(mm, data).await?;

        Ok(res)
    }

    pub async fn list(mm: &ModelManager) -> Result<Vec<DataTypes>> {
        let res = base_list::<Self, _>(mm).await?;

        Ok(res)
    }

    pub async fn info(mm: &ModelManager) -> Result<Value> {
        let res = base_info::<Self>(mm).await?;

        Ok(res)
    }
}

// endregion: DataTypesBmc
