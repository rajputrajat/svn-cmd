use crate::{
    errors::SvnError, sub_commands::info::to_pathtype, sub_commands::info::EntryCommit,
    types::PathType,
};
use serde::Deserialize;
use std::{collections::vec_deque::Iter, collections::VecDeque};

#[derive(Deserialize, Debug)]
pub struct SvnList {
    list: ListsList,
}

#[derive(Deserialize, Debug)]
struct ListsList {
    entry: VecDeque<ListEntry>,
}

#[derive(Deserialize, Debug)]
pub struct ListEntry {
    #[serde(deserialize_with = "to_pathtype")]
    pub kind: PathType,
    pub name: String,
    size: Option<u32>,
    pub commit: EntryCommit,
}

impl SvnList {
    pub(crate) fn parse(xml_text: &str) -> Result<Self, SvnError> {
        serde_xml_rs::from_str::<Self>(xml_text).map_err(|e| SvnError::Deserializer { src: e })
    }

    pub fn iter(&self) -> ListInspector {
        ListInspector {
            iter: self.list.entry.iter(),
        }
    }
}

pub struct ListInspector<'a> {
    iter: Iter<'a, ListEntry>,
}

impl Iterator for SvnList {
    type Item = ListEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.entry.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let xml_text = SvnList::parse(LIST_XML).unwrap();
        println!("{:?}", xml_text);
        assert!(false);
    }

    #[test]
    fn list_iter() {
        let mut list = SvnList::parse(LIST_XML).unwrap();
        (0..10).for_each(|_| {
            println!("{:?}\n", list.next());
        });
        assert!(false);
    }

    const LIST_XML: &str = r##"
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
}
