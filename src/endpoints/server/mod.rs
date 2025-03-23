// TODO get /1.0/events websocket ?
use crate::{Error, IncusClient, build_query, error::FieldError, types::*};
use http_client_unix_domain_socket::{ErrorAndResponseJson, Method};

impl IncusClient {
    pub async fn get_supported_versions(&mut self) -> Result<Vec<IncusVersion>, Error> {
        match self
            .client
            .send_request_json::<(), serde_json::Value, IncusResponseError>(
                "/",
                Method::GET,
                &vec![("Host", "localhost")],
                None,
            )
            .await
        {
            Ok((_, response)) => response
                .get("metadata")
                .ok_or_else(|| FieldError::Missing)?
                .as_array()
                .ok_or_else(|| FieldError::Invalid)?
                .iter()
                .map(|v| {
                    v.as_str()
                        .ok_or_else(|| FieldError::Invalid.into())
                        .map(|s| s.into())
                })
                .collect(),
            Err(ErrorAndResponseJson::ResponseUnsuccessful(_, response)) => {
                Err(Error::Http(response))
            }
            Err(ErrorAndResponseJson::InternalError(e)) => Err(e.into()),
        }
    }

    pub async fn get_server(
        &mut self,
        target: Option<&str>,
        project: Option<&str>,
    ) -> Result<Server, Error> {
        (&self
            .send_request_incus::<()>(
                &format!("{}", build_query!(target, project)),
                Method::GET,
                &[],
                None,
            )
            .await?)
            .try_into()
    }

    pub async fn patch_server(
        &mut self,
        target: Option<&str>,
        server: &Server,
    ) -> Result<IncusResponseStatus, Error> {
        self.send_request_incus::<Server>(
            &format!("{}", build_query!(target)),
            Method::PATCH,
            &[],
            Some(server),
        )
        .await?
        .status()
    }

    pub async fn put_server(
        &mut self,
        target: Option<&str>,
        server: &Server,
    ) -> Result<IncusResponseStatus, Error> {
        self.send_request_incus::<Server>(
            &format!("{}", build_query!(target)),
            Method::PUT,
            &[],
            Some(server),
        )
        .await?
        .status()
    }

    pub async fn get_resources(
        &mut self,
        target: Option<&str>,
    ) -> Result<serde_json::Value, Error> {
        self.send_request_incus::<()>(
            &format!("/resources{}", build_query!(target)),
            Method::GET,
            &[],
            None,
        )
        .await?
        .metadata()
        .map(|m| m.clone())
    }
}
