//! This lib wraps svn command line tool on your system
#![deny(missing_docs)]
#![deny(unsafe_code)]
use std::result::Result;
use thiserror::Error;

/// Accessor to svn command functionality
pub struct SvnCmd {}

/// Builder to create SvnCmd
pub struct SvnCmdBuilder {}

impl SvnCmd {
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
    pub fn log(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN STATUS command: svn path status
    /// `svn status PATH`
    pub fn status(&self) -> Result<SvnStatus, SvnError> {
        Ok(SvnStatus {})
    }

    /// SVN INFO command: read svn info
    /// `svn info PATH`
    pub fn info(&self) -> Result<SvnInfo, SvnError> {
        Ok(SvnInfo {})
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
    pub fn switch_repo_locally(&self) -> Result<(), SvnError> {
        Ok(())
    }

    /// SVN MERGE command: merge change-sets from mentioned repo
    /// `svn merge [--dry-run] --force From_URL@revN To_URL@revM PATH`
    pub fn merge() -> Result<(), SvnError> {
        Ok(())
    }
}

/// Return value of SvnCmd . info()
pub struct SvnInfo {}

/// Return value of SvnCmd . status()
pub struct SvnStatus {}

/// lib specific error type
#[derive(Error, Debug)]
pub enum SvnError {
    /// no connection
    #[error("no connectivity")]
    Disconnection,
}
