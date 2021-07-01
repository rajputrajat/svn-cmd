//! this implements svn types
use std::io;
use thiserror::Error;

/// Credentials
pub struct Credentials {
    username: String,
    password: String,
}

/// global options to use svn tool
pub struct LoginOptions {
    credentials: Credentials,
    cache_auth_tokens: bool,
    non_interactive: bool,
    trust_server_cert: bool, // this is valid only when non_interactive is `true`
    config_options: Option<String>,
}

/// file or dir
pub enum PathType {
    File,
    Dir,
}

/// revision
#[derive(Debug, PartialEq, Eq)]
pub enum RevisionType {
    Head,
    Revision(u64),
}

/// Return value of SvnCmd . info()
pub struct SvnInfo {}

/// Return value of SvnCmd . status()
pub struct SvnStatus {}

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
