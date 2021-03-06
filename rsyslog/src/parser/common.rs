use nom::{bytes::complete::take_until, character::complete::space1, error::VerboseError, IResult};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn parse_word<'a>(part: &'a str) -> Res<&'a str, Option<&str>> {
    let (rem, _) = space1(part)?;

    let (rem, word) = take_until(" ")(rem)?;

    if word == "-" {
        Ok((rem, None))
    } else {
        Ok((rem, Some(word)))
    }
}

pub fn retuple<'a>(
    tuple: Res<&'a str, (&'a str, Option<&'a str>)>,
) -> Res<&'a str, Option<&'a str>> {
    tuple.map(|tuple| (tuple.0, (tuple.1).1))
}
