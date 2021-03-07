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
    pub connect: &'a str,
    pub service: &'a str,
    pub status: u8,
    pub bytes: u32,
    pub protocol: &'a str,
}

impl<'a> ParseMsg<'a> for HerokuRouter<'a> {
    fn parse(msg: &'a str) -> Res<&'a str, Self> {
        let (rem, at) = parse_router_word(msg, "at=")?;
        let (rem, method) = parse_router_word(rem, "method=")?;
        let (rem, path) = parse_router_word(rem, "path=")?;
        let (rem, host) = parse_router_word(rem, "host=")?;
        let (rem, request_id) = parse_router_word(rem, "request_id=")?;
        let (rem, fwd) = parse_router_word(rem, "fwd=")?;
        let (rem, dyno) = parse_router_word(rem, "dyno=")?;
        let (rem, connect) = parse_router_word(rem, "connect=")?;
        let (rem, service) = parse_router_word(rem, "service=")?;
        let (rem, status) = parse_router_word(rem, "status=")?;
        let (rem, bytes) = parse_router_word(rem, "bytes=")?;
        let (rem, protocol) = parse_router_end_word(rem, "protocol=")?;

        let router = Self {
            at: at,
            method: method,
            path: path,
            host: host,
            request_id: request_id,
            fwd: fwd,
            dyno: dyno,
            connect: connect,
            service: service,
            status: helpers::parse_u8(status)?,
            bytes: helpers::parse_u32(bytes)?,
            protocol: protocol,
        };

        Ok((rem, router))
    }
}

pub fn parse_router_word<'a>(part: &'a str, element: &'a str) -> Res<&'a str, &'a str> {
    let (rem, (_, _, el)) = tuple((take_until(element), tag(element), take_until(" ")))(part)?;

    Ok((rem, el))
}

pub fn parse_router_end_word<'a>(part: &'a str, element: &'a str) -> Res<&'a str, &'a str> {
    let (rem, (_, _, el)) = tuple((
        take_until(element),
        tag(element),
        alt((take_until(" "), rest)),
    ))(part)?;

    Ok((rem, el))
}
