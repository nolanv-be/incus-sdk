mod endpoints;
mod error;
pub mod types;
pub mod utils;

pub use error::Error;
use http_client_unix_domain_socket::{ClientUnix, ErrorAndResponseJson, Method};
use serde::{Serialize, de::DeserializeOwned};
use types::HttpError;

pub const VERSION: &str = "1.0";
pub const SOCKET_PATH: &str = "/run/incus/unix.socket";
pub struct IncusClient {
    version: String,
    client: ClientUnix,
}

impl IncusClient {
    pub async fn try_default() -> Result<Self, Error> {
        Ok(IncusClient {
            version: VERSION.into(),
            client: ClientUnix::try_new(SOCKET_PATH).await?,
        })
    }

    async fn send_request_incus<IN: Serialize, OUT: DeserializeOwned>(
        &mut self,
        endpoint: &str,
        method: Method,
        headers: &[(&str, &str)],
        body_request: Option<&IN>,
    ) -> Result<OUT, Error> {
        let mut headers_concat = vec![("Host", "localhost")];
        for header in headers {
            headers_concat.push(header.clone());
        }

        match self
            .client
            .send_request_json::<IN, OUT, HttpError>(
                &format!("/{}{}", self.version, endpoint),
                method,
                &headers_concat,
                body_request,
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use types::ApiStatus;

    use super::*;

    #[tokio::test]
    async fn get_supported_version() {
        let mut incus = IncusClient::try_default()
            .await
            .expect("IncusSdk::try_default");

        let version = incus
            .get_supported_versions()
            .await
            .expect("incus.get_supported_versions")
            .metadata;

        assert_eq!(
            version.first().map(|v| v.version()),
            Some(Some("1.0".into()))
        );
    }

    #[tokio::test]
    async fn get_server() {
        let mut incus = IncusClient::try_default()
            .await
            .expect("IncusSdk::try_default");

        let server = incus
            .get_server(None, None)
            .await
            .expect("incus.get_server")
            .metadata;

        assert_eq!(server.api_status(), Some(ApiStatus::Stable));

        assert_eq!(
            server.inner().get("api_version").map(|v| v.as_str()),
            Some(Some("1.0"))
        );
    }

    #[tokio::test]
    async fn get_resources() {
        let mut incus = IncusClient::try_default()
            .await
            .expect("IncusSdk::try_default");

        let resources = incus
            .get_resources(None)
            .await
            .expect("incus.get_resources")
            .metadata;

        assert_eq!(
            resources
                .get("cpu")
                .map(|cpu| cpu.get("architecture").map(|arch| arch.as_str())),
            Some(Some(Some("x86_64")))
        );
    }

    #[tokio::test]
    async fn patch_server() {
        let mut incus = IncusClient::try_default()
            .await
            .expect("IncusSdk::try_default");

        let time = format!("{:?}", std::time::Instant::now());
        let result = incus
            .patch_server(None, &HashMap::from([("user.test".into(), time.clone())]))
            .await
            .expect("incus.patch_server");

        dbg!(&result);

        assert_eq!(result.status_code, 200);

        let server = incus
            .get_server(None, None)
            .await
            .expect("incus.get_server")
            .metadata;

        dbg!(&server);

        assert_eq!(
            server
                .inner()
                .get("config")
                .map(|c| c.get("user.test").map(|t| t.as_str())),
            Some(Some(Some(time.as_str())))
        );
    }

    #[tokio::test]
    async fn get_instances() {
        let mut incus = IncusClient::try_default()
            .await
            .expect("IncusSdk::try_default");

        let instances = incus
            .get_instances(None, None, None)
            .await
            .expect("incus.get_instances")
            .metadata;

        assert_eq!(
            instances,
            vec!["/1.0/instances/nodejs", "/1.0/instances/rust"]
        );
    }

    #[tokio::test]
    async fn get_instance_by_name() {
        let mut incus = IncusClient::try_default()
            .await
            .expect("IncusSdk::try_default");

        let instance = incus
            .get_instance_by_name("rust")
            .await
            .expect("incus.get_instance_by_name")
            .metadata;

        assert_eq!(instance.get("name").map(|n| n.as_str()), Some(Some("rust")));
    }
}
