//! this module will implement all svn cmd wrapper

use crate::errors::SvnError;
use std::process::Command;

/// cmd wrapper struct
pub(crate) struct SvnWrapper;

impl SvnWrapper {
    pub(crate) fn svn_wrapper(args: &[&str]) -> Result<String, SvnError> {
        match Command::new("svn").args(args).output() {
            Ok(o) => String::from_utf8(o.stdout).map_err(|e| SvnError::FromUtf8Error { src: e }),
            Err(e) => Err(SvnError::MissingSvnCli { src: e }),
        }
    }
}
