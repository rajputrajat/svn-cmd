use crate::types::PathType;
use serde::{
    de::{self, Deserializer},
    Deserialize,
};
use url::Url;

#[derive(Debug, Deserialize)]
struct InfoEntry {
    #[serde(deserialize_with = "to_pathtype")]
    kind: PathType,
    #[serde(deserialize_with = "to_url")]
    url: Url,
    relative_url: String,
    repository: EntryRepository,
}

fn to_pathtype<'de, D>(deserializer: D) -> Result<PathType, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s == "dir" {
        Ok(PathType::Dir)
    } else if s == "file" {
        Ok(PathType::File)
    } else {
        Err(de::Error::custom("invalid file type"))
    }
}

fn to_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Url::parse(s).map_err(de::Error::custom)
}

#[derive(Debug, Deserialize)]
struct EntryRepository {
    root: String,
    uuid: String,
}

#[derive(Debug, Deserialize)]
struct EntryCommit {
    revision: u32,
    author: String,
    date: String,
}

/// Return value of SvnCmd . info()
#[derive(Debug, Deserialize)]
pub(crate) struct SvnInfo {
    entry: InfoEntry,
}

impl SvnInfo {
    pub(crate) fn parse<T: Into<String>>(xml: T) {}
}
