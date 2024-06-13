use crate::{
    errors::SvnError, sub_commands::info::to_pathtype, sub_commands::info::EntryCommit,
    types::PathType,
};
use log::error;
use serde::Deserialize;
use std::{
    collections::{vec_deque::Iter, HashMap, VecDeque},
    fmt::Display,
    mem,
    ops::Deref,
};

/// svn list
#[derive(Deserialize, Debug, Clone, Default)]
pub struct SvnList {
    /// the list
    pub list: Entry,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Entry {
    pub entry: Option<VecDeque<ListEntry>>,
}

/// on asking create a map from file/dir name to the list_entry
pub struct SvnListMap {
    svn_list: SvnList,
    map: HashMap<String, usize>,
}

/// SvnList is madeup of these entries
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct ListEntry {
    #[serde(deserialize_with = "to_pathtype")]
    /// is file or dir
    pub kind: PathType,
    /// relative path name
    pub name: Option<String>,
    /// file size
    pub size: Option<usize>,
    /// commit structure
    pub commit: EntryCommit,
}

impl Display for ListEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "kind: {:?}, name: {:?}, size: {:?}, commit: {:?}",
            self.kind, self.name, self.size, self.commit
        )
    }
}

impl SvnList {
    /// parse XML text
    pub(crate) fn parse(xml_text: &str) -> Result<Self, SvnError> {
        serde_xml_rs::from_str::<Self>(xml_text.trim()).map_err(|e| {
            error!("serde_xml_rs parsing error '{e:?}'");
            SvnError::Deserializer(e)
        })
    }

    /// returns iterator
    pub fn iter_opt(&self) -> Option<ListInspector> {
        self.list.entry.as_ref().map(|entries| ListInspector {
            iter: entries.iter(),
        })
    }

    /// return SvnListMap
    pub fn into_list_map(mut self) -> SvnListMap {
        let map = self
            .list
            .entry
            .as_mut()
            .map(|entries| {
                entries
                    .iter_mut()
                    .enumerate()
                    .map(|(i, entry)| {
                        let mut name = entry.name.take().unwrap();
                        let name = &mut name; // name will be present as parsed from the svn list xml out
                        (mem::take(name), i)
                    })
                    .collect()
            })
            .unwrap_or(HashMap::new());
        SvnListMap {
            svn_list: self,
            map,
        }
    }
}

impl Deref for SvnListMap {
    type Target = HashMap<String, usize>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

#[derive(Clone)]
pub struct ListInspector<'a> {
    iter: Iter<'a, ListEntry>,
}

impl<'a> Iterator for ListInspector<'a> {
    type Item = &'a ListEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_() {
        let xml_text: SvnList = serde_xml_rs::from_str(LIST_XML_3.trim()).unwrap();
        println!("{:?}", xml_text);
    }

    #[test]
    fn parsing() {
        for list_xml in [LIST_XML_1, LIST_XML_2] {
            let xml_text = SvnList::parse(list_xml).unwrap();
            println!("{:?}", xml_text);
        }
    }

    #[test]
    fn list_iter() {
        for list_xml in [LIST_XML_1, LIST_XML_2, LIST_XML_3] {
            let list = SvnList::parse(list_xml).unwrap();
            (0..10).for_each(|_| {
                if let Some(mut iter) = list.iter_opt() {
                    println!("{:?}\n", iter.next());
                }
            });
        }
    }

    const LIST_XML_1: &str = r##"
<?xml version="1.0" encoding="UTF-8"?>
<lists>
<list
   path=".">
<entry
   kind="file">
<name>.gitignore</name>
<size>47</size>
<commit
   revision="301001">
<author>rajput</author>
<date>2020-10-09T05:40:54.158765Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>BuffaloChief.vcxproj</name>
<size>15213</size>
<commit
   revision="322455">
<author>rajput</author>
<date>2021-07-21T06:55:07.614369Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>BuffaloChief.vcxproj.filters</name>
<size>2583</size>
<commit
   revision="322455">
<author>rajput</author>
<date>2021-07-21T06:55:07.614369Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>BuffaloChief.vcxproj.user</name>
<size>1414</size>
<commit
   revision="317527">
<author>rajput</author>
<date>2021-05-10T06:01:50.939035Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>Jurisdiction.makefile</name>
<size>1240</size>
<commit
   revision="298675">
<author>rajput</author>
<date>2020-09-16T06:28:02.597638Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>Makefile.in</name>
<size>738</size>
<commit
   revision="298675">
<author>rajput</author>
<date>2020-09-16T06:28:02.597638Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>asset_relocation_def.json</name>
<size>798</size>
<commit
   revision="315356">
<author>rajput</author>
<date>2021-04-07T05:50:21.989546Z</date>
</commit>
</entry>
<entry
   kind="dir">
<name>assets</name>
<commit
   revision="322279">
