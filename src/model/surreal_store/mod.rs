pub mod error;
pub mod base_crud;

use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}, opt::auth::Root};

pub use self::error::{Error, Result};

pub type SrDb = Surreal<Client>;

// use super::SrDb;

pub async fn new_surreal_connection() -> Result<SrDb> {
    let srdb = Surreal::new::<Ws>("127.0.0.1:8000")
            .await
            .map_err(|_| Error::FailedToConnectToDatabase)?;

        srdb.signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .map_err(|_| Error::FailedToSignInAsRoot)?;

    Ok(srdb)
}