//! this module will implement all svn cmd wrapper

use crate::errors::SvnError;
use std::process::Command;

/// cmd wrapper struct
pub(crate) struct SvnWrapper {
    cmd: String,
}

impl SvnWrapper {
    fn common_cmd_runner(&self, args: &[&str]) -> Result<String, SvnError> {
        match Command::new(&self.cmd).args(args).output() {
            Ok(o) => String::from_utf8(o.stdout).map_err(|e| SvnError::FromUtf8Error { src: e }),
            Err(e) => Err(SvnError::MissingSvnCli { src: e }),
        }
    }
}

impl SvnWrapper {
    pub(crate) fn new() -> Self {
        Self {
            cmd: "svn".to_owned(),
        }
    }

    pub(crate) fn from_cmd<T: Into<String>>(cmd: T) -> Self {
        Self { cmd: cmd.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
