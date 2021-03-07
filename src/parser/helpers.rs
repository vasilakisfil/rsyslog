use nom::{error::VerboseError, number::complete as number, IResult};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn retuple<'a, T>(tuple: Res<&'a str, (&'a str, Option<T>)>) -> Res<&'a str, Option<T>> {
    tuple.map(|tuple| (tuple.0, (tuple.1).1))
}

pub fn parse_u8(part: &str) -> Result<u8, nom::Err<VerboseError<&str>>> {
    let (_, part) = number::double(part)?;
    Ok(part as u8)
}

pub fn parse_u64(part: &str) -> Result<u64, nom::Err<VerboseError<&str>>> {
    let (_, part) = number::double(part)?;
    Ok(part as u64)
}
