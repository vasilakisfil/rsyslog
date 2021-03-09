pub mod datetime;
pub mod structured_data;

use crate::{parser::helpers::parse_u8, Error, Message, ParseMsg};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, space0},
    error::VerboseError,
    IResult,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn parse<'a, T: ParseMsg<'a>, S: ParseMsg<'a>, M: ParseMsg<'a>>(
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

    let (_, router) = M::parse(rem)?;

    let message = crate::Message {
        facility: pri >> 3,
        severity: pri & 7,
        version,
        timestamp,
        hostname,
        app_name,
        proc_id,
        structured_data,
        msg: router,
    };

    Ok(message)
}

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

pub fn parse_part<'a>(part: &'a str) -> Res<&'a str, Option<&str>> {
    let (rem, word) = take_until(" ")(part)?;

    if word == "-" {
        Ok((rem, None))
    } else {
        Ok((rem, Some(word)))
    }
}
