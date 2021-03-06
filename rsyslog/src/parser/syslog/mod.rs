pub mod structured_data;

use crate::parser::{
    common::{parse_word, retuple},
    heroku::router::parse_msg,
};
use crate::Error;
use chrono::{DateTime, FixedOffset};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, space1},
    error::VerboseError,
    sequence::pair,
    IResult,
};
use structured_data::parse_optional_structured_data;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn parse<'a>(msg: &'a str) -> Result<crate::Message, Error> {
    let (rem, pri) = parse_pri(msg)?;
    let (rem, version) = parse_version(rem)?;
    let (rem, timestamp) = parse_word(rem)?;
    let (rem, hostname) = parse_word(rem)?;
    let (rem, app_name) = parse_word(rem)?;
    let (rem, proc_id) = parse_word(rem)?;
    let (rem, structured_data) = retuple(pair(space1, parse_optional_structured_data)(rem))?;

    let (_, router) = parse_msg(rem)?;

    let message = crate::Message {
        facility: pri >> 3,
        severity: pri & 7,
        version,
        timestamp: timestamp.map(parse_timestamp).transpose()?,
        hostname: hostname.map(Into::into),
        app_name: app_name.map(Into::into),
        proc_id: proc_id.map(Into::into),
        structured_data: structured_data.map(Into::into),
        msg: router,
    };

    Ok(message)
}

fn parse_pri<'a>(part: &'a str) -> Res<&'a str, u8> {
    let (rem, _) = take_until("<")(part)?;
    let (rem, _) = tag("<")(rem)?;

    let (rem, pri) = take_until(">")(rem)?;
    let pri = crate::helpers::parse_u8(pri)?;

    let (rem, _) = tag(">")(rem)?;

    Ok((rem, pri))
}

fn parse_version<'a>(part: &'a str) -> Res<&'a str, u8> {
    let (rem, version) = digit1(part)?;

    Ok((rem, crate::helpers::parse_u8(version)?))
}

fn parse_timestamp<'a>(timestamp: &str) -> Result<DateTime<FixedOffset>, Error> {
    Ok(chrono::DateTime::parse_from_rfc3339(timestamp)?)
}
