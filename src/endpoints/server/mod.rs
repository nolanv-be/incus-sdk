use crate::{Error, IncusClient, types::*};
use http_client_unix_domain_socket::{ErrorAndResponseJson, Method};

impl IncusClient {
    pub async fn get_supported_version(&mut self) -> Result<IncusResponse<IncusVersion>, Error> {
        match self
            .client
            .send_request_json::<(), IncusResponse<Vec<String>>, HttpError>(
                "/",
                Method::GET,
                &vec![("Host", "localhost")],
                None,
            )
            .await
        {
            Ok((_, response)) => {
                let incus_version_path = response.metadata.first().ok_or_else(|| {
                    Error::ClientUnix(http_client_unix_domain_socket::Error::ResponseParsing(
                        serde::de::Error::invalid_length(response.metadata.len(), &"1"),
                    ))
                })?;

                let incus_version = IncusVersion(
                    incus_version_path
                        .split_once("/")
                        .ok_or_else(|| {
                            Error::ClientUnix(
                                http_client_unix_domain_socket::Error::ResponseParsing(
                                    serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(incus_version_path),
                                        &"/1.0",
                                    ),
                                ),
                            )
                        })?
                        .1
                        .into(),
                );

                Ok(IncusResponse {
                    status: response.status,
                    status_code: response.status_code,
                    r#type: response.r#type,
                    metadata: incus_version,
                })
            }
            Err(ErrorAndResponseJson::ResponseUnsuccessful(_, response)) => {
                Err(Error::HttpError(response))
            }
            Err(ErrorAndResponseJson::InternalError(e)) => Err(e.into()),
        }
    }
}
