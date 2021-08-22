use crate::errors::SvnError;
use crate::sub_commands::info::EntryCommit;
use serde::Deserialize;
use std::collections::VecDeque;

#[derive(Deserialize, Debug)]
pub struct SvnList {
    list: ListsList,
}

#[derive(Deserialize, Debug)]
struct ListsList {
    entries: VecDeque<ListEntry>,
}

#[derive(Deserialize, Debug)]
pub struct ListEntry {
    name: String,
    size: u32,
    commit: EntryCommit,
}

impl SvnList {
    pub(crate) fn parse(xml_text: &str) -> Result<Self, SvnError> {
        serde_xml_rs::from_str::<Self>(xml_text).map_err(|e| SvnError::Deserializer { src: e })
    }
}

impl Iterator for SvnList {
    type Item = ListEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.entries.pop_front()
    }
}
