use crate::{parser::helpers, Error, NomRes, Originator, ParseMsg};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::rest,
    sequence::tuple,
};

#[derive(Debug, Eq, PartialEq)]
pub struct HerokuRouter<'a> {
    pub at: &'a str,
    pub code: Option<&'a str>,
    pub desc: Option<&'a str>,
    pub method: &'a str,
    pub path: &'a str,
    pub host: &'a str,
    pub request_id: &'a str,
    pub fwd: &'a str,
    pub dyno: &'a str,
    pub connect: u64,
    pub service: u64,
    pub status: u16,
    pub bytes: Option<u64>,
    pub protocol: &'a str,
}

impl<'a> ParseMsg<'a> for HerokuRouter<'a> {
    fn parse(msg: &'a str, _: &Originator) -> Result<(&'a str, Self), Error<'a>> {
        let (rem, at) = parse_word(msg, "at=", " ")?;
        let (rem, code, desc) = match at {
            "error" => {
                let (rem, code) = parse_word(rem, "code=", " ")?;
                let (rem, desc) = parse_word(rem, "desc=\"", "\" ")?;
                (rem, Some(code), Some(desc))
            }
            _ => (rem, None, None),
        };
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
        let bytes = match bytes {
            "" => None,
            b => Some(b),
        };
        let (rem, protocol) = parse_end_word(rem, "protocol=")?;

        let router = Self {
            at,
            code,
            desc,
            method,
            path,
            host,
            request_id,
            fwd,
            dyno,
            connect: helpers::parse_u64(connect)?,
            service: helpers::parse_u64(service)?,
            status: helpers::parse_u16(status)?,
            bytes: bytes.map(helpers::parse_u64).transpose()?,
            protocol,
        };

        Ok((rem, router))
    }
}

pub fn parse_word<'a>(part: &'a str, start: &'a str, stop: &'a str) -> NomRes<&'a str, &'a str> {
    //TODO: first take until scary here, should be tag instead
    let (rem, (_, _, el)) = tuple((take_until(start), tag(start), take_until(stop)))(part)?;

    Ok((rem, el))
}

pub fn parse_end_word<'a>(part: &'a str, element: &'a str) -> NomRes<&'a str, &'a str> {
    let (rem, (_, _, el)) = tuple((
        take_until(element),
        tag(element),
        alt((take_until("\n\r"), take_until("\n"), take_until(" "), rest)),
    ))(part)?;

    Ok((rem, el))
}
