//! this implements svn types

use std::path::PathBuf;
use url::Url;

pub(crate) trait ToCmdArgs {
    fn to_cmd_args(&self) -> String;
}

/// Credentials
#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl ToCmdArgs for Credentials {
    fn to_cmd_args(&self) -> String {
        format!(
            " --username {} --password {} ",
            self.username, self.password
        )
    }
}

/// optional values
#[derive(Debug, Clone)]
pub struct Optionals {
    pub cache_auth_tokens: bool,
    pub non_interactive: bool,
    pub trust_server_cert: bool, // this is valid only when non_interactive is `true`
    pub config_options: Option<String>,
}

impl ToCmdArgs for Optionals {
    fn to_cmd_args(&self) -> String {
        let mut args = String::new();
        if self.cache_auth_tokens {
            args.push_str(" --no-auth-cache ");
        }
        if self.non_interactive {
            args.push_str(" --non-interactive ");
        }
        if self.trust_server_cert {
            args.push_str(" --trust-server-cert ");
        }
        if let Some(config_ops) = &self.config_options {
            args.push(' ');
            args.push_str(config_ops);
            args.push(' ');
        }
        args
    }
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
#[derive(Debug, Clone)]
pub struct LoginOptions {
    pub credentials: Option<Credentials>,
    pub more: Optionals,
}

impl ToCmdArgs for LoginOptions {
    fn to_cmd_args(&self) -> String {
        format!(
            " {} {} ",
            self.credentials
                .as_ref()
                .map_or_else(|| "".to_owned(), |v| v.to_cmd_args()),
            self.more.to_cmd_args()
        )
    }
}

/// file or dir
#[derive(Debug, PartialEq, PartialOrd, Clone)]
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

pub enum Target {
    Local(PathBuf),
    Remote(Url),
}
