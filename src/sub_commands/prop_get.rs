use crate::SvnError;
use itertools::Itertools;
use log::error;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SvnPropget {
    /// target
    pub target: Target,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Target {
    /// requested url
    pub path: String,
    /// property entry
    pub property: Vec<Property>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Property {
    /// name of the property
    #[serde(deserialize_with = "deserialize_property_name")]
    pub name: PropertyName,
    #[serde(deserialize_with = "deserialize_external_paths")]
    #[serde(rename = "$value")]
    pub paths: Vec<ExternalPath>,
}

impl SvnPropget {
    /// parse XML text
    pub(crate) fn parse(xml_text: &str) -> Result<Self, SvnError> {
        serde_xml_rs::from_str::<Self>(xml_text.trim()).map_err(|e| {
            error!("serde_xml_rs parsing error '{e:?}'");
            SvnError::Deserializer(e)
        })
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum PropertyName {
    SvnExternals,
    Other(String),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ExternalPath {
    pub name: String,
    pub relative_path: String,
}

pub(crate) fn deserialize_property_name<'de, D>(deserializer: D) -> Result<PropertyName, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(match s.as_str() {
        "svn:externals" => PropertyName::SvnExternals,
        other => PropertyName::Other(other.to_owned()),
    })
}

fn deserialize_external_paths<'de, D>(deserializer: D) -> Result<Vec<ExternalPath>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.split_whitespace()
        .tuples()
        .map(|(name, relative_path)| ExternalPath {
            name: name.to_owned(),
            relative_path: relative_path.to_owned(),
        })
        .collect_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result as AnyResult;

    #[test]
    fn prop_get() -> AnyResult<()> {
        let props = SvnPropget::parse(PROP_LIST)?;
        println!("{props:#?}");
        Ok(())
    }

    const PROP_LIST: &str = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <properties>
    <target   path="https://svn.ali.global/GDK_games/GDK_games/BLS/Class_II/FuDaiLianLian_Boost/MarsPortrait/Peacock/tags/gampro_usa_1.01_68099_RC05_PC01_Signed/source/lib">
    <property   name="svn:externals">^/GDK_games/BLS/Class_II/FuDaiLianLian_Boost/MarsPortrait/FuDaiLianLianCommon/tags/Release.009/source FuDaiLianLianCommon
    ^/GDK_games/BLS/Class_II/FuDaiLianLian_Boost/MarsPortrait/NitroCommon/tags/Release.003/source NitroCommon
    ^/GDK_games/BLS/Class_II/FuDaiLianLian_Boost/MarsPortrait/NitroParticles/tags/Release.003/source NitroParticles
    </property>
    </target>
    </properties>
    "##;
}
