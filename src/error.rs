use crate::types::HttpError;

#[derive(Debug)]
pub enum Error {
    ClientUnix(http_client_unix_domain_socket::Error),
    HttpError(HttpError),
    MissingField(&'static str),
}

impl From<http_client_unix_domain_socket::Error> for Error {
    fn from(value: http_client_unix_domain_socket::Error) -> Self {
        Error::ClientUnix(value)
    }
}
