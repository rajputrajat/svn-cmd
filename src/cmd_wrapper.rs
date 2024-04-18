//! this module will implement all svn cmd wrapper

use crate::errors::SvnError;
use log::trace;
use std::{os::windows::process::CommandExt, process::Command};

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
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    pub(crate) fn common_cmd_runner(&self, args: &[&str]) -> Result<String, SvnError> {
        trace!("command args: {:?}", args);
        match Command::new(&self.cmd)
            .args(args)
            .creation_flags(Self::CREATE_NO_WINDOW)
            .output()
        {
            Ok(o) => {
                if o.stderr.is_empty() {
                    trace!("svn cmd output: '{:?}'", o.stdout);
                    String::from_utf8(o.stdout).map_err(|e| {
                        trace!("invalid utf8 output of svn cmd '{:?} {args:?}'", self.cmd,);
                        SvnError::FromUtf8Error { src: e }
                    })
                } else {
                    Err(SvnError::Other(format!(
                        "some error while running svn command: {:?}",
                        match String::from_utf8(o.stderr) {
                            Ok(o) => o,
                            Err(e) => e.to_string(),
                        }
                    )))
                }
            }
            Err(e) => Err(SvnError::MissingSvnCli { src: e }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn simple_run() {
        let wrap = SvnWrapper::new();
        let out = dbg!(wrap.common_cmd_runner(&["info"]));
        assert_eq!(out.unwrap(), " ".to_owned());
    }
}
