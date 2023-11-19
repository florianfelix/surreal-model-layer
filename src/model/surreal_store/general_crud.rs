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
    pub id: Thing,
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

pub async fn general_get<R>(mm: &ModelManager, tb: &str, id: &str) -> Result<R>
where
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let selected: Option<R> = srdb.select((tb, id)).await?;

    if let Some(selected) = selected {
        return Ok(selected);
    } else {
        return Err(Error::FailedToGet(id.into()));
    }
}

pub async fn general_update<C, R>(mm: &ModelManager, tb: &str, id: &str, content: C) -> Result<R>
where
    C: Serialize,
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let updated: Option<R> = srdb.update((tb, id)).content(content).await?;

    if let Some(updated) = updated {
        return Ok(updated);
    } else {
        return Err(Error::FailedToUpdate(id.into()));
    }
}

/// List all records
pub async fn general_list<R>(mm: &ModelManager, tb: &str) -> Result<Vec<R>>
where
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let listed: Vec<R> = srdb.select(tb).await?;

    Ok(listed)
}

/// Delete a single record based on the given id
pub async fn general_delete<R>(mm: &ModelManager, tb: &str, id: &str) -> Result<R>
where
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let deleted: Option<R> = srdb.delete((tb, id)).await?;

    if let Some(deleted) = deleted {
        return Ok(deleted);
    } else {
        return Err(Error::FailedToDeleteIdNotFound(id.into()));
    }
}
