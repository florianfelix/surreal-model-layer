use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use surrealdb::sql::Thing;

use crate::model::{
    surreal_store::{Error, Result},
    ModelManager,
};

use super::SurrealBmc;

/// Create the data in `content` in the `TABLE` of the calling model
/// (Set by the impl of `SurrealBmc` in the parent calling model).
/// Uses a standard random id.
pub async fn base_create<MC, C, R>(mm: &ModelManager, content: C) -> Result<R>
where
    MC: SurrealBmc,
    C: Serialize,
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let q = "CREATE type::table($table) CONTENT $content;";

    let mut response = srdb
        .query(q)
        .bind(("table", MC::TABLE))
        .bind(("content", &content))
        .await?;

    let created: Option<R> = response.take(0)?;

    if let Some(created) = created {
        return Ok(created);
    } else {
        return Err(Error::FailedToCreate);
    }
}

/// Same as `base_create`, but uses a ULID as the id
pub async fn base_create_with_ulid<MC, C, R>(mm: &ModelManager, content: C) -> Result<R>
where
    MC: SurrealBmc,
    C: Serialize,
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let id = ulid::Ulid::new().to_string();

    let created: Option<R> = srdb.create((MC::TABLE, id)).content(content).await?;

    if let Some(created) = created {
        return Ok(created);
    } else {
        return Err(Error::FailedToCreate);
    }
}

/// Same as `base_create`, but uses the given string as the id.
pub async fn base_create_with_id<MC, C, R>(mm: &ModelManager, content: C, id: &str) -> Result<R>
where
    MC: SurrealBmc,
    C: Serialize,
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let q = r"CREATE $thing CONTENT $content;";

    let identifier = id;

    let thing: Thing = (MC::TABLE, identifier).into();

    let mut response = srdb
        .query(q)
        .bind(("thing", thing))
        .bind(("content", &content))
        .await?;

    let created: Option<R> = response.take(0)?;

    if let Some(created) = created {
        return Ok(created);
    } else {
        return Err(Error::FailedToCreate);
    }
}

/// Get a single record based on the given id
pub async fn base_get<MC, R>(mm: &ModelManager, id: &str) -> Result<R>
where
    MC: SurrealBmc,
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let selected: Option<R> = srdb.select((MC::TABLE, id)).await?;

    if let Some(selected) = selected {
        return Ok(selected);
    } else {
        return Err(Error::FailedToGet(id.into()));
    }
}

/// Update a single record based on the given id with the given data
pub async fn base_update<MC, C, R>(mm: &ModelManager, data: C, id: &str) -> Result<R>
where
    MC: SurrealBmc,
    C: Serialize,
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let updated: Option<R> = srdb.update((MC::TABLE, id)).content(data).await?;

    if let Some(updated) = updated {
        return Ok(updated);
    } else {
        return Err(Error::FailedToUpdate(id.into()));
    }
}

/// List all records
pub async fn base_list<MC, R>(mm: &ModelManager) -> Result<Vec<R>>
where
    MC: SurrealBmc,
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let labels: Vec<R> = srdb.select(MC::TABLE).await?;

    Ok(labels)
}

/// Delete a single record based on the given id
pub async fn base_delete<MC, R>(mm: &ModelManager, id: &str) -> Result<R>
where
    MC: SurrealBmc,
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let deleted: Option<R> = srdb.delete((MC::TABLE, id)).await?;

    if let Some(deleted) = deleted {
        return Ok(deleted);
    } else {
        return Err(Error::FailedToDeleteIdNotFound(id.into()));
    }
}

pub async fn base_info<MC>(mm: &ModelManager) -> Result<Value>
where
    MC: SurrealBmc,
{
    let srdb = mm.srdb().clone();
    srdb.use_ns("test").use_db("test").await?;

    let q = r"
    INFO FOR TABLE type::table($tablename);
    ";

    let q = format!(
        "
    INFO FOR TABLE {};
    ",
        MC::TABLE
    );

    println!("TABLEINFO: {}", MC::TABLE);

    let mut res = srdb.query(q).await?;

    let val: Option<Value> = res.take(0)?;

    if let Some(val) = val {
        let converted = serde_json::from_value(val)?;
        return Ok(converted);
    } else {
        return Err(Error::FailedToCreate);
    }
}
