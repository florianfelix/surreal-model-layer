/// Isomorphic(?) functions the Model Controllers can call
pub mod base_crud;
pub mod base_graph;
pub mod general_crud;
pub mod error;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub use self::error::{Error, Result};

/// Model Bmc Structs need to implement this trait.
///
/// And set the TABLE name.
///
/// The base... functions will use the TABLE name.
pub trait SurrealBmc {
    const TABLE: &'static str;
}

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
