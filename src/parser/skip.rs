use crate::{Error, ParsePart};

#[derive(Debug, Eq, PartialEq)]
pub struct Skip;

impl<'a> ParsePart<'a> for Skip {
    fn parse(part: &'a str) -> Result<(&'a str, Self), Error> {
        Ok((part, Self))
    }
}

impl<'a> ParsePart<'a> for Vec<Skip> {
    fn parse(part: &'a str) -> Result<(&'a str, Self), Error> {
        Ok((part, vec![]))
    }
}
