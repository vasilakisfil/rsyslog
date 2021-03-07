use chrono::{DateTime, FixedOffset};

mod error;
mod parser;

pub use error::Error;
pub use parser::syslog::parse;

#[derive(Debug, Eq, PartialEq)]
pub struct Message<'a, T> {
    pub facility: u8,
    pub severity: u8,
    pub version: u8,
    pub timestamp: Option<DateTime<FixedOffset>>,
    pub hostname: Option<&'a str>,
    pub app_name: Option<&'a str>,
    pub proc_id: Option<&'a str>,
    pub structured_data: Option<&'a str>,
    pub msg: T,
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub struct StructuredData<'a> {
    id: &'a str,
    params: Vec<SdParam<'a>>,
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub struct SdParam<'a> {
    name: &'a str,
    value: &'a str,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Router<'a> {
    pub at: &'a str,
    pub method: &'a str,
    pub path: &'a str,
    pub host: &'a str,
    pub request_id: &'a str,
    pub fwd: &'a str,
    pub dyno: &'a str,
    pub connect: &'a str,
    pub service: &'a str,
    pub status: u8,
    pub bytes: u32,
    pub protocol: &'a str,
}

impl<'a> From<(&'a str, Vec<SdParam<'a>>)> for StructuredData<'a> {
    fn from(tuple: (&'a str, Vec<SdParam<'a>>)) -> Self {
        Self {
            id: tuple.0,
            params: tuple.1,
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for SdParam<'a> {
    fn from(tuple: (&'a str, &'a str)) -> Self {
        Self {
            name: tuple.0,
            value: tuple.1,
        }
    }
}