<author>rajput</author>
<date>2021-07-19T07:01:05.938601Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>build_all.ps1</name>
<size>1841</size>
<commit
   revision="322264">
<author>rajput</author>
<date>2021-07-19T04:19:23.747592Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>clean_and_run_buffalo_chief.ps1</name>
<size>808</size>
<commit
   revision="310568">
<author>rajput</author>
<date>2021-02-04T08:51:11.394823Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>configure</name>
<size>2437</size>
<commit
   revision="298675">
<author>rajput</author>
<date>2020-09-16T06:28:02.597638Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>esycpy.exe</name>
<size>2029056</size>
<commit
   revision="313388">
<author>rajput</author>
<date>2021-03-09T07:31:23.286485Z</date>
</commit>
</entry>
<entry
   kind="dir">
<name>lib</name>
<commit
   revision="317527">
<author>rajput</author>
<date>2021-05-10T06:01:50.939035Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>manifest</name>
<size>15987</size>
<commit
   revision="298675">
<author>rajput</author>
<date>2020-09-16T06:28:02.597638Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>paramfile.txt</name>
<size>46</size>
<commit
   revision="316081">
<author>rajput</author>
<date>2021-04-16T14:03:40.573352Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>perforce_data_map.json</name>
<size>354</size>
<commit
   revision="308290">
<author>rajput</author>
<date>2021-01-05T17:04:06.210955Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>setup.txt</name>
<size>818</size>
<commit
   revision="315350">
<author>rajput</author>
<date>2021-04-07T05:43:13.582363Z</date>
</commit>
</entry>
<entry
   kind="dir">
<name>src</name>
<commit
   revision="322533">
<author>rajput</author>
<date>2021-07-22T05:46:30.580142Z</date>
</commit>
</entry>
</list>
</lists>
    "##;

    const LIST_XML_2: &str = r##"
<?xml version="1.0" encoding="UTF-8"?>
<lists>
<list
   path="https://svn.ali.global/GDK_games/GDK_games/BLS/CDS/BonusBoostBaoZhuZhaoFu/PurpleCelebration/branches/devline_sagrawal/source">
<entry
   kind="file">
<name>BaoZhuZhaoFu_PurpleCelebration.vcproj</name>
<size>20617</size>
<commit
   revision="373439">
<author>sa102001</author>
<date>2023-07-21T05:31:45.995541Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>BaoZhuZhaoFu_PurpleCelebration.vcxproj</name>
<size>27302</size>
<commit
   revision="373685">
<author>sa102001</author>
<date>2023-07-25T10:21:50.950544Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>BaoZhuZhaoFu_PurpleCelebration.vcxproj.filters</name>
<size>21408</size>
<commit
   revision="373439">
<author>sa102001</author>
<date>2023-07-21T05:31:45.995541Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>BaoZhuZhaoFu_PurpleCelebration.vcxproj.user</name>
<size>758</size>
<commit
   revision="373439">
<author>sa102001</author>
<date>2023-07-21T05:31:45.995541Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>Jurisdiction.makefile</name>
<size>1248</size>
<commit
   revision="373439">
<author>sa102001</author>
<date>2023-07-21T05:31:45.995541Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>Makefile.in</name>
<size>420</size>
<commit
   revision="373439">
<author>sa102001</author>
<date>2023-07-21T05:31:45.995541Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>ReadMe.txt</name>
<size>15414</size>
<commit
   revision="373439">
<author>sa102001</author>
<date>2023-07-21T05:31:45.995541Z</date>
</commit>
</entry>
<entry
   kind="dir">
<name>assets</name>
<commit
   revision="382999">
<author>sa102001</author>
<date>2024-01-23T09:48:16.976029Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>configure</name>
<size>1542</size>
<commit
   revision="373439">
<author>sa102001</author>
<date>2023-07-21T05:31:45.995541Z</date>
</commit>
</entry>
<entry
   kind="dir">
<name>libs</name>
<commit
   revision="373439">
<author>sa102001</author>
<date>2023-07-21T05:31:45.995541Z</date>
</commit>
</entry>
<entry
   kind="file">
<name>manifest</name>
<size>14107</size>
<commit
   revision="373439">
<author>sa102001</author>
<date>2023-07-21T05:31:45.995541Z</date>
</commit>
</entry>
<entry
   kind="dir">
<name>src</name>
<commit
   revision="386704">
<author>sa102001</author>
<date>2024-03-26T03:12:16.268312Z</date>
</commit>
</entry>
</list>
</lists>
    "##;

    const LIST_XML_3: &str = r##"
<?xml version="1.0" encoding="UTF-8"?>
<lists>
<list>
<entry
   kind="file">
<name>BaoZhuZhaoFu_PurpleCelebration.vcproj</name>
<size>20617</size>
<commit
   revision="373439">
<author>sa102001</author>
<date>2023-07-21T05:31:45.995541Z</date>
</commit>
</entry>
</list>
</lists>
    "##;
}
