use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_till, take_until, take_while},
    character::complete::{digit1, space1},
    combinator::rest,
    error::{ErrorKind, VerboseError, VerboseErrorKind},
    number::complete as number,
    regexp::str::re_find,
    sequence::tuple,
    Err as NomErr, IResult,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn main() {
    let msg = r#"258 <158>1 2021-02-25T13:04:19.887695+00:00 host heroku router - at=info method=POST path="/api/v1/events/smartcam" host=ratatoskr.mobility46.se request_id=5599e09a-f8e3-4ed9-8be8-6883ce842cf2 fwd="157.230.107.240" dyno=web.1 connect=0ms service=97ms status=200 bytes=140 protocol=https"#;
    let msg = r#"258 <158>1 2021-02-25T13:04:19.887695+00:00 host heroku router - at=info method=POST path="/api/v1/events/smartcam" host=ratatoskr.mobility46.se request_id=5599e09a-f8e3-4ed9-8be8-6883ce842cf2 fwd="157.230.107.240" dyno=web.1 connect=0ms service=97ms status=200 bytes=140 protocol=https"#;
    match parser(msg) {
        Ok((_, msg)) => println!("{:?}", msg),
        Err(err) => match err {
            NomErr::Error(e) => println!("{}", nom::error::convert_error(msg, e)),
            _ => println!("{}", err),
        },
    }
}

fn parser(msg: &'static str) -> Res<&'static str, Message> {
    let (rem, _) = take_until("<")(msg)?;
    let (rem, _) = tag("<")(rem)?;
    let (rem, pri) = take_until(">")(rem)?;
    let (_, pri) = number::double(pri)?;
    let pri = pri as u8;

    let (rem, _) = tag(">")(rem)?;

    let (rem, version) = digit1(rem)?;
    let (_, version) = number::double(version)?;
    let version = version as u8;

    let (rem, _) = space1(rem)?;

    let (rem, timestamp) = take_until(" ")(rem)?;

    let (rem, _) = space1(rem)?;
    let (rem, hostname) = take_until(" ")(rem)?;

    let (rem, _) = space1(rem)?;
    let (rem, app_name) = take_until(" ")(rem)?;

    let (rem, _) = space1(rem)?;
    let (rem, proc_id) = take_until(" ")(rem)?;

    let (rem, router) = parse_router_msg(rem)?;

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

    let (rem, (_, _, protocol)) = tuple((
        take_until("protocol="),
        tag("protocol="),
        alt((take_until(" "), rest)),
    ))(rem)?;

    let router = Router {
        at: at.into(),
        method: method.into(),
        path: path.into(),
        request_id: request_id.into(),
        fwd: fwd.into(),
        dyno: dyno.into(),
        connect: connect.into(),
        service: service.into(),
        status: parse_u8(status)?,
        bytes: parse_u32(bytes)?,
        protocol: protocol.into(),
    };

    Ok((msg, router))
}

fn parse_u8(part: &'static str) -> Result<u8, NomErr<VerboseError<&'static str>>> {
    let (_, part) = number::double(part)?;
    Ok(part as u8)
}

fn parse_u32(part: &'static str) -> Result<u32, NomErr<VerboseError<&'static str>>> {
    let (_, part) = number::double(part)?;
    Ok(part as u32)
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
    request_id: String,
    fwd: String,
    dyno: String,
    connect: String,
    service: String,
    status: u8,
    bytes: u32,
    protocol: String,
}
