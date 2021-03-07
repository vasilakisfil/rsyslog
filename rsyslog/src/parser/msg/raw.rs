use crate::{ParseMsg};
use nom::{
    combinator::rest,
    error::VerboseError,
    IResult,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, Eq, PartialEq)]
pub struct Raw<'a> {
    pub msg: &'a str,
}

impl<'a> From<&'a str> for Raw<'a> {
    fn from(msg: &'a str) -> Self {
        Self { msg }
    }
}

impl<'a> ParseMsg<'a> for Raw<'a> {
    fn parse(msg: &'a str) -> Res<&'a str, Self> {
        let (rem, msg) = rest(msg)?;

        Ok((rem, msg.into()))
    }
}
