use serde_json::Value;
use surrealdb::sql::Thing;

use crate::model::{
    surreal_store::{Error, Result},
    ModelManager,
};

use super::SurrealBmc;

/// Connect two records
pub async fn base_connect<MC>(mm: &ModelManager, parent: Thing, child: Thing) -> Result<Value>
where
    MC: SurrealBmc,
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

    let created: Option<Value> = response.take(0)?;
    dbg!(&created);

    if let Some(val) = created {
        let converted = serde_json::from_value(val)?;
        return Ok(converted);
    } else {
        return Err(Error::FailedToCreateGraphConnection);
    }

    // Ok(())
}

/// List all connections
pub async fn base_list_connections<MC>(mm: &ModelManager) -> Result<Vec<Value>>
where
    MC: SurrealBmc,
{
    let srdb = mm.srdb().clone();
    let res: Vec<Value> = srdb.select(MC::TABLE).await?;
    // dbg!(&res);

    Ok(res)
}
