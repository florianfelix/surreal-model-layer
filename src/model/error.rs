use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::surreal_store;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    FailedToConnectToSurreal,

    // FailedTryFrom,

    // FailedToCreate,
    // FailedToList,
    // FailedToGet(String),
    // FailedToUpdate(String),
    // FailedToDelete(String),
    SerializeError,

    SurrealError(String),
    Store(surreal_store::Error),
}

// region:    --- Froms
impl From<surrealdb::Error> for Error {
    fn from(val: surrealdb::Error) -> Self {
        Self::SurrealError(val.to_string())
    }
}

impl From<surreal_store::Error> for Error {
    fn from(val: surreal_store::Error) -> Self {
        Self::Store(val)
    }
}

// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
