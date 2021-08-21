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
pub struct SvnLog<F>
where
    F: Fn(u32) -> String,
{
    queue: VecDeque<LogEntry>,
    log_fetcher: F,
}

impl<F> SvnLog<F>
where
    F: Fn(u32) -> String,
{
    fn new(log_fetcher: F) -> Result<Self, SvnError> {
        let logger = Self {
            queue: VecDeque::new(),
            log_fetcher,
        };
        logger.fetch(10)?;
        Ok(logger)
    }

    fn fetch(&mut self, count: u32) -> Result<(), SvnError> {
        let text = (self.log_fetcher)(count);
        LogParser::parse(&text).map(|vl| {
            self.queue.extend(vl.entry);
        })
    }
}

#[derive(Debug)]
pub struct LogIter(LogEntry);

#[derive(Deserialize, Debug)]
pub struct LogParser {
    entry: Vec<LogEntry>,
}

#[derive(Deserialize, Debug)]
pub struct LogEntry {
    revision: u32,
    author: String,
    #[serde(deserialize_with = "to_datetime")]
    date: DateTime<FixedOffset>,
    msg: String,
}

impl<F> Iterator for SvnLog<F>
where
    F: Fn(u32) -> String,
{
    type Item = LogIter;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            self.fetch(10);
        }
        self.queue.pop_front().map(|v| LogIter(v))
    }
}

impl LogParser {
    fn parse(text: &str) -> Result<Self, SvnError> {
        serde_xml_rs::from_str::<Self>(text).map_err(|e| SvnError::Deserializer { src: e })
    }
}

fn to_datetime<'de, D>(deserialize: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserialize)?;
    DateTime::parse_from_rfc3339(&s).map_err(de::Error::custom)
}
