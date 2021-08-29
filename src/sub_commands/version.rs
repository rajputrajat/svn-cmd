use crate::cmd_wrapper::SvnWrapper;
use crate::SvnError;
use regex::Regex;
use semver::Version;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct RevisionNumber(u32);

#[derive(Debug, PartialEq)]
pub struct CmdVersion {
    cmd_path: PathBuf,
    version: Version,
    built_rev: RevisionNumber,
}

impl CmdVersion {
    pub(crate) async fn parse(cmd_out: &str) -> Result<Self, SvnError> {
        let re = Regex::new(r"(\d+\.\d+\.\d+).*r(\d+)")
            .map_err(|e| SvnError::Other(format!("error while running regex: {:?}", e)))?;
        //let out = CmdVersion::get_cmd_out().await?;
        let first_line = cmd_out
            .lines()
            .next()
            .ok_or_else(|| SvnError::Other("svn --version output is empty".to_owned()))?;
        let mut cap_iter = re.captures_iter(first_line);
        let captures = cap_iter
            .next()
            .ok_or_else(|| SvnError::Other("regex failed".to_owned()))?;
        let version = &captures[1];
        let version = Version::parse(version)
            .map_err(|e| SvnError::Other(format!("error while parsing semver: {:?}", e)))?;
        let built_rev = &captures[2];
        let built_rev =
            RevisionNumber(built_rev.parse::<u32>().map_err(|e| {
                SvnError::Other(format!("invalid num: {:?}, e: {:?}", built_rev, e))
            })?);
        let cmd_path = which::which("svn")
            .map_err(|e| SvnError::Other(format!("which not found for svn: {:?}", e)))?;
        Ok(Self {
            cmd_path,
            version,
            built_rev,
        })
    }

    pub(crate) async fn get_cmd_out() -> Result<String, SvnError> {
        SvnWrapper::new().common_cmd_runner(&["--version"]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn check_version_parser() {
        let cmd_ver = CmdVersion::parse(VERSION_OUT).await.unwrap();
        assert_eq!(
            cmd_ver,
            CmdVersion {
                cmd_path: PathBuf::from("C:\\Program Files\\TortoiseSVN\\bin\\svn.EXE"),
                version: Version::new(1, 14, 1),
                built_rev: RevisionNumber(1_886_195)
            }
        );
    }

    const VERSION_OUT: &str = r##"svn, version 1.14.1 (r1886195)
   compiled Feb  9 2021, 20:19:00 on x86-microsoft-windows

Copyright (C) 2021 The Apache Software Foundation.
This software consists of contributions made by many people;
see the NOTICE file for more information.
Subversion is open source software, see http://subversion.apache.org/"##;
}
