use chrono::{DateTime, FixedOffset};

mod error;
mod helpers;
mod parser;

pub use parser::parse;
pub use error::Error;

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

pub struct StructuredData {
    id: String,
    params: Vec<SdParam>
}

pub struct SdParam {
    name: String,
    value: String
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

