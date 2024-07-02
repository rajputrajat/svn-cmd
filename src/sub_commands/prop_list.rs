use super::prop_get::{deserialize_property_name, PropertyName};
use crate::SvnError;
use log::error;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SvnProplist {
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
}

impl SvnProplist {
    /// parse XML text
    pub(crate) fn parse(xml_text: &str) -> Result<Self, SvnError> {
        serde_xml_rs::from_str::<Self>(xml_text.trim()).map_err(|e| {
            error!("serde_xml_rs parsing error '{e:?}'");
            SvnError::Deserializer(e)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result as AnyResult;

    #[test]
    fn prop_list() -> AnyResult<()> {
        let props = SvnProplist::parse(PROP_LIST)?;
        println!("{props:#?}");
        Ok(())
    }

    const PROP_LIST: &str = r##"
        <?xml version="1.0" encoding="UTF-8"?>
        <properties>
        <target   path="https://svn.ali.global/GDK_games/GDK_games/BLS/Class_II/FuDaiLianLian_Boost/MarsPortrait/Peacock/tags/gampro_usa_1.01_68099_RC05_PC01_Signed/source/lib">
        <property   name="svn:externals"/>
        </target>
        </properties>
    "##;

    const _PROP_LIST_ERROR: &str = r##"
        <?xml version="1.0" encoding="UTF-8"?><properties>svn: E170013: Unable to connect to a repository at URL 'https://svn.ali.global/GDK_games/GDK_games/BLS/Class_II/FuDaiLianLian_Boost/MarsPortrait/Peacock/tags/gampro_usa_1.01_68099_RC05_PC01_Signed/source/liv'svn: E731001: No such host is known.
    "##;
}
