use crate::{Error, ParseMsg};

pub struct Skip;

impl<'a> ParseMsg<'a> for Skip {
    fn parse(part: &'a str) -> Result<(&'a str, Self), Error> {
        Ok((part, Self))
    }
}

impl<'a> ParseMsg<'a> for Vec<Skip> {
    fn parse(part: &'a str) -> Result<(&'a str, Self), Error> {
        Ok((part, vec![]))
    }
}
