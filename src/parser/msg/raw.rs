use crate::{Error, Originator, ParseMsg};
use nom::combinator::rest;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Raw<'a> {
    pub msg: &'a str,
}

impl<'a> From<&'a str> for Raw<'a> {
    fn from(msg: &'a str) -> Self {
        Self { msg }
    }
}

impl<'a> ParseMsg<'a> for Raw<'a> {
    fn parse(msg: &'a str, _: &Originator) -> Result<(&'a str, Self), Error<'a>> {
        let (rem, msg) = rest(msg)?;

        Ok((rem, msg.into()))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LineRaw<'a> {
    pub msg: &'a str,
}

impl<'a> From<&'a str> for LineRaw<'a> {
    fn from(msg: &'a str) -> Self {
        Self { msg }
    }
}

impl<'a> ParseMsg<'a> for LineRaw<'a> {
    fn parse(msg: &'a str, _: &Originator) -> Result<(&'a str, Self), Error<'a>> {
        //TODO: should use terminated with is_not maybe ?
        use nom::{
            branch::alt,
            character::complete::{line_ending, not_line_ending},
            sequence::terminated,
        };

        let (rem, msg) = alt((terminated(not_line_ending, line_ending), rest))(msg)?;

        Ok((rem, msg.into()))
    }
}
