use chrono::{DateTime, FixedOffset};

mod error;
pub mod parser;

pub use error::Error;
pub use parser::syslog::parse;

type Res<T, U> = nom::IResult<T, U, nom::error::VerboseError<T>>;

pub trait ParseMsg<'a> {
    fn parse(msg: &'a str) -> Res<&'a str, Self>
    where
        Self: Sized;
}

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
