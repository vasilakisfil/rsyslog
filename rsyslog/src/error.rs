use nom::error::VerboseError;

#[derive(Debug, PartialEq)]
pub enum Error {
    Nom(nom::Err<VerboseError<&'static str>>),
    Timestamp(chrono::format::ParseError),
}

impl From<nom::Err<VerboseError<&'static str>>> for Error {
    fn from(verbose: nom::Err<VerboseError<&'static str>>) -> Self {
        Error::Nom(verbose)
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(verbose: chrono::format::ParseError) -> Self {
        Error::Timestamp(verbose)
    }
}
