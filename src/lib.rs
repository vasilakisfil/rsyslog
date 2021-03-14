mod error;
pub mod parser;

pub use error::Error;
pub(crate) type NomRes<T, U> = nom::IResult<T, U, nom::error::VerboseError<T>>;

#[cfg(not(feature = "serde-serialize"))]
pub trait ParsePart<'a> {
    fn parse(msg: &'a str) -> Result<(&'a str, Self), Error>
    where
        Self: Sized;
}
#[cfg(feature = "serde-serialize")]
pub trait ParsePart<'a>: serde::Serialize {
    fn parse(msg: &'a str) -> Result<(&'a str, Self), Error>
    where
        Self: Sized;
}

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde-serialize", derive(serde::Serialize))]
pub struct Message<
    'a,
    T = Option<&'a str>,
    S = Vec<parser::StructuredData<'a>>,
    M = parser::msg::Raw<'a>,
> where
    T: ParsePart<'a>,
    S: ParsePart<'a>,
    M: ParseMsg<'a>,
{
    pub facility: u8,
    pub severity: u8,
    pub version: u8,
    pub timestamp: T,
    pub hostname: Option<&'a str>,
    pub app_name: Option<&'a str>,
    pub proc_id: Option<&'a str>,
    pub msg_id: Option<&'a str>,
    pub structured_data: S,
    pub msg: M,
}

impl<'a, T, S, M> Message<'a, T, S, M>
where
    T: ParsePart<'a>,
    S: ParsePart<'a>,
    M: ParseMsg<'a>,
{
    pub fn parse(msg: &'a str) -> Result<Message<'a, T, S, M>, Error<'a>> {
        parser::parse(msg).map(|tuple| tuple.1)
    }

    pub fn parse_with_rem(msg: &'a str) -> Result<(&'a str, Message<'a, T, S, M>), Error<'a>> {
        parser::parse(msg)
    }

    pub fn iter(msg: &'a str) -> MessageIter<'a, T, S, M> {
        MessageIter {
            rem: msg,
            t: std::marker::PhantomData,
            s: std::marker::PhantomData,
            m: std::marker::PhantomData,
        }
    }
}

pub struct MessageIter<'a, T, S, M>
where
    T: ParsePart<'a>,
    S: ParsePart<'a>,
    M: ParseMsg<'a>,
{
    rem: &'a str,
    t: std::marker::PhantomData<T>,
    s: std::marker::PhantomData<S>,
    m: std::marker::PhantomData<M>,
}

impl<'a, T, S, M> Iterator for MessageIter<'a, T, S, M>
where
    T: ParsePart<'a>,
    S: ParsePart<'a>,
    M: ParseMsg<'a>,
{
    type Item = Message<'a, T, S, M>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = Message::parse_with_rem(self.rem);

        match res {
            Err(_) => None,
            Ok((rem, msg)) => {
                self.rem = rem;

                Some(msg)
            }
        }
    }
}

pub struct Originator<'a> {
    pub hostname: Option<&'a str>,
    pub app_name: Option<&'a str>,
    pub proc_id: Option<&'a str>,
    pub msg_id: Option<&'a str>,
}

pub trait ParseMsg<'a> {
    fn parse(msg: &'a str, foo: Originator<'a>) -> Result<(&'a str, Self), Error<'a>>
    where
        Self: Sized;
}
