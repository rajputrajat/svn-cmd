//! Errors are defined here

use std::io;
use thiserror::Error;

/// lib specific error type
#[derive(Error, Debug)]
pub enum SvnError {
    /// no connection
    #[error("no connectivity")]
    Disconnection,

    /// Svn utility isn't installed
    #[error(transparent)]
    MissingSvnCli(#[from] io::Error),

    /// invalid UTF8 output
    #[error("utf8 error. lossy string is passed in")]
    FromUtf8Error(String),

    /// invalid UTF8 output
    #[error(transparent)]
    Deserializer(#[from] serde_xml_rs::Error),

    /// requested path doesn't exist
    #[error("requested path doesn't exist")]
    InvalidPath,

    /// invalid credentials
    #[error("invalid credentials supplied")]
    InvalidCredentials,

    /// not working copy
    #[error("current dir is not working dir")]
    NotWorkingDir,

    /// errors from the crate 'managed-command'
    #[error(transparent)]
    ManagedCommandError(#[from] managed_command::Error),

    /// errors from the crate 'oneshot'
    #[error(transparent)]
    OneshotRecvError(#[from] oneshot::RecvError),

    /// other error
    #[error("other error: `{0}`")]
    Other(String),
}
