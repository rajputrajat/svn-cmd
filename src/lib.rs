//! This lib wraps svn command line tool on your system
#![warn(missing_docs)]
#![warn(unsafe_code)]

mod cmd_wrapper;
mod errors;
mod sub_commands;
mod types;

pub use crate::{
    cmd_wrapper::{StderrFuture, StdoutFuture},
    errors::SvnError,
    sub_commands::{
        info::{EntryCommit, SvnInfo},
        list::{ListEntry, SvnList},
        log::SvnLog,
        status::SvnStatus,
        version::CmdVersion,
    },
    types::{Credentials, LoginOptions, Optionals, PathType},
};

use crate::{
    cmd_wrapper::SvnWrapper,
    sub_commands::log::{RevCount, StartRev, XmlOut},
    types::ToCmdArgs,
};
use cmd_wrapper::RunnerContext;
use log::trace;
use rr_common_utils::Future;
use std::{result::Result, sync::Arc};

/// Accessor to svn command functionality
#[derive(Debug, Clone)]
pub struct SvnCmd {
    #[allow(dead_code)]
    options: LoginOptions,
    extra_args: String,
}

/// Builder to create SvnCmd
pub struct SvnCmdBuilder {}

impl SvnCmd {
    /// create SvnCmd struct
    pub fn new(creds: Option<Credentials>, more: Option<Optionals>) -> SvnCmd {
        let options = LoginOptions {
            credentials: creds,
            more: more.unwrap_or_default(),
        };
        let extra_args = options.to_cmd_args();
        SvnCmd {
            options,
            extra_args,
        }
    }

    /// get svn version installed
    pub fn version() -> Result<CmdVersion, SvnError> {
        let out = SvnWrapper::new().common_cmd_runner(&["--version"])?;
        CmdVersion::parse(&out)
    }

    /// get list of files
    pub fn list(&self, target: &str, recursive: bool) -> Result<SvnList, SvnError> {
        let mut args = vec!["list", "--xml", target];
        if recursive {
            args.push("--recursive");
        }
        let xml_text = self.get_cmd_out(&args)?;
        trace!("{}", xml_text);
        SvnList::parse(&xml_text)
    }

    /// get list of files
    pub fn list_cancellable(
        &self,
        target: &str,
        recursive: bool,
        runner_context: &RunnerContext,
    ) -> Result<(Future<Result<SvnList, SvnError>>, StderrFuture), SvnError> {
        let mut args = vec!["list", "--xml", target];
        if recursive {
            args.push("--recursive");
        }
        let (xml_text_future, err_text_future) =
            self.get_cmd_out_cancellable(&args, runner_context)?;
        Ok((
            xml_text_future
                .0
                .try_map(|xml_text| SvnList::parse(&xml_text)),
            err_text_future,
        ))
    }

    /// get list of files
    pub fn list_from_svn_list_xml_output(&self, xml_str: &str) -> Result<SvnList, SvnError> {
        SvnList::parse(xml_str)
    }

    /// get diff
    pub fn diff() -> Result<(), SvnError> {
        Ok(())
    }

    /// read file content
    pub fn cat(&self, target: &str) -> Result<String, SvnError> {
        self.get_cmd_out(&["cat", target])
    }

    /// read file content
    pub fn cat_cancellable(
        &self,
        target: &str,
        runner_context: &RunnerContext,
    ) -> Result<(StdoutFuture, StderrFuture), SvnError> {
        self.get_cmd_out_cancellable(&["cat", target], runner_context)
    }

    /// SVN ADD command to add new files to stage for commit operation
    /// `svn add PATH`
    pub fn add(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN COMMIT command to commit changes to remote repo
    /// `svn commit -m "dummy log message"`
    pub fn commit_local_changes(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN CHECKOUT command: checkout files from mentioned repo url
    /// `svn checkout REPO_URL LOCAL_PATH`
    pub fn checkout(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN UPDATE command: update local svn dir with remote repo
    /// `svn update`
    pub fn update(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN LOG command: read svn logs
    /// `svn log REPO_URL | LOCAL_PATH`
    pub fn log(&self, target: &str) -> Result<SvnLog, SvnError> {
        let mut args = vec!["log", "--xml"];
        args.push(&self.extra_args);
        SvnLog::new(&args, target, Arc::new(SvnCmd::log_fetcher))
    }

    /// SVN STATUS command: svn path status
    /// `svn status PATH`
    pub fn status(&self, target: &str) -> Result<SvnStatus, SvnError> {
        let out = self.get_cmd_out(&["status", "--xml", target])?;
        SvnStatus::parse(out)
    }

    /// SVN INFO command: read svn info
    /// `svn info PATH`
    pub fn info(&self, target: &str) -> Result<SvnInfo, SvnError> {
        let out = self.get_cmd_out(&["info", "--xml", target])?;
        SvnInfo::parse(&out)
    }

    /// SVN DELETE command: delete file/dir from remote url
    /// `svn delete PATH`
    pub fn delete(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN REVERT command: revert to specific commit
    /// `svn revert PATH`
    pub fn revert(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN COPY command: copy from svn repo path to mentioned path
    /// `svn copy CURR_PATH NEW_PATH`
    pub fn copy_to(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN SWITCH command: switch current working svn path to requested path
    /// `svn switch CURR_PATH NEW_PATH`
    pub fn switch(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN MERGE command: merge change-sets from mentioned repo
    /// `svn merge [--dry-run] --force From_URL@revN To_URL@revM PATH`
    pub fn merge(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN IMPORT command: import dir/files from local filesystem
    /// `svn import -m "<commit message>"`
    pub fn import(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN MKDIR command: create a dir in svn repo
    /// `svn mkdir -m "<commit message>"`
    pub fn mkdir(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN <raw> command: run a raw command
    /// `svn <raw_cmd>
    pub fn raw_cmd(&self, cmd: String) -> Result<String, SvnError> {
        let args: Vec<&str> = cmd.split_whitespace().collect();
        self.get_cmd_out(&args)
    }
}

// following is for private methods
impl SvnCmd {
    fn get_cmd_out(&self, args: &[&str]) -> Result<String, SvnError> {
        let mut all_args: Vec<&str> = Vec::new();
        all_args.extend_from_slice(args);
        self.extra_args
            .split_whitespace()
            .for_each(|s| all_args.push(s));
        SvnWrapper::new().common_cmd_runner(&all_args)
    }

    fn get_cmd_out_cancellable(
        &self,
        args: &[&str],
        runner_context: &RunnerContext,
    ) -> Result<(StdoutFuture, StderrFuture), SvnError> {
        let mut all_args: Vec<&str> = Vec::new();
        all_args.extend_from_slice(args);
        self.extra_args
            .split_whitespace()
            .for_each(|s| all_args.push(s));
        SvnWrapper::new().common_cmd_runner_cancellable(&all_args, runner_context)
    }

    fn log_fetcher(
        args: String,
        target: String,
        (count, start): (RevCount, Option<StartRev>),
    ) -> Result<XmlOut, SvnError> {
        let count_str = format!("-l {}", count.0);
        let rev_range;
        let mut args: Vec<&str> = vec![&args, &count_str];
        if let Some(s) = start {
            rev_range = format!("{}:0", s.0);
            args.extend(vec!["-r", &rev_range]);
        }
        args.push(&target);
        Ok(XmlOut(SvnWrapper::new().common_cmd_runner(&args)?))
    }
}
