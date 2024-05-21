use nom::error::VerboseError;

#[derive(Debug, PartialEq)]
pub enum Error<'a> {
    Nom(nom::Err<VerboseError<&'a str>>),
    NomVerbose(String),
    Custom(String),
    #[cfg(feature = "chrono-timestamp")]
    Timestamp(chrono::format::ParseError),
}

impl<'a> Error<'a> {
    pub fn into_detailed_with(self, msg: &'a str) -> Self {
        match self {
            Error::Nom(nom::Err::Error(verbose)) => {
                Error::NomVerbose(nom::error::convert_error(msg, verbose))
            }
            _ => self,
        }
    }
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Nom(_) => write!(f, "nom error"),
            Error::NomVerbose(e) => write!(f, "nom error: {}", e),
            Error::Custom(inner) => write!(f, "{}", inner),
            #[cfg(feature = "chrono-timestamp")]
            Error::Timestamp(e) => write!(f, "{}", e),
        }
    }
}

impl<'a> std::error::Error for Error<'a> {
    /*
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Nom(ref e) => Some(e),
            Error::Custom(_) => None,
            #[cfg(feature = "chrono-timestamp")]
            Error::Timestamp(ref e) => Some(e),
        }
    }
    */
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
