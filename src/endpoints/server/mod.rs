use crate::{Error, IncusClient, types::*};
use http_client_unix_domain_socket::{ErrorAndResponseJson, Method};

impl IncusClient {
    pub async fn get_supported_version(&mut self) -> Result<IncusResponse<String>, Error> {
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
                let incus_version = response.metadata.first().cloned().ok_or_else(|| {
                    Error::ClientUnix(http_client_unix_domain_socket::Error::ResponseParsing(
                        serde::de::Error::invalid_length(response.metadata.len(), &"1"),
                    ))
                })?;

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

    pub async fn get_server(
        &mut self,
        target: Option<&str>,
        project: Option<&str>,
    ) -> Result<IncusResponse<serde_json::Value>, Error> {
        let mut queries = Vec::new();
        if let Some(target) = target {
            queries.push(format!("target={target}"));
        }
        if let Some(project) = project {
            queries.push(format!("project={project}"));
        }

        let query_string = if !queries.is_empty() {
            format!("?{}", queries.join("&"))
        } else {
            "".into()
        };

        self.send_request_incus::<(), IncusResponse<serde_json::Value>>(
            &format!("{query_string}"),
            Method::GET,
            &[],
            None,
        )
        .await
    }
}
