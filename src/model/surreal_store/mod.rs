/// Isomorphic(?) functions the Model Controllers can call
pub mod base_crud;
pub mod error;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub use self::error::{Error, Result};

pub type SrDb = Surreal<Client>;

/// returns a new surreal client connection logged in as root:root
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
