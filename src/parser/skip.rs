use crate::ParseMsg;
use nom::{error::VerboseError, IResult};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub struct Skip;

impl<'a> ParseMsg<'a> for Skip {
    fn parse(part: &'a str) -> Res<&'a str, Self> {
        Ok((part, Self))
    }
}

impl<'a> ParseMsg<'a> for Vec<Skip> {
    fn parse(part: &'a str) -> Res<&'a str, Self> {
        Ok((part, vec![]))
    }
}
