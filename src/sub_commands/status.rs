use crate::errors::SvnError;
use log::trace;
use serde::{
    de::{self, Deserializer},
    Deserialize,
};
use std::collections::{hash_map::Entry::Vacant, HashMap};
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
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

/// represents output of svn-status command
#[derive(Debug)]
pub struct SvnStatus {
    /// status data
    pub status: HashMap<StatusItemType, Vec<PathBuf>>,
}

impl SvnStatus {
    pub(crate) fn parse<T: AsRef<str>>(text: T) -> Result<Self, SvnError> {
        let status_vec = StatusParser::parse(text.as_ref().trim())?;
        let mut status_map: HashMap<StatusItemType, Vec<PathBuf>> = HashMap::new();
        for entry in status_vec.target.entry {
            let item = entry.wc_status.item;
            let path = entry.path;
            if let Vacant(e) = status_map.entry(item) {
                e.insert(vec![path]);
            } else {
                status_map.get_mut(&item).unwrap().push(path); // we have already placed a check so unwrap is fine here
            }
        }
        Ok(SvnStatus { status: status_map })
    }
}

/// Return value of SvnCmd . status()
#[derive(Debug, Deserialize)]
struct StatusParser {
    target: StatusTarget,
}

impl StatusParser {
    pub(crate) fn parse<T: AsRef<str>>(text: T) -> Result<Self, SvnError> {
        match serde_xml_rs::from_str::<StatusParser>(text.as_ref()) {
            Ok(v) => {
                trace!("{:?}", v);
                Ok(v)
            }
            Err(e) => Err(SvnError::Deserializer(e)),
        }
    }
}

#[derive(Debug, Deserialize)]
struct StatusTarget {
    entry: Vec<TargetEntry>,
}

#[derive(Debug, Deserialize)]
struct TargetEntry {
    #[serde(deserialize_with = "to_pathbuf")]
    path: PathBuf,
    #[serde(rename(deserialize = "wc-status"))]
    wc_status: WcStatus,
}

#[derive(Debug, Deserialize)]
struct WcStatus {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status() {
        let de = SvnStatus::parse(SVN_STATUS).unwrap();
        println!("{:#?}", de);
    }

    const SVN_STATUS: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<status>
    <target path=".">
        <entry path="GDK\GDKBase">
            <wc-status item="external" props="none"></wc-status>
        </entry>
        <entry path="GDK\Libraries">
            <wc-status props="none" item="external"></wc-status>
        </entry>
        <entry path="GDK\Mercury\src\OpenGL.cpp">
            <wc-status item="modified" revision="156377" props="none">
                <commit revision="142706">
                    <author>ddunford</author>
                    <date>2020-03-02T19:40:25.301780Z</date>
                </commit>
            </wc-status>
        </entry>
        <entry path="GDK\cmake">
            <wc-status props="none" item="external"></wc-status>
        </entry>
        <entry path="GDK\interface">
            <wc-status props="none" item="external"></wc-status>
        </entry>
        <entry path="Games\.vs">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\Binaries - Shortcut.lnk">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\Bootstrap.exe - Debug.lnk">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\Bootstrap.exe - Release.lnk">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\BuffaloChief">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\BuffaloChief.sln">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\BuffaloChief.sln.lnk">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\BuffaloGoldRevolution">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\BuffaloGoldRevolution.sln">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\ChoysKingdomDancingFoo">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\ChoysKingdomDancingFoo.sln">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\ChoysKingdomLunarFestival">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\ChoysKingdomLunarFestival.sln">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\Debug - Monaco Logs.lnk">
            <wc-status props="none" item="unversioned"></wc-status>
        </entry>
        <entry path="Games\FacadeIntegritySuite.sln - Shortcut.lnk">
            <wc-status props="none" item="unversioned"></wc-status>
        </entry>
        <entry path="Games\GDKRuntime_2015.sln - Shortcut.lnk">
            <wc-status props="none" item="unversioned"></wc-status>
        </entry>
        <entry path="Games\GDK_2015.sln - Shortcut.lnk">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\HHR_BuffaloGoldRevolution.sln - Shortcut.lnk">
            <wc-status props="none" item="unversioned"></wc-status>
        </entry>
        <entry path="Games\Monaco bin - Debug.lnk">
            <wc-status props="none" item="unversioned"></wc-status>
        </entry>
        <entry path="Games\Monaco bin - Release.lnk">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\Monaco.sln - Shortcut.lnk">
            <wc-status props="none" item="unversioned"></wc-status>
        </entry>
        <entry path="Games\fit">
            <wc-status props="none" item="unversioned"></wc-status>
        </entry>
        <entry path="Games\reel_strip.patch">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="Games\runtime_before_drop_3.patch">
            <wc-status props="none" item="unversioned"></wc-status>
        </entry>
        <entry path="Games\src.patch">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="GDK\GDKBase\GDKBase.vcxproj.user">
            <wc-status item="unversioned" props="none"></wc-status>
        </entry>
        <entry path="GDK\Libraries\Common.vcxproj.user">
            <wc-status props="none" item="unversioned"></wc-status>
        </entry>
        <entry path="GDK\interface\platform_interface">
            <wc-status props="none" item="external"></wc-status>
        </entry>
    </target>
</status>
    "##;
}
