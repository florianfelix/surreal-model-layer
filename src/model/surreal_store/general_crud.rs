use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::Thing;

use crate::model::{
    surreal_store::{Error, Result},
    ModelManager,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record<C>
where
    C: Serialize,
{
    id: Thing,
    #[serde(flatten)]
    pub data: C,
}

pub async fn general_create<C, R>(mm: &ModelManager, tb: &str, content: C) -> Result<R>
where
    C: Serialize,
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let q = "CREATE type::table($table) CONTENT $content;";

    let mut response = srdb
        .query(q)
        .bind(("table", tb))
        .bind(("content", &content))
        .await?;

    let created: Option<R> = response.take(0)?;

    if let Some(created) = created {
        return Ok(created);
    } else {
        return Err(Error::FailedToCreate);
    }
}
