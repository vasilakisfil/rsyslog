mod error;
pub mod parser;

pub use error::Error;
pub use parser::{
    msg::{HerokuRouter, Raw},
    structured_data::{SdParam, StructuredData},
};

#[cfg(feature = "chrono-timestamp")]
pub type DateTime = chrono::DateTime<chrono::FixedOffset>;

type Res<T, U> = nom::IResult<T, U, nom::error::VerboseError<T>>;

#[cfg(not(feature = "serde-serialize"))]
pub trait ParseMsg<'a> {
    fn parse(msg: &'a str) -> Res<&'a str, Self>
    where
        Self: Sized;
}
#[cfg(feature = "serde-serialize")]
pub trait ParseMsg<'a>: serde::Serialize {
    fn parse(msg: &'a str) -> Res<&'a str, Self>
    where
        Self: Sized;
}

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde-serialize", derive(serde::Serialize))]
pub struct Message<'a, T = Option<&'a str>, S = Vec<StructuredData<'a>>, M = Raw<'a>>
where
    T: ParseMsg<'a>,
    S: ParseMsg<'a>,
    M: ParseMsg<'a>,
{
    pub facility: u8,
    pub severity: u8,
    pub version: u8,
    pub timestamp: T,
    pub hostname: Option<&'a str>,
    pub app_name: Option<&'a str>,
    pub proc_id: Option<&'a str>,
    pub structured_data: S,
    pub msg: M,
}

impl<'a, T, S, M> Message<'a, T, S, M>
where
    T: ParseMsg<'a>,
    S: ParseMsg<'a>,
    M: ParseMsg<'a>,
{
    pub fn parse(msg: &'a str) -> Result<Message<'a, T, S, M>, Error<'a>> {
        parser::parse(msg)
    }
}
