use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use uuid;
use uuid::Uuid;

use crate::model::ModelManager;

#[allow(unused_imports)]
use crate::model::{Error, Result};

#[allow(unused_imports)]
use super::surreal_store::base_crud::{
    base_create_with_id, base_delete, base_get, base_list, base_update,
};
use super::surreal_store::SurrealBmc;

// region: Types
#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: Thing,
    pub user_name: String,
    pub pwd: Option<String>,
    #[serde(with = "uuid::serde::compact")]
    pub salt: Uuid,
    #[serde(with = "uuid::serde::compact")]
    pub salt_token: Uuid,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserForCreate {
    pub user_name: String,
    #[serde(with = "uuid::serde::compact")]
    pub salt: Uuid,
    #[serde(with = "uuid::serde::compact")]
    pub salt_token: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserForLogin {
    pub id: Thing,

    // -- pwd and token info
    pub pwd: Option<String>, // encrypted, #_scheme_id_#....
    #[serde(with = "uuid::serde::compact")]
    pub salt: Uuid,
    #[serde(with = "uuid::serde::compact")]
    pub salt_token: Uuid,
}

// endregion: Types

pub struct UserBmc;

impl SurrealBmc for UserBmc {
    const TABLE: &'static str = "user";
}

// region: UserBmc
impl UserBmc {
    // CREATE
    pub async fn create(mm: &ModelManager, new_user_name: &str) -> Result<User> {
        let salt = Uuid::new_v4();
        let salt_token = Uuid::new_v4();

        let new_user = UserForCreate {
            user_name: new_user_name.into(),
            salt,
            salt_token,
        };

        let res = base_create_with_id::<Self, _, _>(mm, new_user, &new_user_name).await?;

        Ok(res)
    }

    // GET BY ID
    pub async fn get(mm: &ModelManager, id: String) -> Result<User> {
        let res = base_get::<Self, _>(mm, &id).await?;

        Ok(res)
    }

    // LIST ALL
    pub async fn list(mm: &ModelManager) -> Result<Vec<UserForLogin>> {
        let res = base_list::<Self, _>(mm).await?;

        Ok(res)
    }

    // UPDATE BY ID
}

// endregion: UserBmc
