use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use uuid::Uuid;
use uuid;

use crate::model::ModelManager;

#[allow(unused_imports)]
use crate::model::{Error, Result};

use super::surreal_store::base_crud::{
    base_create_with_id, base_delete, base_get, base_list, base_update, SurrealBmc,
};

// region: Types
#[derive(Debug, Clone, Deserialize)]
pub struct Label {
    pub id: Thing,
    pub label_name: String
}

#[derive(Debug, Clone, Serialize)]
pub struct LabelForCreate {
    pub label_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LabelForUpdate {
    pub label_name: String,
}

// endregion: Types

// region: LabelBmc
pub struct LabelBmc;

impl SurrealBmc for LabelBmc {
    const TABLE: &'static str = "labels";
}

impl LabelBmc {
    // CREATE
    pub async fn create(mm: &ModelManager, lbl_create: LabelForCreate) -> Result<Label> {
        let res = base_create_with_id::<Self, _, _>(mm, &lbl_create, &lbl_create.label_name).await?;

        Ok(res)
    }

    // CREATE WITH ID
    pub async fn create_with_id(
        mm: &ModelManager,
        ta_create: LabelForCreate,
        id: String,
    ) -> Result<Label> {
        let res = base_create_with_id::<Self, _, _>(mm, ta_create, &id).await?;

        Ok(res)
    }

    // GET
    pub async fn get(mm: &ModelManager, id: String) -> Result<Label> {
        let res = base_get::<Self, _>(mm, &id).await?;

        Ok(res)
    }

    // LIST ALL
    pub async fn list(mm: &ModelManager) -> Result<Vec<Label>> {
        let res = base_list::<Self, _>(mm).await?;

        Ok(res)
    }

    // UPDATE
    pub async fn update(mm: &ModelManager, id: String, data: LabelForUpdate) -> Result<Label> {
        let res = base_update::<Self, _, _>(mm, data, &id).await?;

        Ok(res)
    }

    // DELETE
    pub async fn delete(mm: &ModelManager, id: String) -> Result<Label> {
        let res = base_delete::<Self, _>(mm, &id).await?;

        Ok(res)
    }
}

// endregion: LabelBmc
