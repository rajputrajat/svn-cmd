//! this module will implement all svn cmd wrapper

use crate::errors::SvnError;
use log::trace;
use managed_command::Command as ManagedCommand;
use rr_common_utils::{Future, JobDesc, ThreadPool};
use simple_broadcaster::{Canceller, CloneAs};
use std::{os::windows::process::CommandExt, process::Command, sync::Arc};

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

    // pub(crate) fn from_cmd<T: Into<String>>(cmd: T) -> Self {
    //     Self { cmd: cmd.into() }
    // }
}

/// Runner context
#[derive(Debug, Clone)]
pub struct RunnerContext {
    /// ThreadPool to use thread from
    pub pool: Arc<ThreadPool>,
    /// command run canceller
    pub canceller: Canceller,
}

/// This wraps in rr_common_utils::Future<String> for stdout
#[derive(Debug)]
pub struct StdoutFuture(pub Future<String>);

/// This wraps in rr_common_utils::Future<String> for stderr
#[derive(Debug)]
pub struct StderrFuture(pub Future<String>);

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
                    String::from_utf8(o.stdout).map_err(|e| {
                        trace!("invalid utf8 output of svn cmd '{:?} {args:?}'", self.cmd,);
                        SvnError::FromUtf8Error(e)
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
            Err(e) => Err(SvnError::MissingSvnCli(e)),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn common_cmd_runner_cancellable(
        &self,
        args: &[&str],
        runner_context: &RunnerContext,
    ) -> Result<(StdoutFuture, StderrFuture), SvnError> {
        trace!("command args: {:?}", args);
        let mut cmd = Command::new(&self.cmd);
        cmd.args(args);
        cmd.creation_flags(Self::CREATE_NO_WINDOW);
        let mut cmd: ManagedCommand = cmd.into();
        let (_stdin, stdout, stderr) =
            cmd.run(runner_context.canceller.clone_as(format!("run: {args:?}")))?;
        let stderr_future = StderrFuture(runner_context.pool.run_async(
            move || {
                let mut out = String::new();
                while let Ok(stderr_str) = stderr.recv() {
                    out.push_str(stderr_str.as_str());
                }
                out
            },
            JobDesc::create(
                "common_cmd_runner_cancellable",
                "capturing the stderr of svn cmd '{args}'",
            ),
        ));
        let stdout_future = StdoutFuture(runner_context.pool.run_async(
            move || {
                let mut out = String::new();
                while let Ok(stdout_str) = stdout.recv() {
                    out.push_str(stdout_str.as_str());
                }
                out
            },
            JobDesc::create(
                "common_cmd_runner_cancellable",
                "capturing the stdout of svn cmd '{args}'",
            ),
        ));
        Ok((stdout_future, stderr_future))
    }
}
