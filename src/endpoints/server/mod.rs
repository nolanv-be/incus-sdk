use crate::{
    Error, IncusClient,
    types::{HttpError, IncusStrings},
};
use http_client_unix_domain_socket::{ErrorAndResponseJson, Method};

impl IncusClient {
    pub async fn get_supported_version(&mut self) -> Result<IncusStrings, Error> {
        match self
            .client
            .send_request_json::<(), IncusStrings, HttpError>(
                "/",
                Method::GET,
                &vec![("Host", "localhost")],
                None,
            )
            .await
        {
            Ok((_, response)) => Ok(response),
            Err(ErrorAndResponseJson::ResponseUnsuccessful(_, response)) => {
                Err(Error::HttpError(response))
            }
            Err(ErrorAndResponseJson::InternalError(e)) => Err(e.into()),
        }
    }
}
