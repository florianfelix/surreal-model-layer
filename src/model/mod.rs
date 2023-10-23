use self::surreal_store::{new_surreal_connection, SrDb};

#[allow(unused_imports)]
pub use self::error::{Error, Result};

pub mod error;
pub mod label;
pub mod surreal_store;
pub mod transaction;
pub mod users;
pub mod datatypes;

#[derive(Clone)]
pub struct ModelManager {
    srdb: SrDb,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let srdb = new_surreal_connection()
            .await
            .map_err(|_| Error::FailedToConnectToSurreal)?;

        srdb.use_ns("test").use_db("test").await?;

        Ok(Self { srdb })
    }

    pub fn srdb(&self) -> &SrDb {
        &self.srdb
    }
}
