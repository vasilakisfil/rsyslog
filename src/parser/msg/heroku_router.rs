use crate::{parser::helpers, ParseMsg};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::rest,
    error::VerboseError,
    sequence::tuple,
    IResult,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, Eq, PartialEq)]
pub struct HerokuRouter<'a> {
    pub at: &'a str,
    pub method: &'a str,
    pub path: &'a str,
    pub host: &'a str,
    pub request_id: &'a str,
    pub fwd: &'a str,
    pub dyno: &'a str,
    pub connect: u64,
    pub service: u64,
    pub status: u8,
    pub bytes: u64,
    pub protocol: &'a str,
}

impl<'a> ParseMsg<'a> for HerokuRouter<'a> {
    fn parse(msg: &'a str) -> Res<&'a str, Self> {
        let (rem, at) = parse_word(msg, "at=", " ")?;
        let (rem, method) = parse_word(rem, "method=", " ")?;
        let (rem, path) = parse_word(rem, "path=\"", "\" ")?;
        let (rem, host) = parse_word(rem, "host=", " ")?;
        let (rem, request_id) = parse_word(rem, "request_id=", " ")?;
        let (rem, fwd) = parse_word(rem, "fwd=\"", "\" ")?;
        let (rem, dyno) = parse_word(rem, "dyno=", " ")?;
        let (rem, connect) = parse_word(rem, "connect=", "ms ")?;
        let (rem, service) = parse_word(rem, "service=", "ms ")?;
        let (rem, status) = parse_word(rem, "status=", " ")?;
        let (rem, bytes) = parse_word(rem, "bytes=", " ")?;
        let (rem, protocol) = parse_end_word(rem, "protocol=")?;

        let router = Self {
            at,
            method,
            path,
            host,
            request_id,
            fwd,
            dyno,
            connect: helpers::parse_u64(connect)?,
            service: helpers::parse_u64(service)?,
            status: helpers::parse_u8(status)?,
            bytes: helpers::parse_u64(bytes)?,
            protocol,
        };

        Ok((rem, router))
    }
}

pub fn parse_word<'a>(part: &'a str, start: &'a str, stop: &'a str) -> Res<&'a str, &'a str> {
    let (rem, (_, _, el)) = tuple((take_until(start), tag(start), take_until(stop)))(part)?;

    Ok((rem, el))
}

pub fn parse_end_word<'a>(part: &'a str, element: &'a str) -> Res<&'a str, &'a str> {
    let (rem, (_, _, el)) = tuple((
        take_until(element),
        tag(element),
        alt((take_until(" "), rest)),
    ))(part)?;

    Ok((rem, el))
}