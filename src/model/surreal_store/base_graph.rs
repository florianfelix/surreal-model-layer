use serde::de::DeserializeOwned;
use serde_json::Value;
use surrealdb::sql::Thing;

use crate::model::{
    surreal_store::{Error, Result},
    ModelManager,
};

use super::SurrealBmc;

/// Connect two records
pub async fn base_connect<MC, R>(mm: &ModelManager, parent: Thing, child: Thing) -> Result<R>
where
    MC: SurrealBmc,
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    // parent and child are Thing
    // parent = "user:BobTheBuilder"
    // child = "transaction:bzwmxto9yj8ffz4yega7"
    // MC::TABLE is a table name as &str

    // let parent = parent.to_raw();
    // let child = child.to_raw();

    // DOES NOT WORK: expected a table name
    let q = "RELATE $parent->$connection->$child;";

    // DOES NOT WORK: Failed to parse but type::table seems to work?
    let q = "RELATE $parent->type::table($connection)->$child;";

    // DOES NOT WORK
    let q = "RELATE type::thing($parent)->type::table($connection)->type::thing($child);";

    // DOES NOT WORK
    let q = "RELATE type::record($parent)->type::table($connection)->type::record($child);";

    // WORKS
    let q = format!("RELATE {}->{}->{}", parent, MC::TABLE, child); // This works with strings and <Thing>s

    let mut response = srdb
        .query(q)
        .bind(("parent", parent))
        .bind(("connection", MC::TABLE))
        .bind(("child", child))
        .await?;

    let created: Option<R> = response.take(0)?;

    if let Some(created) = created {
        return Ok(created);
    } else {
        return Err(Error::FailedToCreateGraphConnection);
    }

    // Ok(())
}

/// List all objects incoming from this connection
pub async fn base_list_incoming<MC, R>(mm: &ModelManager) -> Result<Vec<R>>
where
    MC: SurrealBmc,
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let q = "SELECT in FROM type::table($table);";

    let mut response = srdb.query(q).bind(("table", MC::TABLE)).await?;

    let incoming: Vec<R> = response.take(0)?;

    Ok(incoming)
}

/// List all objects outgoing from this connection
pub async fn base_list_outgoing<MC, R>(mm: &ModelManager) -> Result<Vec<R>>
where
    MC: SurrealBmc,
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let q = "SELECT out FROM type::table($table);";

    let mut response = srdb.query(q).bind(("table", MC::TABLE)).await?;

    let outgoing: Vec<R> = response.take(0)?;

    Ok(outgoing)
}

/// List all connections
pub async fn base_list_connections<MC, R>(mm: &ModelManager) -> Result<Vec<R>>
where
    MC: SurrealBmc,
    R: DeserializeOwned,
    // R: DeserializeOwned + std::fmt::Debug,
{
    let srdb = mm.srdb().clone();
    let res: Vec<R> = srdb.select(MC::TABLE).await?;
    // dbg!(&res);

    Ok(res)
}

/// List connected to record
pub async fn base_list_connected_ids_from_id<MC, R>(mm: &ModelManager, id: Thing) -> Result<Vec<R>>
where
    MC: SurrealBmc,
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let q = format!("SELECT  ->{}.out FROM $id", MC::TABLE);

    let mut response = srdb
        .query(q)
        // .bind(("table", MC::TABLE))
        .bind(("id", id))
        .await?;

    let connected: Vec<R> = response.take(0)?;

    Ok(connected)
}

/// Delete a connection
pub async fn base_delete<MC, R>(mm: &ModelManager, id: Thing) -> Result<R>
where
    MC: SurrealBmc,
    R: DeserializeOwned,
{
    let srdb = mm.srdb().clone();

    let deleted: Option<R> = srdb.delete((MC::TABLE, id.to_raw())).await?;

    if let Some(deleted) = deleted {
        return Ok(deleted);
    } else {
        return Err(Error::FailedToDeleteIdNotFound(id.to_raw()));
    }
}
