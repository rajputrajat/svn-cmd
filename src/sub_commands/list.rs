use crate::cmd_wrapper::SvnWrapper;
use crate::errors::SvnError;
use chrono::prelude::*;
use serde::{
    de::{self, Deserializer},
    Deserialize,
};

pub struct SvnList {}

impl SvnList {}

pub struct ListEntry {}

impl Iterator for SvnList {
    type Item = ListEntry;
    fn next(&mut self) -> Option<Self::Item> {}
}
