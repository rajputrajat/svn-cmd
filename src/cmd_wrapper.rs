//! this module will implement all svn cmd wrapper

use std::process::Command;

/// cmd wrapper struct
pub(crate) fn svn_wrapper() -> String {
    let cmd = Command::new("svn").arg("");
}
