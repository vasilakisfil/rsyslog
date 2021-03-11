use nom::error::VerboseError;

#[derive(Debug, PartialEq)]
pub enum Error<'a> {
    Nom(nom::Err<VerboseError<&'a str>>),
    #[cfg(feature = "chrono-timestamp")]
    Timestamp(chrono::format::ParseError),
}

impl<'a> From<nom::Err<VerboseError<&'a str>>> for Error<'a> {
    fn from(verbose: nom::Err<VerboseError<&'a str>>) -> Self {
        Error::Nom(verbose)
    }
}

#[cfg(feature = "chrono-timestamp")]
impl<'a> From<chrono::format::ParseError> for Error<'a> {
    fn from(verbose: chrono::format::ParseError) -> Self {
        Error::Timestamp(verbose)
    }
}
