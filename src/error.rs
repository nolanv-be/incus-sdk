use crate::types::*;

#[derive(Debug)]
pub enum Error {
    ClientUnix(http_client_unix_domain_socket::Error),
    Http(IncusResponseError),
    Field(FieldError),
}

impl From<http_client_unix_domain_socket::Error> for Error {
    fn from(value: http_client_unix_domain_socket::Error) -> Self {
        Error::ClientUnix(value)
    }
}
impl From<IncusResponseError> for Error {
    fn from(e: IncusResponseError) -> Self {
        Error::Http(e)
    }
}

#[derive(Debug)]
pub enum FieldError {
    Missing,
    Invalid,
    Unknown,
}
impl From<FieldError> for Error {
    fn from(e: FieldError) -> Self {
        Error::Field(e)
    }
}
