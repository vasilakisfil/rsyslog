use crate::ParseMsg;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::rest,
    error::VerboseError,
    multi::{many0, many1},
    sequence::delimited,
    IResult,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, Eq, PartialEq)]
pub struct StructuredDataList<'a>(pub Vec<StructuredData<'a>>);

impl<'a> From<Vec<StructuredData<'a>>> for StructuredDataList<'a> {
    fn from(list: Vec<StructuredData<'a>>) -> Self {
        Self(list)
    }
}

impl<'a> ParseMsg<'a> for StructuredDataList<'a> {
    fn parse(sd: &'a str) -> Res<&'a str, Self> {
        let (rem, data) = many1(parse_structured_data)(sd)?;

        Ok((rem, Self(data)))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructuredData<'a> {
    pub id: &'a str,
    pub params: Vec<SdParam<'a>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SdParam<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

impl<'a> From<(&'a str, Vec<SdParam<'a>>)> for StructuredData<'a> {
    fn from(tuple: (&'a str, Vec<SdParam<'a>>)) -> Self {
        Self {
            id: tuple.0,
            params: tuple.1,
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for SdParam<'a> {
    fn from(tuple: (&'a str, &'a str)) -> Self {
        Self {
            name: tuple.0,
            value: tuple.1,
        }
    }
}

fn parse_structured_data<'a>(part: &'a str) -> Res<&'a str, StructuredData> {
    let (rem, data) = delimited::<_, _, _, _, VerboseError<&'a str>, _, _, _>(
        tag("["),
        take_until("]"),
        tag("]"),
    )(part)?;

    let (_, data): (&'a str, StructuredData) = parse_structured_data_inner(data)?;

    Ok((rem, data))
}

fn parse_structured_data_inner(part: &str) -> Res<&str, StructuredData> {
    use nom::character::complete::space0;

    let (rem, _) = space0(part)?;
    let (rem, id) = alt((take_until(" "), rest))(rem)?;

    let (rem, sd_params) = many0(parse_structured_elements)(rem)?;

    Ok((rem, (id, sd_params).into()))
}

fn parse_structured_elements<'a>(part: &'a str) -> Res<&'a str, SdParam> {
    use nom::character::complete::space0;

    let (rem, _) = space0(part)?;
    let (rem, key_value) = alt((take_until(" "), rest))(rem)?;
    let (key_value_rem, key) = take_until("=")(key_value)?;
    let (value, _) = tag("=")(key_value_rem)?;

    Ok((rem, (key, value).into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_structured_data() {
        assert_eq!(
            StructuredDataList::parse("[a]").expect("parsing data").1,
            vec![StructuredData {
                id: "a",
                params: vec![]
            }]
            .into(),
        );

        assert_eq!(
            StructuredDataList::parse(
                "[exampleSDID@32473 iut=\"3\" eventSource=\"Application\" eventID=\"1011\"]"
            )
            .expect("parsing data")
            .1,
            vec![StructuredData {
                id: "exampleSDID@32473",
                params: vec![
                    SdParam {
                        name: "iut",
                        value: "\"3\""
                    },
                    SdParam {
                        name: "eventSource",
                        value: "\"Application\""
                    },
                    SdParam {
                        name: "eventID",
                        value: "\"1011\""
                    },
                ]
            }]
            .into(),
        );
    }

    #[test]
    fn simple_structured_data_inner() {
        assert_eq!(
            parse_structured_data_inner("a"),
            Ok((
                "",
                StructuredData {
                    id: "a",
                    params: vec![]
                }
            ))
        );

        assert_eq!(
            parse_structured_data_inner("a key=value anotherkey=anothervalue"),
            Ok((
                "",
                StructuredData {
                    id: "a",
                    params: vec![
                        SdParam {
                            name: "key",
                            value: "value"
                        },
                        SdParam {
                            name: "anotherkey",
                            value: "anothervalue"
                        }
                    ]
                }
            ))
        );

        assert_eq!(
            parse_structured_data_inner(
                "exampleSDID@32473 iut=\"3\" eventSource=\"Application\" eventID=\"1011\""
            ),
            Ok((
                "",
                StructuredData {
                    id: "exampleSDID@32473",
                    params: vec![
                        SdParam {
                            name: "iut",
                            value: "\"3\""
                        },
                        SdParam {
                            name: "eventSource",
                            value: "\"Application\""
                        },
                        SdParam {
                            name: "eventID",
                            value: "\"1011\""
                        }
                    ]
                }
            ))
        );
    }
}
