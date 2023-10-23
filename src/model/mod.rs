// region: Imports
use self::surreal_store::{new_surreal_connection, SrDb};

#[allow(unused_imports)]
pub use self::error::{Error, Result};

pub mod error;
/// base functions for the specific BMCs
pub mod surreal_store;
/// specific data model. uses the base functions.
pub mod label;
/// specific data model. uses the base functions.
pub mod transaction;
/// specific data model. uses the base functions.
pub mod users;
/// table to test the roundtrip of varying datatypes
pub mod datatypes;

// endregion: Imports


// region: Model Manager
/// Holds the Surreal Client
#[derive(Clone)]
pub struct ModelManager {
    /// Surreal Client
    srdb: SrDb,
}

impl ModelManager {
    /// Init ModelManager with default Surreal Connection
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

// endregion: Model Manager