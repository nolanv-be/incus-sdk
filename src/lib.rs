mod endpoints;
pub mod error;
pub mod types;
pub mod utils;

pub use error::Error;
use http_client_unix_domain_socket::{ClientUnix, ErrorAndResponseJson, Method};
use serde::Serialize;
use types::*;

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

    async fn send_request_incus<IN: Serialize, OUT: From<serde_json::Value>>(
        &mut self,
        endpoint: &str,
        method: Method,
        headers: &[(&str, &str)],
        body_request: Option<&IN>,
    ) -> Result<IncusResponse, Error> {
        let mut headers_concat = vec![("Host", "localhost")];
        for header in headers {
            headers_concat.push(header.clone());
        }

        match self
            .client
            .send_request_json::<IN, serde_json::Value, IncusResponseError>(
                &format!("/{}{}", self.version, endpoint),
                method,
                &headers_concat,
                body_request,
            )
            .await
        {
            Ok((_, response)) => Ok(response.into()),
            Err(ErrorAndResponseJson::ResponseUnsuccessful(_, response)) => {
                Err(Error::Http(response))
            }
            Err(ErrorAndResponseJson::InternalError(e)) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn get_supported_version() {
        let mut incus = IncusClient::try_default()
            .await
            .expect("IncusSdk::try_default");

        let version = incus
            .get_supported_versions()
            .await
            .expect("incus.get_supported_versions");

        assert_eq!(
            version.first().expect("first").version().expect("version"),
            "1.0"
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
            .expect("incus.get_server");

        assert_eq!(
            server.api_status().expect("server.api_status"),
            ApiStatus::Stable
        );
        assert_eq!(
            server.auth_methods().expect("server.auth_methods"),
            vec![AuthMethod::Tls]
        );

        assert_eq!(
            server
                .environment()
                .expect("server.environment")
                .storage_supported_drivers()
                .expect("storage_supported_drivers")
                .iter()
                .map(|s| s.name().expect("storage_supported_drivers.name"))
                .collect::<Vec<Storage>>(),
            vec![
                Storage::Btrfs,
                Storage::Dir,
                Storage::Lvm,
                Storage::Lvmcluster
            ]
        );

        assert_eq!(
            server
                .environment()
                .expect("server.environment")
                .architectures()
                .expect("environment.architecture"),
            vec![Architecture::X86_64, Architecture::I686]
        );

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
            .expect("incus.get_resources");

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

        assert_eq!(result, IncusResponseStatus::Success);

        let server = incus
            .get_server(None, None)
            .await
            .expect("incus.get_server");

        dbg!(&server);

        assert_eq!(
            server.config().expect("server.config").get("user.test"),
            Some(&time)
        );
    }

    #[tokio::test]
    async fn certificates() {
        let mut incus = IncusClient::try_default()
            .await
            .expect("IncusSdk::try_default");

        if let Some(fingerprint) = incus
            .get_certificate_fingerprints(None)
            .await
            .expect("incus.get_certificate_fingerprints")
            .fingerprints()
            .expect("certificates.fingerprints")
            .first()
        {
            dbg!(&fingerprint);
            incus
                .delete_certificate(&fingerprint)
                .await
                .expect("incus.delete_certificate");
        }

        incus
            .post_certificate(&certificate::post::Certificate {
                certificate: "MIIFkzCCA3ugAwIBAgIUVgqIkvkqAYqe2D1lfjt+Eg7f0qEwDQYJKoZIhvcNAQENBQAwWTELMAkGA1UEBhMCQkUxEDAOBgNVBAgMB0hhaW5hdXQxEDAOBgNVBAcMB1RvdXJuYWkxDzANBgNVBAoMBk5vbGFuVjEVMBMGA1UEAwwMbm9sYW52LmxvY2FsMB4XDTI1MDMxOTEwNTExNloXDTI2MDMxOTEwNTExNlowWTELMAkGA1UEBhMCQkUxEDAOBgNVBAgMB0hhaW5hdXQxEDAOBgNVBAcMB1RvdXJuYWkxDzANBgNVBAoMBk5vbGFuVjEVMBMGA1UEAwwMbm9sYW52LmxvY2FsMIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAyPJ05zb1ejzadZpC7gXrzulvqJgWPmnwVW2E3jlYmbcd1iHFBRF7M53ddhRbadk0Tszz4AIvEp2FFd3IrPzxhC0QWW5aUrpPBixm3l/wsVYpcBlNLgqi8cdjrl9nVMJqGbIouFP9kRBPP8mqsIJG292Ffv8MUc4OJO2Ffs1FYkBMcvWGCOGnqCxoCelpw/40d61yAkjKTo5UoooVocmsfw0C2tqHaTDt47P6w94vVUKKaqIB93LXsoN4dGIqXS7xX8KWqJwvRbSyI6YOnkVlyY5cJIIeC2AWUryUmcXauDm0ONt5ykbNPnztwTEcLl6YkIFjHRenwfoNFUBL9p8ByV/V/6VBEmFP9Ko11tB63tF5pStb6c/1onZyRpeyXH5NXFb3+VrXZH96RVR0M3nBFMEz3eZC6OXByvTb2JUuzVHmbth6rbKRnXsWvFt+mk0Zd0WcsQZIhjT55Q84KqfRqOgsAE+yH1VqCqejEnGQcYHjE4RdruVP9tcDwKDlqyQTxR3o4ChLfdkMSNFErbWUEajLN9lbuimYRcsovCmgS04RCKS0u6JIiQKFb7XTqqPeMx+hAgmS33tPtYSgaPu933dphhujOgQICfdtv2rLutP/Nf18iXsh27re82s08JwhDyBuckH4RoLv8zC6v1FZTC3r2sWT9OnT3cEvKXAdvQsCAwEAAaNTMFEwHQYDVR0OBBYEFBdFuOU6V+ckVdfpBtZ64RPFARxkMB8GA1UdIwQYMBaAFBdFuOU6V+ckVdfpBtZ64RPFARxkMA8GA1UdEwEB/wQFMAMBAf8wDQYJKoZIhvcNAQENBQADggIBAAVZs0+5um0IUiheRPp9RIPgVgZj2ExWuhLwLnOHOtF4gpzkCwgUM28P+IZQu91yH2W/xVVpt2yypft/LEHAaPIC84JVQku+vkSMxpqZGZXQcKazK+KzeJD3NsEzMUOJpD2xok9e8uzEEFp42UB2eeQfUvpi7UDbKpIG/W7F8a8qVV9IGj8GraaXW1k3CUbC5MIHSSuwk0SgGn9eDZ2v77AzgVYr7p8029FzQiH2jfIh1WxdfcEB5mqF7W8wXCIz69ubijcOeR/mHzCMXznGEMIoLRLw4muT1ZHqtRCNRVAdYbtCLIf7HbuHOE5QthKSpmMU5XXPJaloxuYmBl09UxCtDOB7KxoNcsqHBgJ1Cn6Kw2mGHE1uMJL3Qq+TPMU2HwvfFgG2kSQfnkS+l846s9gntXJoQ9nFFf3sSAdHOyNZZ0GOKbA/BgbexEPlF47qsemSh7fGxPlztnX8KDlj99hX/eMczvoBkAjeqaAt3wdXXgMJIyAQZeA0y5aJE49PvaomUblXtD+q4Hs81NaWTsGVsTqYY+Lm2TZm+sF3z3B+qTeKpTqwNf6jQ6pdf56ZOF8I9rt0JadGH0GeY00ZfBLw6o1Zx3ty3wk8reuk/pqzYnkTKDB3sh6sPr3TPGl+6EdD/B+nsKHNIDmI4eYVukUxG+zP2aZNBz3zGL19/C9p".into(),
                description: "this is a test".into(),
                name: "test".into(),
                projects: vec!["default".into()],
                restricted: true,
                token: false,
                trust_token: "".into(),
                certificate_type: CertificateType::Client,
            })
            .await
            .expect("incus.post_certificate");

        let fingerprints = incus
            .get_certificate_fingerprints(None)
            .await
            .expect("incus.get_certificate_fingerprints")
            .fingerprints()
            .expect("certificates.fingerprints");

        assert_eq!(
            fingerprints,
            vec!["19d41e4e88554147821a6ffeec95aad6d787f9abccf57c1e13faa2d7313c26df".to_string()]
        );
    }

    // #[tokio::test]
    // async fn get_instances() {
    //     let mut incus = IncusClient::try_default()
    //         .await
    //         .expect("IncusSdk::try_default");
    //
    //     let instances = incus
    //         .get_instances(None, None, None)
    //         .await
    //         .expect("incus.get_instances");
    //
    //     assert_eq!(
    //         instances,
    //         vec!["/1.0/instances/nodejs", "/1.0/instances/rust"]
    //     );
    // }
    //
    // #[tokio::test]
    // async fn get_instance_by_name() {
    //     let mut incus = IncusClient::try_default()
    //         .await
    //         .expect("IncusSdk::try_default");
    //
    //     let instance = incus
    //         .get_instance_by_name("rust")
    //         .await
    //         .expect("incus.get_instance_by_name")
    //         .metadata;
    //
    //     assert_eq!(instance.get("name").map(|n| n.as_str()), Some(Some("rust")));
    // }
}
