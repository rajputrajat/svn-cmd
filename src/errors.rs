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
    #[error("command line svn tool isn't installed or not added in PATH env: {src:?}")]
    MissingSvnCli { src: io::Error },

    /// invalid UTF8 output
    #[error("invalid utf8 output: {src:?}")]
    FromUtf8Error { src: FromUtf8Error },

    /// invalid UTF8 output
    #[error("error while deserializing: {src:?}")]
    Deserializer { src: serde_xml_rs::Error },

    /// requested path doesn't exist
    #[error("requested path doesn't exist")]
    InvalidPath,

    /// invalid credentials
    #[error("invalid credentials supplied")]
    InvalidCredentials,

    /// not working copy
    #[error("current dir is not working dir")]
    NotWorkingDir,

    /// other error
    #[error("other error: `{0}`")]
    Other(String),
}
