//! this module will implement all svn cmd wrapper

use crate::errors::SvnError;
use std::process::Command;

/// cmd wrapper struct
pub(crate) struct SvnWrapper {
    cmd: String,
}

// associated functions
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

// private methods
impl SvnWrapper {
    async fn common_cmd_runner(&self, args: &[&str]) -> Result<String, SvnError> {
        match Command::new(&self.cmd).args(args).output() {
            Ok(o) => {
                if o.stderr.is_empty() {
                    String::from_utf8(o.stdout).map_err(|e| SvnError::FromUtf8Error { src: e })
                } else {
                    Err(SvnError::Other(match String::from_utf8(o.stderr) {
                        Ok(o) => o,
                        Err(e) => e.to_string(),
                    }))
                }
            }
            Err(e) => Err(SvnError::MissingSvnCli { src: e }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    #[should_panic]
    async fn simple_run() {
        let wrap = SvnWrapper::new();
        let out = dbg!(wrap.common_cmd_runner(&["info"]).await);
        assert_eq!(out.unwrap(), " ".to_owned());
    }
}
