use crate::{Error, ParseMsg};
use nom::bytes::complete::take_until;

#[cfg(feature = "chrono-timestamp")]
impl<'a> ParseMsg<'a> for Option<crate::DateTime> {
    fn parse(part: &'a str) -> Result<(&'a str, Self), Error> {
        let (rem, word) = take_until(" ")(part)?;

        match word {
            "-" => Ok((rem, None)),
            _ => {
                let dt = chrono::DateTime::parse_from_rfc3339(word)
                    .map_err(|_| nom::Err::Error(VerboseError { errors: vec![] }))?;

                Ok((rem, Some(dt)))
            }
        }
    }
}

impl<'a> ParseMsg<'a> for Option<&'a str> {
    fn parse(part: &'a str) -> Result<(&'a str, Self), Error> {
        let (rem, word) = take_until(" ")(part)?;

        match word {
            "-" => Ok((rem, None)),
            _ => Ok((rem, Some(word))),
        }
    }
}
