mod endpoints;
pub mod error;
pub mod macros;
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

    async fn send_request_incus<IN: Serialize>(
        &mut self,
        endpoint: &str,
        method: Method,
        headers: &[(&str, &str)],
        body_request: Option<&IN>,
    ) -> Result<IncusResponse, Error> {
        self.send_request_incus_raw(
            &format!("/{}{}", self.version, endpoint),
            method,
            headers,
            body_request,
        )
        .await
    }

    async fn send_request_incus_raw<IN: Serialize>(
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
            .send_request_json::<IN, serde_json::Value, IncusError>(
                endpoint,
                method,
                &headers_concat,
                body_request,
            )
            .await
        {
            Ok((_, response)) => response.try_into(),
            Err(ErrorAndResponseJson::ResponseUnsuccessful(_, response)) => {
                Err(Error::Http(response))
            }
            Err(ErrorAndResponseJson::InternalError(e)) => Err(e.into()),
        }
    }

    pub async fn abort_connection(self) -> Option<Error> {
        self.client.abort().await.map(|e| Error::ClientUnix(e))
    }
}

// TODO Temporary test while implementing
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn get_supported_version() -> Result<(), crate::Error> {
        let mut incus = IncusClient::try_default().await?;

        let supported_versions = incus.get_supported_versions().await?;

        assert_eq!(
            supported_versions
                .versions()?
                .first()
                .ok_or(crate::error::FieldError::Missing)?,
            &"1.0"
        );
        Ok(())
    }

    #[tokio::test]
    async fn get_server() -> Result<(), crate::Error> {
        let mut incus = IncusClient::try_default().await?;

        let server = incus.get_server(None, None).await?;

        assert_eq!(server.api_status()?, ApiStatus::Stable);
        assert_eq!(server.auth_methods()?, vec![AuthMethod::Tls]);

        assert_eq!(
            server
                .environment()?
                .storage_supported_drivers()?
                .iter()
                .map(|s| s.name())
                .collect::<Result<Vec<Storage>, crate::Error>>()?,
            vec![
                Storage::Btrfs,
                Storage::Dir,
                Storage::Lvm,
                Storage::Lvmcluster
            ]
        );

        assert_eq!(
            server.environment()?.architectures()?,
            vec![Architecture::X86_64, Architecture::I686]
        );

        assert_eq!(server.api_version()?, "1.0");
        Ok(())
    }

    #[tokio::test]
    async fn get_resources() -> Result<(), crate::Error> {
        let mut incus = IncusClient::try_default().await?;

        let resources = incus.get_resources(None).await?;

        assert_eq!(
            Architecture::try_from(
                resources
                    .get_json_wrapper_map("cpu")?
                    .get_str("architecture")?
            )?,
            Architecture::X86_64
        );
        Ok(())
    }

    #[tokio::test]
    async fn patch_server() -> Result<(), crate::Error> {
        let mut incus = IncusClient::try_default().await?;

        let time = format!("{:?}", std::time::Instant::now());
        let result = incus
            .patch_server(
                None,
                &Server::default()
                    .with_config(HashMap::from([("user.test".into(), time.clone())]))?,
            )
            .await?;

        assert_eq!(result, IncusResponseStatus::Success);

        let server = incus.get_server(None, None).await?;

        assert_eq!(server.config()?.get("user.test"), Some(&time));

        Ok(())
    }

    #[tokio::test]
    async fn crud_certificate() -> Result<(), crate::Error> {
        const FINGERPRINT: &str =
            "19d41e4e88554147821a6ffeec95aad6d787f9abccf57c1e13faa2d7313c26df";
        let original_certificate = Certificate::default()
            .with_certificate("MIIFkzCCA3ugAwIBAgIUVgqIkvkqAYqe2D1lfjt+Eg7f0qEwDQYJKoZIhvcNAQENBQAwWTELMAkGA1UEBhMCQkUxEDAOBgNVBAgMB0hhaW5hdXQxEDAOBgNVBAcMB1RvdXJuYWkxDzANBgNVBAoMBk5vbGFuVjEVMBMGA1UEAwwMbm9sYW52LmxvY2FsMB4XDTI1MDMxOTEwNTExNloXDTI2MDMxOTEwNTExNlowWTELMAkGA1UEBhMCQkUxEDAOBgNVBAgMB0hhaW5hdXQxEDAOBgNVBAcMB1RvdXJuYWkxDzANBgNVBAoMBk5vbGFuVjEVMBMGA1UEAwwMbm9sYW52LmxvY2FsMIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAyPJ05zb1ejzadZpC7gXrzulvqJgWPmnwVW2E3jlYmbcd1iHFBRF7M53ddhRbadk0Tszz4AIvEp2FFd3IrPzxhC0QWW5aUrpPBixm3l/wsVYpcBlNLgqi8cdjrl9nVMJqGbIouFP9kRBPP8mqsIJG292Ffv8MUc4OJO2Ffs1FYkBMcvWGCOGnqCxoCelpw/40d61yAkjKTo5UoooVocmsfw0C2tqHaTDt47P6w94vVUKKaqIB93LXsoN4dGIqXS7xX8KWqJwvRbSyI6YOnkVlyY5cJIIeC2AWUryUmcXauDm0ONt5ykbNPnztwTEcLl6YkIFjHRenwfoNFUBL9p8ByV/V/6VBEmFP9Ko11tB63tF5pStb6c/1onZyRpeyXH5NXFb3+VrXZH96RVR0M3nBFMEz3eZC6OXByvTb2JUuzVHmbth6rbKRnXsWvFt+mk0Zd0WcsQZIhjT55Q84KqfRqOgsAE+yH1VqCqejEnGQcYHjE4RdruVP9tcDwKDlqyQTxR3o4ChLfdkMSNFErbWUEajLN9lbuimYRcsovCmgS04RCKS0u6JIiQKFb7XTqqPeMx+hAgmS33tPtYSgaPu933dphhujOgQICfdtv2rLutP/Nf18iXsh27re82s08JwhDyBuckH4RoLv8zC6v1FZTC3r2sWT9OnT3cEvKXAdvQsCAwEAAaNTMFEwHQYDVR0OBBYEFBdFuOU6V+ckVdfpBtZ64RPFARxkMB8GA1UdIwQYMBaAFBdFuOU6V+ckVdfpBtZ64RPFARxkMA8GA1UdEwEB/wQFMAMBAf8wDQYJKoZIhvcNAQENBQADggIBAAVZs0+5um0IUiheRPp9RIPgVgZj2ExWuhLwLnOHOtF4gpzkCwgUM28P+IZQu91yH2W/xVVpt2yypft/LEHAaPIC84JVQku+vkSMxpqZGZXQcKazK+KzeJD3NsEzMUOJpD2xok9e8uzEEFp42UB2eeQfUvpi7UDbKpIG/W7F8a8qVV9IGj8GraaXW1k3CUbC5MIHSSuwk0SgGn9eDZ2v77AzgVYr7p8029FzQiH2jfIh1WxdfcEB5mqF7W8wXCIz69ubijcOeR/mHzCMXznGEMIoLRLw4muT1ZHqtRCNRVAdYbtCLIf7HbuHOE5QthKSpmMU5XXPJaloxuYmBl09UxCtDOB7KxoNcsqHBgJ1Cn6Kw2mGHE1uMJL3Qq+TPMU2HwvfFgG2kSQfnkS+l846s9gntXJoQ9nFFf3sSAdHOyNZZ0GOKbA/BgbexEPlF47qsemSh7fGxPlztnX8KDlj99hX/eMczvoBkAjeqaAt3wdXXgMJIyAQZeA0y5aJE49PvaomUblXtD+q4Hs81NaWTsGVsTqYY+Lm2TZm+sF3z3B+qTeKpTqwNf6jQ6pdf56ZOF8I9rt0JadGH0GeY00ZfBLw6o1Zx3ty3wk8reuk/pqzYnkTKDB3sh6sPr3TPGl+6EdD/B+nsKHNIDmI4eYVukUxG+zP2aZNBz3zGL19/C9p")?
            .with_name("test")?
            .with_description("this is a test")?
            .with_projects(vec!["default"])?
            .with_restricted(true)?
            .with_certificate_type(CertificateType::Client)?;

        let mut incus = IncusClient::try_default().await?;

        if let Some(fingerprint) = incus
            .get_certificate_fingerprints(None)
            .await?
            .fingerprints()?
            .first()
        {
            incus.delete_certificate(&fingerprint).await?;
        }

        incus
            .post_certificate(&original_certificate.clone())
            .await?;

        let certificate_fingerprints = incus.get_certificate_fingerprints(None).await?;

        assert_eq!(
            certificate_fingerprints.fingerprints()?,
            vec![FINGERPRINT.to_string()]
        );

        let certificate = incus.get_certificate(FINGERPRINT).await?;

        assert_eq!(certificate.name()?, "test".to_string());

        incus
            .patch_certificate(&FINGERPRINT, &Certificate::default().with_name("test2")?)
            .await
            .expect("incus.patch_certificate");

        let certificate = incus.get_certificate(FINGERPRINT).await?;

        assert_eq!(certificate.name()?, "test2".to_string());

        incus
        .put_certificate(
            &FINGERPRINT,
            &original_certificate
                .with_certificate("-----BEGIN CERTIFICATE-----\nMIIFkzCCA3ugAwIBAgIUVgqIkvkqAYqe2D1lfjt+Eg7f0qEwDQYJKoZIhvcNAQENBQAwWTELMAkGA1UEBhMCQkUxEDAOBgNVBAgMB0hhaW5hdXQxEDAOBgNVBAcMB1RvdXJuYWkxDzANBgNVBAoMBk5vbGFuVjEVMBMGA1UEAwwMbm9sYW52LmxvY2FsMB4XDTI1MDMxOTEwNTExNloXDTI2MDMxOTEwNTExNlowWTELMAkGA1UEBhMCQkUxEDAOBgNVBAgMB0hhaW5hdXQxEDAOBgNVBAcMB1RvdXJuYWkxDzANBgNVBAoMBk5vbGFuVjEVMBMGA1UEAwwMbm9sYW52LmxvY2FsMIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAyPJ05zb1ejzadZpC7gXrzulvqJgWPmnwVW2E3jlYmbcd1iHFBRF7M53ddhRbadk0Tszz4AIvEp2FFd3IrPzxhC0QWW5aUrpPBixm3l/wsVYpcBlNLgqi8cdjrl9nVMJqGbIouFP9kRBPP8mqsIJG292Ffv8MUc4OJO2Ffs1FYkBMcvWGCOGnqCxoCelpw/40d61yAkjKTo5UoooVocmsfw0C2tqHaTDt47P6w94vVUKKaqIB93LXsoN4dGIqXS7xX8KWqJwvRbSyI6YOnkVlyY5cJIIeC2AWUryUmcXauDm0ONt5ykbNPnztwTEcLl6YkIFjHRenwfoNFUBL9p8ByV/V/6VBEmFP9Ko11tB63tF5pStb6c/1onZyRpeyXH5NXFb3+VrXZH96RVR0M3nBFMEz3eZC6OXByvTb2JUuzVHmbth6rbKRnXsWvFt+mk0Zd0WcsQZIhjT55Q84KqfRqOgsAE+yH1VqCqejEnGQcYHjE4RdruVP9tcDwKDlqyQTxR3o4ChLfdkMSNFErbWUEajLN9lbuimYRcsovCmgS04RCKS0u6JIiQKFb7XTqqPeMx+hAgmS33tPtYSgaPu933dphhujOgQICfdtv2rLutP/Nf18iXsh27re82s08JwhDyBuckH4RoLv8zC6v1FZTC3r2sWT9OnT3cEvKXAdvQsCAwEAAaNTMFEwHQYDVR0OBBYEFBdFuOU6V+ckVdfpBtZ64RPFARxkMB8GA1UdIwQYMBaAFBdFuOU6V+ckVdfpBtZ64RPFARxkMA8GA1UdEwEB/wQFMAMBAf8wDQYJKoZIhvcNAQENBQADggIBAAVZs0+5um0IUiheRPp9RIPgVgZj2ExWuhLwLnOHOtF4gpzkCwgUM28P+IZQu91yH2W/xVVpt2yypft/LEHAaPIC84JVQku+vkSMxpqZGZXQcKazK+KzeJD3NsEzMUOJpD2xok9e8uzEEFp42UB2eeQfUvpi7UDbKpIG/W7F8a8qVV9IGj8GraaXW1k3CUbC5MIHSSuwk0SgGn9eDZ2v77AzgVYr7p8029FzQiH2jfIh1WxdfcEB5mqF7W8wXCIz69ubijcOeR/mHzCMXznGEMIoLRLw4muT1ZHqtRCNRVAdYbtCLIf7HbuHOE5QthKSpmMU5XXPJaloxuYmBl09UxCtDOB7KxoNcsqHBgJ1Cn6Kw2mGHE1uMJL3Qq+TPMU2HwvfFgG2kSQfnkS+l846s9gntXJoQ9nFFf3sSAdHOyNZZ0GOKbA/BgbexEPlF47qsemSh7fGxPlztnX8KDlj99hX/eMczvoBkAjeqaAt3wdXXgMJIyAQZeA0y5aJE49PvaomUblXtD+q4Hs81NaWTsGVsTqYY+Lm2TZm+sF3z3B+qTeKpTqwNf6jQ6pdf56ZOF8I9rt0JadGH0GeY00ZfBLw6o1Zx3ty3wk8reuk/pqzYnkTKDB3sh6sPr3TPGl+6EdD/B+nsKHNIDmI4eYVukUxG+zP2aZNBz3zGL19/C9p\n-----END CERTIFICATE-----")?
        ).await?;

        let certificate = incus.get_certificate(FINGERPRINT).await?;

        assert_eq!(certificate.name()?, "test".to_string());

        Ok(())
    }

    #[tokio::test]
    async fn crud_images() -> Result<(), crate::Error> {
        let mut incus = IncusClient::try_default().await?;

        let image_fingerprints = incus.get_image_fingerprints(None, None, None).await?;

        assert_eq!(image_fingerprints.fingerprints()?, Vec::<&str>::new());
        Ok(())
    }
}
