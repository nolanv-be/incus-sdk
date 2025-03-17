// TODO get /1.0/events websocket ?
use crate::{Error, IncusClient, build_query, types::*};
use http_client_unix_domain_socket::{ErrorAndResponseJson, Method};
use std::collections::HashMap;

impl IncusClient {
    pub async fn get_supported_versions(
        &mut self,
    ) -> Result<IncusResponse<Vec<IncusVersion>>, Error> {
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
            Ok((_, response)) => Ok(IncusResponse {
                status: response.status,
                status_code: response.status_code,
                r#type: response.r#type,
                metadata: response
                    .metadata
                    .iter()
                    .map(|v| v.as_str().into())
                    .collect(),
            }),
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
    ) -> Result<IncusResponse<Server>, Error> {
        self.send_request_incus::<(), IncusResponse<Server>>(
            &format!("{}", build_query!(target, project)),
            Method::GET,
            &[],
            None,
        )
        .await
    }

    pub async fn patch_server(
        &mut self,
        target: Option<&str>,
        config: &HashMap<String, String>,
    ) -> Result<IncusEmptyResponse, Error> {
        self.send_request_incus::<HashMap<String, &HashMap<String, String>>, IncusEmptyResponse>(
            &format!("{}", build_query!(target)),
            Method::PATCH,
            &[],
            Some(&HashMap::from([("config".into(), config)])),
        )
        .await
    }

    pub async fn put_server(
        &mut self,
        target: Option<&str>,
        config: &HashMap<String, String>,
    ) -> Result<IncusEmptyResponse, Error> {
        self.send_request_incus::<HashMap<String, &HashMap<String, String>>, IncusEmptyResponse>(
            &format!("{}", build_query!(target)),
            Method::PUT,
            &[],
            Some(&HashMap::from([("config".into(), config)])),
        )
        .await
    }

    pub async fn get_resources(
        &mut self,
        target: Option<&str>,
    ) -> Result<IncusResponse<serde_json::Value>, Error> {
        self.send_request_incus::<(), IncusResponse<serde_json::Value>>(
            &format!("/resources{}", build_query!(target)),
            Method::GET,
            &[],
            None,
        )
        .await
    }
}
