use crate::{Error, NomRes, ParsePart};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::map,
    combinator::rest,
    error::VerboseError,
    multi::{many0, many1},
    sequence::delimited,
};

impl<'a> ParsePart<'a> for Vec<StructuredData<'a>> {
    fn parse(sd: &'a str) -> Result<(&'a str, Self), Error<'a>> {
        let (rem, sdata) = alt((map(tag("-"), |_| vec![]), many1(parse_structured_data)))(sd)?;

        Ok((rem, sdata))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde-serialize", derive(serde::Serialize))]
pub struct StructuredData<'a> {
    pub id: &'a str,
    pub params: Vec<SdParam<'a>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde-serialize", derive(serde::Serialize))]
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

fn parse_structured_data<'a>(part: &'a str) -> NomRes<&'a str, StructuredData<'a>> {
    let (rem, data) = delimited::<_, _, _, _, VerboseError<&'a str>, _, _, _>(
        tag("["),
        take_until("]"),
        tag("]"),
    )(part)?;

    let (_, data): (&'a str, StructuredData) = parse_structured_data_inner(data)?;

    Ok((rem, data))
}

fn parse_structured_data_inner(part: &str) -> NomRes<&str, StructuredData> {
    use nom::character::complete::space0;

    let (rem, _) = space0(part)?;
    let (rem, id) = alt((take_until(" "), rest))(rem)?;

    let (rem, sd_params) = many0(parse_structured_elements)(rem)?;

    Ok((rem, (id, sd_params).into()))
}

fn parse_structured_elements(part: &str) -> NomRes<&str, SdParam> {
    use nom::character::complete::space0;

    let (rem, _) = space0(part)?;
    let (rem, key_value) = alt((take_until(" "), rest))(rem)?;
    let (key_value_rem, key) = take_until("=")(key_value)?;
    let (value, _) = tag("=")(key_value_rem)?;

    let value: &str = &value[1..value.len() - 1];

    Ok((rem, (key, value).into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_structured_data1() {
        let msg = "[a]";
        assert_eq!(
            <Vec<StructuredData> as ParsePart>::parse(msg)
                .expect("parsing data")
                .1,
            vec![StructuredData {
                id: "a",
                params: vec![]
            }]
        );
    }

    #[test]
    fn simple_structured_data2() {
        let msg = r#"[exampleSDID@32473 iut="3" eventSource="Application" eventID="1011"]"#;
        assert_eq!(
            <Vec<StructuredData> as ParsePart>::parse(msg)
                .expect("parsing data")
                .1,
            vec![StructuredData {
                id: "exampleSDID@32473",
                params: vec![
                    SdParam {
                        name: "iut",
                        value: "3"
                    },
                    SdParam {
                        name: "eventSource",
                        value: "Application"
                    },
                    SdParam {
                        name: "eventID",
                        value: "1011"
                    },
                ]
            }]
        );
    }

    #[test]
    fn simple_structured_data_inner1() {
        let msg = "a";
        assert_eq!(
            parse_structured_data_inner(msg),
            Ok((
                "",
                StructuredData {
                    id: "a",
                    params: vec![]
                }
            ))
        );
    }

    #[test]
    fn simple_structured_data_inner2() {
        let msg = r#"a key="value" anotherkey="anothervalue""#;
        assert_eq!(
            parse_structured_data_inner(msg),
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
    }

    #[test]
    fn simple_structured_data_inner3() {
        let msg = r#"exampleSDID@32473 iut="3" eventSource="Application" eventID="1011""#;
        assert_eq!(
            parse_structured_data_inner(msg),
            Ok((
                "",
                StructuredData {
                    id: "exampleSDID@32473",
                    params: vec![
                        SdParam {
                            name: "iut",
                            value: "3"
                        },
                        SdParam {
                            name: "eventSource",
                            value: "Application"
                        },
                        SdParam {
                            name: "eventID",
                            value: "1011"
                        }
                    ]
                }
            ))
        );
    }
}
