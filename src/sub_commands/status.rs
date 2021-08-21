use crate::errors::SvnError;
use async_std::path::PathBuf;
use log::trace;
use serde::{
    de::{self, Deserializer},
    Deserialize,
};

#[derive(Debug)]
pub enum StatusItemType {
    Unversioned,
    External,
    Added,
    Removed,
    Modified,
}

impl StatusItemType {
    fn parse<T: AsRef<str>>(item: T) -> Result<Self, SvnError> {
        match item.as_ref() {
            "unversioned" => Ok(Self::Unversioned),
            "modified" => Ok(Self::Modified),
            "added" => Ok(Self::Added),
            "external" => Ok(Self::External),
            "removed" => Ok(Self::Removed),
            _ => Err(SvnError::Other("unhandled item type".to_owned())),
        }
    }
}

/// Return value of SvnCmd . status()
#[derive(Debug, Deserialize)]
pub struct SvnStatus {
    target: StatusTarget,
}

impl SvnStatus {
    pub(crate) fn parse<T: AsRef<str>>(text: T) -> Result<Self, SvnError> {
        match serde_xml_rs::from_str::<SvnStatus>(text.as_ref()) {
            Ok(v) => {
                trace!("{:?}", v);
                Ok(v)
            }
            Err(e) => Err(SvnError::Deserializer { src: e }),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StatusTarget {
    entry: Vec<TargetEntry>,
}

#[derive(Debug, Deserialize)]
pub struct TargetEntry {
    #[serde(deserialize_with = "to_pathbuf")]
    path: PathBuf,
    #[serde(rename(deserialize = "wc-status"))]
    wc_status: WcStatus,
}

#[derive(Debug, Deserialize)]
pub struct WcStatus {
    #[serde(deserialize_with = "to_itemtype")]
    item: StatusItemType,
}

fn to_pathbuf<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(PathBuf::from(&s))
}

fn to_itemtype<'de, D>(deserializer: D) -> Result<StatusItemType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    StatusItemType::parse(s).map_err(de::Error::custom)
}
