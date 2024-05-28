//! Errors are defined here

use std::{io, string::FromUtf8Error};
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
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),

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

    /// other error
    #[error("other error: `{0}`")]
    Other(String),
}
