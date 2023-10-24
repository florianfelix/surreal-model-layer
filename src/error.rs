use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	// -- Modules
    Generic,
    
	Model(model::Error),

    Surreal(surrealdb::Error),
}

// region:    --- Froms
impl From<model::Error> for Error {
	fn from(val: model::Error) -> Self {
		Self::Model(val)
	}
}

// only needed to delete the tables in the main function
// ideally this could be removed
// only the model layer should care about the store errors
impl From<surrealdb::Error> for Error {
	fn from(val: surrealdb::Error) -> Self {
		Self::Surreal(val)
	}
}
// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate