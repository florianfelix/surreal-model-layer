use chrono::{DateTime, Local, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::surreal_store::{
    base_crud::{
        base_create, base_create_with_id, base_create_with_ulid, base_delete, base_get, base_list,
        base_update,
    },
    SurrealBmc,
};
// use super::surreal_store::transaction::TransactionBmcBase;
use crate::model::ModelManager;

#[allow(unused_imports)]
use crate::model::{Error, Result};

// region: Types
#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    pub id: Thing,
    pub title: Option<String>,
    pub label: Option<Thing>,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransactionForCreate {
    pub title: String,
    pub label: Option<Thing>,
    pub amount: f64,
    // pub label: Option<(String, String)>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransactionForUpdate {
    pub title: Option<String>,
    pub label: Option<Thing>,
    pub amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionSrv {
	pub id: Thing,
	#[serde(flatten)]
	pub data: TransactionContentSrv,
}
// endregion: Types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionContentSrv {
	pub title: String,
	// pub amount: BigDecimal,
	pub amount: f64,
	// pub created_at: String,
	// pub created_at: DateTime<Local>,
	pub date: DateTime<Utc>,
	pub immo: Vec<Thing>,
	pub konto: Vec<Thing>,
}
impl TransactionContentSrv {
    pub fn new() -> Self {
        Self {
            title: "NewTitle".to_string(),
            amount: 12.42,
            date: Local::now().into(),
            immo: vec![("immo", "steendiek").into()],
            konto: vec![("konto", "wasser").into()],
        }
    }
}

// region: TransactionBmc
pub struct TransactionBmc;

impl SurrealBmc for TransactionBmc {
    const TABLE: &'static str = "transaction";
}

impl TransactionBmc {
    // CREATE
    pub async fn create(mm: &ModelManager, ta_create: TransactionForCreate) -> Result<TransactionSrv> {
        let res = base_create::<Self, _, _>(mm, ta_create).await?;

        Ok(res)
    }
    // CREATE
    pub async fn create_ulid(
        mm: &ModelManager,
        ta_create: TransactionContentSrv,
    ) -> Result<TransactionSrv> {
        let res = base_create_with_ulid::<Self, _, _>(mm, ta_create).await?;

        Ok(res)
    }

    // CREATE WITH ID
    pub async fn create_with_id(
        mm: &ModelManager,
        id: String,
        ta_create: TransactionForCreate,
    ) -> Result<TransactionSrv> {
        let res = base_create_with_id::<Self, _, _>(mm, ta_create, &id).await?;

        Ok(res)
    }

    // GET
    pub async fn get(mm: &ModelManager, id: String) -> Result<TransactionSrv> {
        let res = base_get::<Self, _>(mm, &id).await?;

        Ok(res)
    }

    // LIST ALL
    pub async fn list(mm: &ModelManager) -> Result<Vec<TransactionSrv>> {
        let res = base_list::<Self, _>(mm).await?;

        Ok(res)
    }

    // UPDATE
    pub async fn update(
        mm: &ModelManager,
        id: String,
        data: TransactionForUpdate,
    ) -> Result<TransactionSrv> {
        let res = base_update::<Self, _, _>(mm, data, &id).await?;

        Ok(res)
    }

    // DELETE
    pub async fn delete(mm: &ModelManager, id: String) -> Result<TransactionSrv> {
        let res = base_delete::<Self, _>(mm, &id).await?;

        Ok(res)
    }
}
// endregion: TransactionBmc
