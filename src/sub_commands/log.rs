use crate::errors::SvnError;
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
        let mut logger = Self {
            queue: VecDeque::new(),
            log_fetcher,
        };
        logger.fetch(10)?;
        Ok(logger)
    }

    fn fetch(&mut self, count: u32) -> Result<(), SvnError> {
        let text = (self.log_fetcher)(count);
        LogParser::parse(&text).map(|vl| {
            self.queue.extend(vl.logentry);
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct LogParser {
    logentry: Vec<LogEntry>,
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
    type Item = LogEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            let _ = self.fetch(10);
        }
        self.queue.pop_front()
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn fetch_logs() {
        let fetcher = |_: u32| {
            let out = Command::new("svn")
                .args(&[
                    "log",
                    "--xml",
                    "-l",
                    "10",
                    "https://svn.ali.global/GDK_games/GDK_games/BLS/NYL/",
                ])
                .output()
                .unwrap();
            String::from_utf8(out.stdout).unwrap()
        };

        let mut sl = SvnLog::new(fetcher).unwrap();
        (0..30).for_each(|_| {
            println!("{:?}", sl.next());
        });
        assert!(false);
    }

    #[test]
    fn parse() {
        let de = serde_xml_rs::from_str::<LogParser>(&LOG_SAMPLE).unwrap();
        println!("{:?}", de);
        assert!(false);
    }

    const LOG_SAMPLE: &str = r##"
<?xml version="1.0" encoding="UTF-8"?>
<log>
<logentry
   revision="324641">
<author>rs102580</author>
<date>2021-08-20T10:46:56.484066Z</date>
<msg>Jira Issue:
            - [GS88LFNYL-26: Complete SIT Checklist and Deliver Release Candidate](https://biggestlittlestudio.atlassian.net/browse/GS88LFNYL-26)
Explanation for Change:
            - Creating RC03 tag https://svn.ali.global/GDK_games/GDK_games/BLS/NYL/GS88LunarFestival/tags/gampro_usa_1.01.67101_RC03  from https://svn.ali.global/GDK_games/GDK_games/BLS/NYL/GS88LunarFestival/trunk@HEAD trunk.
Possible Impact:
            - NA.
</msg>
</logentry>
<logentry
   revision="324640">
<author>rs102580</author>
<date>2021-08-20T10:42:11.300901Z</date>
<msg>Merged revision(s) 324639 from GDK_games/BLS/NYL/GS88LunarFestival/branches/devline_srahul:
Jira Issue:
            - [GS88LFNYL-15: Deliver Final math with all vars, all denoms, etc. to ENG for FIT validation](https://biggestlittlestudio.atlassian.net/browse/GS88LFNYL-15)
Explanation for Change:
            - FIT has passed with the latest math drop.
Possible Impact:
            - NA.

........
Review: NA.</msg>
</logentry>
<logentry
   revision="324639">
<author>rs102580</author>
<date>2021-08-20T10:40:20.504584Z</date>
<msg>Jira Issue:
            - [GS88LFNYL-15: Deliver Final math with all vars, all denoms, etc. to ENG for FIT validation](https://biggestlittlestudio.atlassian.net/browse/GS88LFNYL-15)
Explanation for Change:
            - FIT has passed with the latest math drop.
Possible Impact:
            - NA.
</msg>
</logentry>
<logentry
   revision="324632">
<author>rs102580</author>
<date>2021-08-20T09:07:58.892422Z</date>
<msg>Merged revision(s) 324631 from GDK_games/BLS/NYL/GS88LunarFestival/branches/devline_srahul:
Jira Issue:
            - [GS88LFNYL-50: Create the GCD](https://biggestlittlestudio.atlassian.net/browse/GS88LFNYL-50)
Explanation for Change:
            - Updated the Game Configuration Sheet - GS88LunarFestival_NYL.docx file with the latest platform build update.
Possible Impact:
            - NA.

........
Review: NA.</msg>
</logentry>
<logentry
   revision="324631">
<author>rs102580</author>
<date>2021-08-20T09:06:34.855827Z</date>
<msg>Jira Issue:
            - [GS88LFNYL-50: Create the GCD](https://biggestlittlestudio.atlassian.net/browse/GS88LFNYL-50)
Explanation for Change:
            - Updated the Game Configuration Sheet - GS88LunarFestival_NYL.docx file with the latest platform build update.
Possible Impact:
            - NA.
</msg>
</logentry>
<logentry
   revision="324629">
<author>rs102580</author>
<date>2021-08-20T08:59:07.168075Z</date>
<msg>Merged revision(s) 324627 from GDK_games/BLS/NYL/GS88LunarFestival/branches/devline_srahul:
Jira Issue:
            - [GS88LFNYL-54: Remove Hand Pay page from Help mercury](https://biggestlittlestudio.atlassian.net/browse/GS88LFNYL-54)
Explanation for Change:
            - Updated the help.mercury file just to remove the hand pay page.
Possible Impact:
            - NA.

........
Review: NA.</msg>
</logentry>
<logentry
   revision="324627">
<author>rs102580</author>
<date>2021-08-20T08:54:50.189131Z</date>
<msg>Jira Issue:
            - [GS88LFNYL-54: Remove Hand Pay page from Help mercury](https://biggestlittlestudio.atlassian.net/browse/GS88LFNYL-54)
Explanation for Change:
            - Updated the help.mercury file just to remove the hand pay page.
Possible Impact:
            - NA.
</msg>
</logentry>
<logentry
   revision="324576">
<author>jstrub</author>
<date>2021-08-19T23:02:48.982535Z</date>
<msg>More removal of delays and animations, to make the mock game more LS-like.</msg>
</logentry>
<logentry
   revision="324550">
<author>goela</author>
<date>2021-08-19T10:24:13.473620Z</date>
<msg>Merged revision(s) 324549 from GDK_games/BLS/NYL/GoldStack88DancingFoo/branches/devline_goela/source:
Issue Fixed:-

-GS88DFNYL-70( Remove Hand Pay page from Help mercury )
........
</msg>
</logentry>
<logentry
   revision="324549">
<author>goela</author>
<date>2021-08-19T10:22:54.573528Z</date>
<msg>Issue Fixed:-

-GS88DFNYL-70( Remove Hand Pay page from Help mercury )</msg>
</logentry>
</log>
    "##;
}
