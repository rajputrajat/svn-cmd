//! This lib wraps svn command line tool on your system
#![warn(missing_docs)]
#![warn(unsafe_code)]
use std::result::Result;

mod cmd_wrapper;
mod errors;
mod sub_commands;
mod types;

pub use crate::{
    errors::SvnError,
    sub_commands::{
        info::SvnInfo, list::SvnList, log::SvnLog, status::SvnStatus, version::CmdVersion,
    },
    types::{Credentials, LoginOptions, Optionals},
};

use crate::{
    cmd_wrapper::SvnWrapper,
    sub_commands::log::{RevCount, StartRev, XmlOut},
    types::ToCmdArgs,
};

/// Accessor to svn command functionality
pub struct SvnCmd {
    options: LoginOptions,
    extra_args: String,
}

/// Builder to create SvnCmd
pub struct SvnCmdBuilder {}

impl SvnCmd {
    /// create SvnCmd struct
    pub fn new(creds: Credentials, more: Option<Optionals>) -> Result<SvnCmd, SvnError> {
        let options = LoginOptions {
            credentials: creds,
            more: more.unwrap_or_default(),
        };
        let extra_args = options.to_cmd_args();
        let cmd = SvnCmd {
            options,
            extra_args,
        };
        Ok(cmd)
    }

    /// get svn version installed
    pub async fn version() -> Result<CmdVersion, SvnError> {
        let out = SvnWrapper::new().common_cmd_runner(&["--version"]).await?;
        CmdVersion::parse(&out).await
    }

    /// get list of files
    pub async fn list(&self, target: &str, recursive: bool) -> Result<SvnList, SvnError> {
        let mut args = vec!["list", "--xml", target];
        if recursive {
            args.push("--recursive");
        }
        let xml_text = self.get_cmd_out(&args).await?;
        SvnList::parse(&xml_text)
    }

    /// get diff
    pub async fn diff() -> Result<(), SvnError> {
        Ok(())
    }

    /// read file content
    pub async fn cat() -> Result<String, SvnError> {
        Ok("".to_owned())
    }

    /// SVN ADD command to add new files to stage for commit operation
    /// `svn add PATH`
    pub async fn add(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN COMMIT command to commit changes to remote repo
    /// `svn commit -m "dummy log message"`
    pub async fn commit_local_changes(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN CHECKOUT command: checkout files from mentioned repo url
    /// `svn checkout REPO_URL LOCAL_PATH`
    pub async fn checkout(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN UPDATE command: update local svn dir with remote repo
    /// `svn update`
    pub async fn update(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN LOG command: read svn logs
    /// `svn log REPO_URL | LOCAL_PATH`
    pub async fn log(&self, target: &str) -> Result<SvnLog, SvnError> {
        let mut args = vec!["log", "--xml"];
        args.push(&self.extra_args);
        SvnLog::new(
            &args,
            target,
            Box::new(move |a, b, c| Box::pin(SvnCmd::log_fetcher(a, b, c))),
        )
        .await
    }

    /// SVN STATUS command: svn path status
    /// `svn status PATH`
    pub async fn status(&self, target: &str) -> Result<SvnStatus, SvnError> {
        let out = self.get_cmd_out(&["status", "--xml", target]).await?;
        SvnStatus::parse(&out)
    }

    /// SVN INFO command: read svn info
    /// `svn info PATH`
    pub async fn info(&self, target: &str) -> Result<SvnInfo, SvnError> {
        let out = self.get_cmd_out(&["info", "--xml", target]).await?;
        SvnInfo::parse(&out)
    }

    /// SVN DELETE command: delete file/dir from remote url
    /// `svn delete PATH`
    pub async fn delete(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN REVERT command: revert to specific commit
    /// `svn revert PATH`
    pub async fn revert(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN COPY command: copy from svn repo path to mentioned path
    /// `svn copy CURR_PATH NEW_PATH`
    pub async fn copy_to(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN SWITCH command: switch current working svn path to requested path
    /// `svn switch CURR_PATH NEW_PATH`
    pub async fn switch(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN MERGE command: merge change-sets from mentioned repo
    /// `svn merge [--dry-run] --force From_URL@revN To_URL@revM PATH`
    pub async fn merge(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN IMPORT command: import dir/files from local filesystem
    /// `svn import -m "<commit message>"`
    pub async fn import(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN MKDIR command: create a dir in svn repo
    /// `svn mkdir -m "<commit message>"`
    pub async fn mkdir(&self) -> Result<(), SvnError> {
        Ok(())
    }
}

// following is for private methods
impl SvnCmd {
    async fn get_cmd_out(&self, args: &[&str]) -> Result<String, SvnError> {
        let mut all_args: Vec<&str> = vec![&self.extra_args];
        all_args.extend_from_slice(args);
        SvnWrapper::new().common_cmd_runner(&all_args).await
    }

    async fn log_fetcher(
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
        Ok(XmlOut(SvnWrapper::new().common_cmd_runner(&args).await?))
    }
}
