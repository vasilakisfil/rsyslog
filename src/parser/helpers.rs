use nom::{error::VerboseError, number::complete as number};

pub fn parse_u8(part: &str) -> Result<u8, nom::Err<VerboseError<&str>>> {
    let (_, part) = number::double(part)?;
    Ok(part as u8)
}

pub fn parse_u64(part: &str) -> Result<u64, nom::Err<VerboseError<&str>>> {
    let (_, part) = number::double(part)?;
    Ok(part as u64)
}
