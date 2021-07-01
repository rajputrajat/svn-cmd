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
    #[error("command line svn tool isn't installed or not added in PATH env")]
    MissingSvnCli(#[from] io::Error),
    /// requested path doesn't exist
    #[error("requested path doesn't exist")]
    InvalidPath,
    /// invalid credentials
    #[error("invalid credentials supplied")]
    InvalidCredentials,
}
