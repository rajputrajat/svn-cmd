use crate::errors::SvnError;
use async_std::{future::Future, pin::Pin, task::block_on};
use fix_hidden_lifetime_bug::fix_hidden_lifetime_bug;
use serde::{
    de::{self, Deserializer},
    Deserialize,
};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct RevCount(pub u32);
#[derive(Debug, Clone, Copy)]
pub struct StartRev(pub u32);
#[derive(Debug)]
pub struct XmlOut(pub String);

pub(crate) type LogFetcher = Box<
    dyn Fn(
        String,
        String,
        (RevCount, Option<StartRev>),
    ) -> Pin<Box<dyn Future<Output = Result<XmlOut, SvnError>>>>,
>;

pub struct SvnLog {
    queue: VecDeque<LogEntry>,
    last_entry_revision: Option<StartRev>,
    args: String,
    target: String,
    fetcher: LogFetcher,
}

impl SvnLog {
    #[fix_hidden_lifetime_bug]
    pub(crate) async fn new(
        args: &[&str],
        target: &str,
        fetcher: LogFetcher,
    ) -> Result<Self, SvnError> {
        let mut logger = Self {
            queue: VecDeque::new(),
            last_entry_revision: None,
            args: args.iter().map(|s| format!(" {} ", s)).collect(),
            target: target.to_owned(),
            fetcher,
        };
        logger.fetch((RevCount(10), None)).await?;
        Ok(logger)
    }

    async fn fetch(
        &mut self,
        (count, start): (RevCount, Option<StartRev>),
    ) -> Result<(), SvnError> {
        let text: String = (self.fetcher)(
            self.args.clone(),
            self.target.clone(),
            (count, start.map(|s| StartRev(s.0 - 1))),
        )
        .await?
        .0;
        LogParser::parse(&text).map(|vl| {
            self.queue.extend(vl.logentry);
        })?;
        if let Some(b) = self.queue.back() {
            self.last_entry_revision = Some(StartRev(b.revision));
        }
        Ok(())
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
    date: String,
    msg: String,
}

impl Iterator for SvnLog {
    type Item = LogEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            let _ = block_on(self.fetch((RevCount(10), self.last_entry_revision)));
        }
        self.queue.pop_front()
    }
}

impl LogParser {
    fn parse(text: &str) -> Result<Self, SvnError> {
        serde_xml_rs::from_str::<Self>(text).map_err(|e| SvnError::Deserializer { src: e })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[async_std::test]
    async fn fetch_logs() {
        let mut sl: SvnLog = SvnLog::new("https://svn.ali.global/GDK_games/GDK_games/BLS/NYL/")
            .await
            .unwrap();
        (0..40).for_each(|_| {
            println!("{:?}\n", sl.next());
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
