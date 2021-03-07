use chrono::{DateTime, FixedOffset};

mod error;
pub mod parser;

pub use error::Error;
pub use parser::syslog::parse;
pub use parser::syslog::structured_data::{SdParam, StructuredData, StructuredDataList};

type Res<T, U> = nom::IResult<T, U, nom::error::VerboseError<T>>;

pub trait ParseMsg<'a> {
    fn parse(msg: &'a str) -> Res<&'a str, Self>
    where
        Self: Sized;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Message<'a, S, M> {
    pub facility: u8,
    pub severity: u8,
    pub version: u8,
    pub timestamp: Option<DateTime<FixedOffset>>,
    pub hostname: Option<&'a str>,
    pub app_name: Option<&'a str>,
    pub proc_id: Option<&'a str>,
    pub structured_data: Option<S>,
    pub msg: M,
}
