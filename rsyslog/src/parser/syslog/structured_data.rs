use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    error::VerboseError,
    multi::many1,
    sequence::delimited,
    IResult,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn parse_optional_structured_data<'a>(part: &'a str) -> Res<&'a str, Option<&'a str>> {
    use nom::combinator::map;

    let (rem, data) = alt((
        map(tag("-"), |_| None),
        map(parse_seq_structured_data, |s: Vec<&'a str>| Some(s)),
    ))(part)?;
    let data = data.map(|d| *d.first().unwrap());

    Ok((rem, data))
}

fn parse_seq_structured_data<'a>(part: &'a str) -> Res<&'a str, Vec<&'a str>> {
    let (rem, data) = many1(parse_structured_data)(part)?;

    Ok((rem, data))
}

fn parse_structured_data<'a>(part: &'a str) -> Res<&'a str, &'a str> {
    delimited::<_, _, _, _, VerboseError<&'a str>, _, _, _>(tag("["), take_until("]"), tag("]"))(
        part,
    )
}

/*
fn parse_structured_data_inner(part: &'a str) -> Result<&'a str, Error> {
    Ok(part)
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_structured_data_inner() {
        //parse_structured_data("[exampleSDID@32473 iut=\"3\" eventSource=\"Application\" eventID=\"1011\"]");
        assert_eq!(
            None,
            parse_optional_structured_data("-").expect("parsing data").1
        );
        assert_eq!(
            Some("a"),
            parse_optional_structured_data("[a]")
                .expect("parsing data")
                .1
        );
    }
}
