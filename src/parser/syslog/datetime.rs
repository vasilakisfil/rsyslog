use crate::{DateTime, ParseMsg};
use nom::{bytes::complete::take_until, error::VerboseError, IResult};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

impl<'a> ParseMsg<'a> for Option<DateTime> {
    fn parse(part: &'a str) -> Res<&'a str, Self> {
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
