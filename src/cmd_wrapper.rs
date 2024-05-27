//! this module will implement all svn cmd wrapper

use crate::errors::SvnError;
use log::trace;
use managed_command::{Canceller, Command as ManagedCommand};
use std::{
    os::windows::process::CommandExt,
    process::Command,
    sync::{Arc, Mutex},
    thread,
};

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

    pub(crate) fn common_cmd_runner(
        &self,
        args: &[&str],
        canceller: Canceller,
    ) -> Result<String, SvnError> {
        trace!("command args: {:?}", args);
        let mut cmd = Command::new(&self.cmd);
        cmd.args(args);
        cmd.creation_flags(Self::CREATE_NO_WINDOW);
        let mut cmd: ManagedCommand = cmd.into();
        let (_stdin, stdout, stderr) = cmd.run(canceller)?;
        let stderr_text = Arc::new(Mutex::new(String::new()));
        let stdout_text = Arc::new(Mutex::new(String::new()));
        thread::spawn(move || {
            while let Ok(stderr_str) = stderr.recv() {
                (*stderr_text.lock().unwrap()).push_str(stderr_str.as_str());
            }
        });
        thread::spawn(move || {
            while let Ok(stdout_str) = stdout.recv() {
                (*stdout_text.lock().unwrap()).push_str(stdout_str.as_str());
            }
        });
        // {
        //     Ok(o) => {
        //         if o.stderr.is_empty() {
        //             String::from_utf8(o.stdout).map_err(|e| {
        //                 trace!("invalid utf8 output of svn cmd '{:?} {args:?}'", self.cmd,);
        //                 SvnError::FromUtf8Error { src: e }
        //             })
        //         } else {
        //             Err(SvnError::Other(format!(
        //                 "some error while running svn command: {:?}",
        //                 match String::from_utf8(o.stderr) {
        //                     Ok(o) => o,
        //                     Err(e) => e.to_string(),
        //                 }
        //             )))
        //         }
        //     }
        //     Err(e) => Err(SvnError::MissingSvnCli { src: e }),
        // }
        Ok("".to_owned())
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
