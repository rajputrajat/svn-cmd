//! This lib wraps svn command line tool on your system
#![deny(missing_docs)]
#![deny(unsafe_code)]
use log::trace;
use std::result::Result;

mod cmd_wrapper;
mod errors;
mod types;

use cmd_wrapper::*;
use errors::*;
use types::*;

/// Accessor to svn command functionality
pub struct SvnCmd {
    options: LoginOptions,
}

/// Builder to create SvnCmd
pub struct SvnCmdBuilder {}

impl SvnCmd {
    /// create SvnCmd struct
    pub fn new(creds: Credentials, more: Option<Optionals>) -> Result<SvnCmd, SvnError> {
        trace!("");
        Ok(SvnCmd {
            options: LoginOptions {
                credentials: creds,
                more: more.unwrap_or_default(),
            },
        })
    }

    /// get svn version installed
    pub async fn version() -> Result<String, SvnError> {
        trace!("");
        Ok("".to_owned())
    }

    /// get list of files
    pub async fn list() -> Result<(), SvnError> {
        trace!("");
        Ok(())
    }

    /// get diff
    pub async fn diff() -> Result<(), SvnError> {
        trace!("");
        Ok(())
    }

    /// read file content
    pub async fn cat() -> Result<String, SvnError> {
        trace!("");
        Ok("".to_owned())
    }

    /// SVN ADD command to add new files to stage for commit operation
    /// `svn add PATH`
    pub async fn add(&self) -> Result<(), SvnError> {
        trace!("");
        Ok(())
    }

    /// SVN COMMIT command to commit changes to remote repo
    /// `svn commit -m "dummy log message"`
    pub async fn commit_local_changes(&self) -> Result<(), SvnError> {
        trace!("");
        Ok(())
    }

    /// SVN CHECKOUT command: checkout files from mentioned repo url
    /// `svn checkout REPO_URL LOCAL_PATH`
    pub async fn checkout(&self) -> Result<(), SvnError> {
        trace!("");
        Ok(())
    }

    /// SVN UPDATE command: update local svn dir with remote repo
    /// `svn update`
    pub async fn update(&self) -> Result<(), SvnError> {
        trace!("");
        Ok(())
    }

    /// SVN LOG command: read svn logs
    /// `svn log REPO_URL | LOCAL_PATH`
    pub async fn log(&self) -> Result<(), SvnError> {
        trace!("");
        Ok(())
    }

    /// SVN STATUS command: svn path status
    /// `svn status PATH`
    pub async fn status(&self) -> Result<SvnStatus, SvnError> {
        trace!("");
        Ok(SvnStatus {})
    }

    /// SVN INFO command: read svn info
    /// `svn info PATH`
    pub async fn info(&self, target: Target) -> Result<SvnInfo, SvnError> {
        trace!("");
        Ok(SvnInfo {})
    }

    /// SVN DELETE command: delete file/dir from remote url
    /// `svn delete PATH`
    pub async fn delete(&self) -> Result<(), SvnError> {
        trace!("");
        Ok(())
    }

    /// SVN REVERT command: revert to specific commit
    /// `svn revert PATH`
    pub async fn revert(&self) -> Result<(), SvnError> {
        trace!("");
        Ok(())
    }

    /// SVN COPY command: copy from svn repo path to mentioned path
    /// `svn copy CURR_PATH NEW_PATH`
    pub async fn copy_to(&self) -> Result<(), SvnError> {
        trace!("");
        Ok(())
    }

    /// SVN SWITCH command: switch current working svn path to requested path
    /// `svn switch CURR_PATH NEW_PATH`
    pub async fn switch(&self) -> Result<(), SvnError> {
        trace!("");
        Ok(())
    }

    /// SVN MERGE command: merge change-sets from mentioned repo
    /// `svn merge [--dry-run] --force From_URL@revN To_URL@revM PATH`
    pub async fn merge(&self) -> Result<(), SvnError> {
        trace!("");
        Ok(())
    }

    /// SVN IMPORT command: import dir/files from local filesystem
    /// `svn import -m "<commit message>"`
    pub async fn import(&self) -> Result<(), SvnError> {
        trace!("");
        Ok(())
    }

    /// SVN MKDIR command: create a dir in svn repo
    /// `svn mkdir -m "<commit message>"`
    pub async fn mkdir(&self) -> Result<(), SvnError> {
        trace!("");
        Ok(())
    }
}
