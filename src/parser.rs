use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, space1},
    combinator::rest,
    error::VerboseError,
    sequence::tuple,
    IResult,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn parser(msg: &'static str) -> Res<&'static str, Message> {
    let (rem, pri) = parse_pri(msg)?;
    let (rem, version) = parse_version(rem)?;
    let (rem, timestamp) = parse_timestamp(rem)?;

    let (rem, _) = space1(rem)?;
    let (rem, hostname) = take_until(" ")(rem)?;

    let (rem, _) = space1(rem)?;
    let (rem, app_name) = take_until(" ")(rem)?;

    let (rem, _) = space1(rem)?;
    let (rem, proc_id) = take_until(" ")(rem)?;

    let (_, router) = parse_router_msg(rem)?;

    let message = Message {
        facility: pri >> 3,
        severity: pri & 7,
        version,
        timestamp: timestamp.into(),
        hostname: hostname.into(),
        app_name: app_name.into(),
        proc_id: proc_id.into(),
        msg: router,
    };

    Ok((msg, message))
}

fn parse_router_msg(msg: &'static str) -> Res<&'static str, Router> {
    let (rem, (_, _, at)) = tuple((take_until("at="), tag("at="), take_until(" ")))(msg)?;

    let (rem, (_, _, method)) =
        tuple((take_until("method="), tag("method="), take_until(" ")))(rem)?;

    let (rem, (_, _, path)) = tuple((take_until("path="), tag("path="), take_until(" ")))(rem)?;

    let (rem, (_, _, host)) = tuple((take_until("host="), tag("host="), take_until(" ")))(rem)?;

    let (rem, (_, _, request_id)) = tuple((
        take_until("request_id="),
        tag("request_id="),
        take_until(" "),
    ))(rem)?;

    let (rem, (_, _, fwd)) = tuple((take_until("fwd="), tag("fwd="), take_until(" ")))(rem)?;

    let (rem, (_, _, dyno)) = tuple((take_until("dyno="), tag("dyno="), take_until(" ")))(rem)?;

    let (rem, (_, _, connect)) =
        tuple((take_until("connect="), tag("connect="), take_until(" ")))(rem)?;

    let (rem, (_, _, service)) =
        tuple((take_until("service="), tag("service="), take_until(" ")))(rem)?;

    let (rem, (_, _, status)) =
        tuple((take_until("status="), tag("status="), take_until(" ")))(rem)?;

    let (rem, (_, _, bytes)) = tuple((take_until("bytes="), tag("bytes="), take_until(" ")))(rem)?;

    let (_, (_, _, protocol)) = tuple((
        take_until("protocol="),
        tag("protocol="),
        alt((take_until(" "), rest)),
    ))(rem)?;

    let router = Router {
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

pub fn parse_pri(part: &'static str) -> Res<&'static str, u8> {
    let (rem, _) = take_until("<")(part)?;
    let (rem, _) = tag("<")(rem)?;

    let (rem, pri) = take_until(">")(rem)?;
    let pri = crate::helpers::parse_u8(pri)?;

    let (rem, _) = tag(">")(rem)?;

    Ok((rem, pri))
}

pub fn parse_version(part: &'static str) -> Res<&'static str, u8> {
    let (rem, version) = digit1(part)?;

    Ok((rem, crate::helpers::parse_u8(version)?))
}

pub fn parse_timestamp(part: &'static str) -> Res<&'static str, &str> {
    let (rem, _) = space1(part)?;

    let (rem, timestamp) = take_until(" ")(rem)?;

    Ok((rem, timestamp))
}

#[derive(Debug)]
pub struct Message {
    facility: u8,
    severity: u8,
    version: u8,
    timestamp: String,
    hostname: String,
    app_name: String,
    proc_id: String,
    msg: Router,
}

#[derive(Debug)]
pub struct Router {
    at: String,
    method: String,
    path: String,
    host: String,
    request_id: String,
    fwd: String,
    dyno: String,
    connect: String,
    service: String,
    status: u8,
    bytes: u32,
    protocol: String,
}
