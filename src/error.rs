use std::io;
use axum::response::IntoResponse;
use failure::Fail;

/// Error type for kvs
#[derive(Fail, Debug)]
pub enum KvsError {
    /// IO error
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    /// Serialization or deserialization error
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
    /// Remove no-existent key error
    #[fail(display = "Key not found")]
    KeyNotFound,

    /// Unexpected command type error.
    /// It indicated a corrupted log or a program bug.
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,

}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> Self {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> Self {
        KvsError::Serde(err)
    }
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;

impl IntoResponse for KvsError {
    fn into_response(self) -> axum::response::Response {
        // match self {
        //     KvsError::Io(_) => todo!(),
        //     KvsError::Serde(_) => todo!(),
        //     KvsError::KeyNotFound => todo!(),
        //     KvsError::UnexpectedCommandType => todo!(),
        // }
        "Error".into_response()
    }
}