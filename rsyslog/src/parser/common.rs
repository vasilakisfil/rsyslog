use nom::{error::VerboseError, IResult};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn retuple<'a>(
    tuple: Res<&'a str, (&'a str, Option<&'a str>)>,
) -> Res<&'a str, Option<&'a str>> {
    tuple.map(|tuple| (tuple.0, (tuple.1).1))
}
