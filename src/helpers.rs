use nom::{error::VerboseError, number::complete as number};

pub fn parse_u8(part: &'static str) -> Result<u8, nom::Err<VerboseError<&'static str>>> {
    let (_, part) = number::double(part)?;
    Ok(part as u8)
}

pub fn parse_u32(part: &'static str) -> Result<u32, nom::Err<VerboseError<&'static str>>> {
    let (_, part) = number::double(part)?;
    Ok(part as u32)
}
