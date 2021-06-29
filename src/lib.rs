#![deny(missing_docs)]
#![deny(unsafe_code)]
use std::result::Result;
use thiserror::Error;

/// Accessor to svn command functionality
pub struct SvnCmd {}

/// Builder to create SvnCmd
pub struct SvnCmdBuilder {}

impl SvnCmdBuilder {
    pub fn commit_local_changes(&self) -> Result<(), SvnError> {
        Ok(())
    }

    pub fn checkout(&self) -> Result<(), SvnError> {
        Ok(())
    }

    pub fn update(&self) -> Result<(), SvnError> {
        Ok(())
    }

    pub fn log(&self) -> Result<(), SvnError> {
        Ok(())
    }

    pub fn status(&self) -> Result<SvnStatus, SvnError> {
        Ok(SvnStatus {})
    }

    pub fn info(&self) -> Result<SvnInfo, SvnError> {
        Ok(SvnInfo {})
    }

    pub fn delete(&self) -> Result<(), SvnError> {
        Ok(())
    }

    pub fn revert(&self) -> Result<(), SvnError> {
        Ok(())
    }

    pub fn copy_to(&self) -> Result<(), SvnError> {
        Ok(())
    }

    pub fn switch_repo_locally(&self) -> Result<(), SvnError> {
        Ok(())
    }

    pub fn merge() -> Result<(), SvnError> {
        Ok(())
    }
}

pub struct SvnInfo {}
pub struct SvnStatus {}

#[derive(Error, Debug)]
pub enum SvnError {
    #[error("no connectivity")]
    Disconnection,
}
