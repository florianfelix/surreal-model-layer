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

    let created: Vec<Value> = srdb.create(MC::TABLE).content(content).await?;

    if created.len() > 0 {
        let converted = serde_json::from_value(created[0].clone())?;
        return Ok(converted);
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

    let created: Option<Value> = srdb.create((MC::TABLE, &id)).content(content).await?;

    if let Some(created) = created {
        let converted = serde_json::from_value(created)?;
        return Ok(converted);
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

    // let identifier: Id = vec![ulid::Ulid::new().to_string(), id.to_string()].into();
    let identifier = id;

    let thing: Thing = (MC::TABLE, identifier).into();

    let mut response = srdb
        .query(q)
        .bind(("thing", thing))
        .bind(("content", &content))
        .await?;

    let created: Option<Value> = response.take(0)?;

    // let created: Option<Value> = srdb.create((MC::TABLE, id)).content(content).await?;

    if let Some(created) = created {
        let converted = serde_json::from_value(created)?;
        return Ok(converted);
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

    let selected: Option<Value> = srdb.select((MC::TABLE, id)).await?;

    if let Some(selected) = selected {
        let converted = serde_json::from_value(selected)?;
        return Ok(converted);
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

    let updated: Option<Value> = srdb.update((MC::TABLE, id)).content(data).await?;

    if let Some(updated) = updated {
        let converted = serde_json::from_value(updated)?;
        return Ok(converted);
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

    let deleted: Option<Value> = srdb.delete((MC::TABLE, id)).await?;

    if let Some(deleted) = deleted {
        let converted = serde_json::from_value(deleted)?;
        return Ok(converted);
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
