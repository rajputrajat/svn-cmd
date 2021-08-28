use crate::cmd_wrapper::SvnWrapper;
use crate::SvnError;
use regex::Regex;
use semver::Version;
use std::path::PathBuf;

pub struct RevisionNumber(u32);

pub struct CmdVersion {
    cmd_path: PathBuf,
    version: Version,
    built_rev: RevisionNumber,
}

impl CmdVersion {
    async fn new() -> Result<Self, SvnError> {
        let out = SvnWrapper::new().common_cmd_runner(&["--version"]).await?;
        let re = Regex::new(r"(\d+\.\d+\.\d+).*(r\d+)")
            .map_err(|e| SvnError::Other(format!("error while running regex: {:?}", e)))?;
        let first_line = out
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
        let built_rev = RevisionNumber(built_rev.parse::<u32>().unwrap());
        let cmd_path = which::which("svn")
            .map_err(|e| SvnError::Other(format!("which not found for svn: {:?}", e)))?;
        Ok(Self {
            cmd_path,
            version,
            built_rev,
        })
    }
}
