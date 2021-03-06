use chrono::{DateTime, FixedOffset};

mod error;
mod helpers;
mod parser;

pub use error::Error;
pub use parser::syslog::parse;

#[derive(Debug, Eq, PartialEq)]
pub struct Message {
    pub facility: u8,
    pub severity: u8,
    pub version: u8,
    pub timestamp: Option<DateTime<FixedOffset>>,
    pub hostname: Option<String>,
    pub app_name: Option<String>,
    pub proc_id: Option<String>,
    pub structured_data: Option<String>,
    pub msg: Option<Router>,
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
pub struct Router {
    pub at: String,
    pub method: String,
    pub path: String,
    pub host: String,
    pub request_id: String,
    pub fwd: String,
    pub dyno: String,
    pub connect: String,
    pub service: String,
    pub status: u8,
    pub bytes: u32,
    pub protocol: String,
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
