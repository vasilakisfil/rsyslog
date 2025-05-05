use crate::{Error, ParsePart};

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde-serialize", derive(serde::Serialize))]
pub struct Skip;

impl<'a> ParsePart<'a> for Skip {
    fn parse(part: &'a str) -> Result<(&'a str, Self), Error<'a>> {
        Ok((part, Self))
    }
}

impl<'a> ParsePart<'a> for Vec<Skip> {
    fn parse(part: &'a str) -> Result<(&'a str, Self), Error<'a>> {
        Ok((part, vec![]))
    }
}
