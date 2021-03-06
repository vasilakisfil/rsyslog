use nom::{error::VerboseError, number::complete as number};

pub fn parse_u8<'a>(part: &'a str) -> Result<u8, nom::Err<VerboseError<&'a str>>> {
    let (_, part) = number::double(part)?;
    Ok(part as u8)
}

pub fn parse_u32<'a>(part: &'a str) -> Result<u32, nom::Err<VerboseError<&'a str>>> {
    let (_, part) = number::double(part)?;
    Ok(part as u32)
}
