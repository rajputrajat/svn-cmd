use crate::errors::SvnError;
use crate::types::PathType;
use log::trace;
use serde::{
    de::{self, Deserializer},
    Deserialize,
};

/// Return value of SvnCmd . info()
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SvnInfo {
    /// entry field from svn xml
    pub entry: InfoEntry,
}

impl SvnInfo {
    pub(crate) fn parse(xml: &str) -> Result<Self, SvnError> {
        match serde_xml_rs::from_str::<SvnInfo>(xml.trim()) {
            Ok(v) => {
                trace!("{:?}", v);
                Ok(v)
            }
            Err(e) => Err(SvnError::Deserializer(e)),
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct InfoEntry {
    #[serde(deserialize_with = "to_pathtype")]
    pub kind: PathType,
    pub url: String,
    #[serde(rename(deserialize = "relative-url"))]
    pub relative_url: String,
    pub repository: EntryRepository,
    pub commit: EntryCommit,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct EntryRepository {
    pub root: String,
    pub uuid: String,
}

/// commit entry
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct EntryCommit {
    /// revision number
    pub revision: u32,
    /// author name
    pub author: String,
    /// date of commit
    pub date: String,
}

pub(crate) fn to_pathtype<'de, D>(deserializer: D) -> Result<PathType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s == "dir" {
        Ok(PathType::Dir)
    } else if s == "file" {
        Ok(PathType::File)
    } else {
        Err(de::Error::custom("invalid file type"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_xml_data() {
        const XML: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<info>
    <entry path="." revision="324270" kind="dir">
        <url>https://svn.ali.global/GDK_games/GDK_games/BLS/HHR/BuffaloDeluxe/trunk/source</url>
        <relative-url>^/GDK_games/BLS/HHR/BuffaloDeluxe/trunk/source</relative-url>
        <repository>
            <root>https://svn.ali.global/GDK_games</root>
            <uuid>e0c53376-34c8-4e4f-a567-4bb579746d60</uuid>
        </repository>
        <wc-info>
            <wcroot-abspath>C:/Users/rajput/R/svn/nAble/FeatureDevelopment/Monaco/TXM-603_HHR/2.02/trunk/Runtime/core/Games/BuffaloDeluxe</wcroot-abspath>
            <schedule>normal</schedule>
            <depth>infinity</depth>
        </wc-info>
        <commit revision="324270">
            <author>rajput</author>
            <date>2021-08-16T15:02:49.091280Z</date>
        </commit>
    </entry>
</info>"##;

        let info = SvnInfo::parse(XML).unwrap();
        println!("{:#?}", info);
        assert_eq!(info, SvnInfo {
                entry: InfoEntry {
                    kind: PathType::Dir,
                    url: "https://svn.ali.global/GDK_games/GDK_games/BLS/HHR/BuffaloDeluxe/trunk/source".to_owned(),
                    relative_url: "^/GDK_games/BLS/HHR/BuffaloDeluxe/trunk/source".to_owned(),
                    repository: EntryRepository {
                        root: "https://svn.ali.global/GDK_games".to_owned(),
                        uuid: "e0c53376-34c8-4e4f-a567-4bb579746d60".to_owned(),
                    },
                    commit: EntryCommit {
                        revision: 324270,
                        author: "rajput".to_owned(),
                        date: "2021-08-16T15:02:49.091280Z".to_owned(),
                    },
                },
            },
        );
    }
}
