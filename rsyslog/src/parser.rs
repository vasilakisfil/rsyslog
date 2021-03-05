use crate::Error;
use chrono::{DateTime, FixedOffset};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, space1},
    combinator::rest,
    error::VerboseError,
    multi::many1,
    sequence::{delimited, pair, tuple},
    IResult,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn parse(msg: &'static str) -> Result<crate::Message, Error> {
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

fn parse_timestamp(timestamp: &str) -> Result<DateTime<FixedOffset>, Error> {
    Ok(chrono::DateTime::parse_from_rfc3339(timestamp)?)
}

fn retuple(
    tuple: Res<&'static str, (&'static str, Option<&'static str>)>,
) -> Res<&'static str, Option<&'static str>> {
    tuple.map(|tuple| (tuple.0, (tuple.1).1))
}

fn parse_optional_structured_data(part: &'static str) -> Res<&'static str, Option<&'static str>> {
    use nom::combinator::map;

    let (rem, data) = alt((
        map(tag("-"), |_| None),
        map(parse_seq_structured_data, |s: Vec<&'static str>| Some(s)),
    ))(part)?;
    let data = data.map(|d| *d.first().unwrap());

    Ok((rem, data))
}

fn parse_seq_structured_data(part: &'static str) -> Res<&'static str, Vec<&'static str>> {
    let (rem, data) = many1(parse_structured_data)(part)?;

    Ok((rem, data))
}

fn parse_structured_data(part: &'static str) -> Res<&'static str, &'static str> {
    delimited::<_, _, _, _, VerboseError<&'static str>, _, _, _>(
        tag("["),
        take_until("]"),
        tag("]"),
    )(part)
}

fn parse_msg(msg: &'static str) -> Res<&'static str, Option<crate::Router>> {
    let (rem, _) = space1(msg)?;
    if tag::<_, _, VerboseError<&'static str>>("-")(rem).is_ok() {
        return Ok((msg, None));
    } else {
        parse_router_msg(msg).map(|(rem, router)| (rem, Some(router)))
    }
}

fn parse_router_msg(msg: &'static str) -> Res<&'static str, crate::Router> {
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
    let (_, protocol) = parse_router_end_word(rem, "protocol=")?;

    let router = crate::Router {
        at: at.into(),
        method: method.into(),
        path: path.into(),
        host: host.into(),
        request_id: request_id.into(),
        fwd: fwd.into(),
        dyno: dyno.into(),
        connect: connect.into(),
        service: service.into(),
        status: crate::helpers::parse_u8(status)?,
        bytes: crate::helpers::parse_u32(bytes)?,
        protocol: protocol.into(),
    };

    Ok((msg, router))
}

fn parse_pri(part: &'static str) -> Res<&'static str, u8> {
    let (rem, _) = take_until("<")(part)?;
    let (rem, _) = tag("<")(rem)?;

    let (rem, pri) = take_until(">")(rem)?;
    let pri = crate::helpers::parse_u8(pri)?;

    let (rem, _) = tag(">")(rem)?;

    Ok((rem, pri))
}

fn parse_version(part: &'static str) -> Res<&'static str, u8> {
    let (rem, version) = digit1(part)?;

    Ok((rem, crate::helpers::parse_u8(version)?))
}

fn parse_word(part: &'static str) -> Res<&'static str, Option<&str>> {
    let (rem, _) = space1(part)?;

    let (rem, word) = take_until(" ")(rem)?;

    if word == "-" {
        Ok((rem, None))
    } else {
        Ok((rem, Some(word)))
    }
}

fn parse_router_word(part: &'static str, element: &'static str) -> Res<&'static str, &'static str> {
    let (rem, (_, _, el)) = tuple((take_until(element), tag(element), take_until(" ")))(part)?;

    Ok((rem, el))
}

fn parse_router_end_word(
    part: &'static str,
    element: &'static str,
) -> Res<&'static str, &'static str> {
    let (rem, (_, _, el)) = tuple((
        take_until(element),
        tag(element),
        alt((take_until(" "), rest)),
    ))(part)?;

    Ok((rem, el))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_structured_data_inner() {
        //parse_structured_data("[exampleSDID@32473 iut=\"3\" eventSource=\"Application\" eventID=\"1011\"]");
        assert_eq!(
            None,
            parse_optional_structured_data("-").expect("parsing data").1
        );
        assert_eq!(
            Some("a"),
            parse_optional_structured_data("[a]")
                .expect("parsing data")
                .1
        );
    }
}
