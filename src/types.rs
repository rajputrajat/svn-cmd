//! this implements svn types

use async_std::path::PathBuf;
use serde::Deserialize;
use url::Url;

/// Credentials
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// optional values
pub struct Optionals {
    pub cache_auth_tokens: bool,
    pub non_interactive: bool,
    pub trust_server_cert: bool, // this is valid only when non_interactive is `true`
    pub config_options: Option<String>,
}

impl Default for Optionals {
    fn default() -> Self {
        Self {
            cache_auth_tokens: true,
            non_interactive: true,
            trust_server_cert: true,
            config_options: None,
        }
    }
}

/// global options to use svn tool
pub struct LoginOptions {
    pub credentials: Credentials,
    pub more: Optionals,
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
#[derive(Debug, Deserialize)]
pub struct SvnInfo {
    //entry:
}

/// Return value of SvnCmd . status()
pub struct SvnStatus {}

pub enum Target {
    Local(PathBuf),
    Remote(Url),
}
