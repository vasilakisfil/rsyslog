use nom::{error::VerboseError, number::complete as number, IResult};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn retuple<'a>(
    tuple: Res<&'a str, (&'a str, Option<&'a str>)>,
) -> Res<&'a str, Option<&'a str>> {
    tuple.map(|tuple| (tuple.0, (tuple.1).1))
}

pub fn parse_u8(part: &str) -> Result<u8, nom::Err<VerboseError<&str>>> {
    let (_, part) = number::double(part)?;
    Ok(part as u8)
}

pub fn parse_u32(part: &str) -> Result<u32, nom::Err<VerboseError<&str>>> {
    let (_, part) = number::double(part)?;
    Ok(part as u32)
}
