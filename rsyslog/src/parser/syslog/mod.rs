pub mod structured_data;

use crate::{
    parser::{
        helpers::{parse_u8, retuple},
    },
    Error, Message,
    ParseMsg
};
use chrono::{DateTime, FixedOffset};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, space1, space0},
    error::VerboseError,
    sequence::pair,
    IResult,
};
use structured_data::parse_optional_structured_data;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn parse<'a, T: ParseMsg<'a>>(msg: &'a str) -> Result<Message<T>, Error> {
    let (rem, pri) = parse_pri(msg)?;
    let (rem, version) = parse_version(rem)?;
    let (rem, timestamp) = parse_part(rem)?;
    let (rem, hostname) = parse_part(rem)?;
    let (rem, app_name) = parse_part(rem)?;
    let (rem, proc_id) = parse_part(rem)?;
    let (rem, structured_data) = retuple(pair(space1, parse_optional_structured_data)(rem))?;
    let (rem, _) = space0(rem)?;

    let (_, router) = T::parse(rem)?;

    let message = crate::Message {
        facility: pri >> 3,
        severity: pri & 7,
        version,
        timestamp: timestamp.map(parse_timestamp).transpose()?,
        hostname: hostname,
        app_name: app_name,
        proc_id: proc_id,
        structured_data: structured_data,
        msg: router,
    };

    Ok(message)
}

/*
fn parse_remaining<'a>(part: &'a str) -> Res<&'a str, Msg<Router>> {
    rest(part).map(|(rem, remaining)| (rem, Msg::Raw(remaining)))
}*/

fn parse_pri<'a>(part: &'a str) -> Res<&'a str, u8> {
    let (rem, _) = take_until("<")(part)?;
    let (rem, _) = tag("<")(rem)?;

    let (rem, pri) = take_until(">")(rem)?;
    let pri = parse_u8(pri)?;

    let (rem, _) = tag(">")(rem)?;

    Ok((rem, pri))
}

fn parse_version<'a>(part: &'a str) -> Res<&'a str, u8> {
    let (rem, version) = digit1(part)?;

    Ok((rem, parse_u8(version)?))
}

fn parse_timestamp<'a>(timestamp: &str) -> Result<DateTime<FixedOffset>, Error> {
    Ok(chrono::DateTime::parse_from_rfc3339(timestamp)?)
}

pub fn parse_part<'a>(part: &'a str) -> Res<&'a str, Option<&str>> {
    let (rem, _) = space1(part)?;

    let (rem, word) = take_until(" ")(rem)?;

    if word == "-" {
        Ok((rem, None))
    } else {
        Ok((rem, Some(word)))
    }
}
