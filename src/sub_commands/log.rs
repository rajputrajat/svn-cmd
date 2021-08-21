use crate::errors::SvnError;
use async_std::path::PathBuf;
use chrono::prelude::*;
use log::trace;
use serde::{
    de::{self, Deserializer},
    Deserialize,
};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct SvnLog {
    queue: VecDeque<LogEntry>,
}

#[derive(Debug)]
pub struct LogIter(LogEntry);

#[derive(Deserialize, Debug)]
pub struct LogParser {
    entry: LogEntry,
}

#[derive(Deserialize, Debug)]
pub struct LogEntry {
    revision: u32,
    author: String,
    #[serde(deserialize_with = "to_datetime")]
    date: DateTime<FixedOffset>,
    msg: String,
}

impl Iterator for SvnLog {
    type Item = LogIter;

    fn next(&mut self) -> Option<Self::Item> {}
}

impl SvnLog {}

fn to_datetime<'de, D>(deserialize: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserialize)?;
    DateTime::parse_from_rfc3339(&s).map_err(de::Error::custom)
}
