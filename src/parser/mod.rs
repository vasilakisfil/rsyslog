pub mod datetime;
pub mod helpers;
pub mod msg;
pub mod skip;
pub mod structured_data;

use crate::{Error, Message, NomRes, Originator, ParseMsg, ParsePart};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, space0},
};

pub(crate) fn parse<'a, T: ParsePart<'a>, S: ParsePart<'a>, M: ParseMsg<'a>>(
    msg: &'a str,
) -> Result<Message<'a, T, S, M>, Error<'a>> {
    let (rem, pri) = parse_pri(msg)?;
    let (rem, version) = parse_version(rem)?;
    let (rem, _) = space0(rem)?;
    let (rem, timestamp) = T::parse(rem)?;
    let (rem, _) = space0(rem)?;
    let (rem, hostname) = parse_part(rem)?;
    let (rem, _) = space0(rem)?;
    let (rem, app_name) = parse_part(rem)?;
    let (rem, _) = space0(rem)?;
    let (rem, proc_id) = parse_part(rem)?;
    let (rem, _) = space0(rem)?;
    let (rem, structured_data) = S::parse(rem)?;
    let (rem, _) = space0(rem)?;

    let partial_msg = Originator {
        hostname,
        app_name,
        proc_id,
    };

    let (_, msg) = M::parse(rem, partial_msg)?;

    let message = crate::Message {
        facility: pri >> 3,
        severity: pri & 7,
        version,
        timestamp,
        hostname,
        app_name,
        proc_id,
        structured_data,
        msg,
    };

    Ok(message)
}

fn parse_pri(part: &str) -> NomRes<&str, u8> {
    let (rem, _) = take_until("<")(part)?;
    let (rem, _) = tag("<")(rem)?;

    let (rem, pri) = take_until(">")(rem)?;
    let pri = helpers::parse_u8(pri)?;

    let (rem, _) = tag(">")(rem)?;

    Ok((rem, pri))
}

fn parse_version(part: &str) -> NomRes<&str, u8> {
    let (rem, version) = digit1(part)?;

    Ok((rem, helpers::parse_u8(version)?))
}

fn parse_part<'a>(part: &'a str) -> NomRes<&'a str, Option<&str>> {
    let (rem, word) = take_until(" ")(part)?;

    if word == "-" {
        Ok((rem, None))
    } else {
        Ok((rem, Some(word)))
    }
}
