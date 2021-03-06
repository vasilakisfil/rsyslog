use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::rest,
    error::VerboseError,
    sequence::tuple,
    IResult,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn parse_router_msg<'a>(msg: &'a str) -> Res<&'a str, crate::Router> {
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
