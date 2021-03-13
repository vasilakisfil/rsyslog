use crate::{Error, ParseMsg};
use nom::combinator::rest;

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
    fn parse(msg: &'a str) -> Result<(&'a str, Self), Error> {
        let (rem, msg) = rest(msg)?;

        Ok((rem, msg.into()))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct LineRaw<'a> {
    pub msg: &'a str,
}

impl<'a> From<&'a str> for LineRaw<'a> {
    fn from(msg: &'a str) -> Self {
        Self { msg }
    }
}

impl<'a> ParseMsg<'a> for LineRaw<'a> {
    fn parse(msg: &'a str) -> Result<(&'a str, Self), Error> {
        //TODO: should use terminated with is_not maybe ?
        use nom::{
            character::complete::{line_ending, not_line_ending},
            sequence::terminated,
        };

        let (rem, msg) = terminated(not_line_ending, line_ending)(msg)?;

        Ok((rem, msg.into()))
    }
}
