use crate::{Error, IncusClient, macros::build_query, types::*};
use http_client_unix_domain_socket::Method;

impl IncusClient {
    pub async fn get_certificate_fingerprints(
        &mut self,
        filter: Option<&str>,
    ) -> Result<CertificateFingerprints, Error> {
        Ok(self
            .send_request_incus::<()>(
                &format!("/certificates{}", build_query!(filter)),
                Method::GET,
                &[],
                None,
            )
            .await?
            .metadata()?
            .into())
    }

    pub async fn get_certificate(&mut self, fingerprint: &str) -> Result<Certificate, Error> {
        self.send_request_incus::<()>(
            &format!("/certificates/{fingerprint}"),
            Method::GET,
            &[],
            None,
        )
        .await?
        .metadata()?
        .try_into()
    }

    pub async fn post_certificate(
        &mut self,
        certificate: &Certificate,
    ) -> Result<IncusResponseStatus, Error> {
        self.send_request_incus::<Certificate>(
            "/certificates",
            Method::POST,
            &[],
            Some(certificate),
        )
        .await?
        .status()
    }

    pub async fn patch_certificate(
        &mut self,
        fingerprint: &str,
        certificate: &Certificate,
    ) -> Result<IncusResponseStatus, Error> {
        self.send_request_incus::<Certificate>(
            &format!("/certificates/{fingerprint}"),
            Method::PATCH,
            &[],
            Some(certificate),
        )
        .await?
        .status()
    }

    pub async fn put_certificate(
        &mut self,
        fingerprint: &str,
        certificate: &Certificate,
    ) -> Result<IncusResponseStatus, Error> {
        self.send_request_incus::<Certificate>(
            &format!("/certificates/{fingerprint}"),
            Method::PUT,
            &[],
            Some(certificate),
        )
        .await?
        .status()
    }

    pub async fn delete_certificate(
        &mut self,
        fingerprint: &str,
    ) -> Result<IncusResponseStatus, Error> {
        self.send_request_incus::<()>(
            &format!("/certificates/{fingerprint}"),
            Method::DELETE,
            &[],
            None,
        )
        .await?
        .status()
    }
}
