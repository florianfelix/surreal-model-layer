use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    FailedToConnectToDatabase,
    FailedToSignInAsRoot,

    FailedToCreate,
    FailedToGet(String),
    FailedToUpdate(String),
    FailedToDeleteIdNotFound(String),

    FailedToCreateGraphConnection,
    FailedToListGraphConnection,

    SurrealError(String),

    DeserializeError(String),
    // Model(model::error::Error),
}

// region:    --- Froms
impl From<surrealdb::Error> for Error {
    fn from(val: surrealdb::Error) -> Self {
        Self::SurrealError(val.to_string())
    }
}
impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Self::DeserializeError(val.to_string())
    }
}
// impl From<model::error::Error> for Error {
//     fn from(val: model::error::Error) -> Self {
//         Self::Model(val)
//     }
// }

// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
