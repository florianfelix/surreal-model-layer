use serde_json::Value;
use surrealdb::sql::Thing;

use super::{
    surreal_store::{base_graph::{base_connect, base_list_connections}, SurrealBmc},
    ModelManager,
};

#[allow(unused_imports)]
use crate::model::{Error, Result};

// region: EdgeBmc
pub struct EdgeBmc;

impl SurrealBmc for EdgeBmc {
    const TABLE: &'static str = "edge";
}

impl EdgeBmc {
    /// Connect two records
    pub async fn connect(mm: &ModelManager, parent: Thing, child: Thing) -> Result<Value> {
        let res = base_connect::<Self>(mm, parent, child).await?;

        Ok(res)
    }

    /// List all connections
    pub async fn list(mm: &ModelManager) -> Result<Vec<Value>> {
        let res = base_list_connections::<Self>(mm).await?;
        Ok(res)
    }
}
