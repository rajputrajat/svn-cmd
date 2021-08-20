use crate::types::PathType;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
struct InfoEntry {
    #[serde(deserialize_with = "into_pathtype")]
    kind: PathType,
    url: String,
    relative_url: String,
    repository: EntryRepository,
}

fn into_pathtype<'de, D>(deserializer: D) -> Result<PathType, D::Error>
where
    D: Deserializer<'de>,
{
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
