pub mod structured_data;

use crate::{parser::helpers::parse_u8, Error, Message, ParseMsg};
use chrono::{DateTime, FixedOffset};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, space0, space1},
    combinator::map,
    error::VerboseError,
    IResult,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn parse<'a, S: ParseMsg<'a>, M: ParseMsg<'a>>(msg: &'a str) -> Result<Message<S, M>, Error> {
    let (rem, pri) = parse_pri(msg)?;
    let (rem, version) = parse_version(rem)?;
    let (rem, _) = space1(rem)?;
    let (rem, timestamp) = parse_part(rem)?;
    let (rem, _) = space1(rem)?;
    let (rem, hostname) = parse_part(rem)?;
    let (rem, _) = space1(rem)?;
    let (rem, app_name) = parse_part(rem)?;
    let (rem, _) = space1(rem)?;
    let (rem, proc_id) = parse_part(rem)?;
    let (rem, _) = space1(rem)?;
    let (rem, structured_data) = alt((map(tag("-"), |_| None), map(S::parse, Some)))(rem)?;
    let (rem, _) = space0(rem)?;

    let (_, router) = M::parse(rem)?;

    let message = crate::Message {
        facility: pri >> 3,
        severity: pri & 7,
        version,
        timestamp: timestamp.map(parse_timestamp).transpose()?,
        hostname,
        app_name,
        proc_id,
        structured_data,
        msg: router,
    };

    Ok(message)
}

/*
fn parse_remaining<'a>(part: &'a str) -> Res<&'a str, Msg<Router>> {
    rest(part).map(|(rem, remaining)| (rem, Msg::Raw(remaining)))
}*/

fn parse_pri(part: &str) -> Res<&str, u8> {
    let (rem, _) = take_until("<")(part)?;
    let (rem, _) = tag("<")(rem)?;

    let (rem, pri) = take_until(">")(rem)?;
    let pri = parse_u8(pri)?;

    let (rem, _) = tag(">")(rem)?;

    Ok((rem, pri))
}

fn parse_version(part: &str) -> Res<&str, u8> {
    let (rem, version) = digit1(part)?;

    Ok((rem, parse_u8(version)?))
}

fn parse_timestamp(timestamp: &str) -> Result<DateTime<FixedOffset>, Error> {
    Ok(chrono::DateTime::parse_from_rfc3339(timestamp)?)
}

pub fn parse_part<'a>(part: &'a str) -> Res<&'a str, Option<&str>> {
    let (rem, word) = take_until(" ")(part)?;

    if word == "-" {
        Ok((rem, None))
    } else {
        Ok((rem, Some(word)))
    }
}
