use std::convert::From;
use std::error::Error as StdError;
use std::fmt;
use url::ParseError;

pub mod application;
pub mod authorization;
pub mod contact;
pub mod crypto;
pub mod email;
pub mod group;
pub mod mailer;
pub mod profile;
pub mod ratelimit;
pub mod role;
pub mod summary;
pub mod ticket;
pub mod token;
pub mod user;
pub mod username;

use self::crypto::Error as CryptoError;
use self::mailer::MailerError;
use self::username::Error as UsernameError;
use postgres::error::Error as PgError;

#[derive(Debug)]
pub enum Error {
    Request(Box<StdError>),
    InvalidParam(String, Box<StdError>),
    Database(PgError),
    Mailer(MailerError),
    NotFound,
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Request(ref err) => err.fmt(f),
            Error::InvalidParam(ref _field, ref err) => err.fmt(f),
            Error::Database(ref err) => err.fmt(f),
            Error::Mailer(ref err) => err.fmt(f),
            Error::NotFound => write!(f, "Resource not found"),
            Error::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Request(ref err) => err.description(),
            Error::InvalidParam(ref _field, ref err) => err.description(),
            Error::Database(ref err) => err.description(),
            Error::Mailer(ref err) => err.description(),
            Error::NotFound => "Resource not found",
            Error::Unknown => "Unknown",
        }
    }
}

impl From<PgError> for Error {
    fn from(err: PgError) -> Error {
        Error::Database(err)
    }
}

impl From<UsernameError> for Error {
    fn from(err: UsernameError) -> Error {
        Error::InvalidParam(String::from("username"), Box::new(err))
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::InvalidParam(String::from("url"), Box::new(err))
    }
}

impl From<CryptoError> for Error {
    fn from(err: CryptoError) -> Error {
        Error::InvalidParam(String::from("password"), Box::new(err))
    }
}

impl From<MailerError> for Error {
    fn from(err: MailerError) -> Error {
        Error::Mailer(err)
    }
}
