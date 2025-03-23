use crate::{Error, IncusClient, build_query, types::*};
use http_client_unix_domain_socket::Method;

impl IncusClient {
    pub async fn get_certificate_fingerprints(
        &mut self,
        filter: Option<&str>,
    ) -> Result<CertificateFingerprints, Error> {
        self.send_request_incus::<(), CertificateFingerprints>(
            &format!("/certificates{}", build_query!(filter)),
            Method::GET,
            &[],
            None,
        )
        .await?
        .data()
    }

    pub async fn get_certificate(
        &mut self,
        fingerprint: &str,
    ) -> Result<CertificateReturned, Error> {
        self.send_request_incus::<(), CertificateReturned>(
            &format!("/certificates/{fingerprint}"),
            Method::GET,
            &[],
            None,
        )
        .await?
        .data()
    }

    pub async fn post_certificate(
        &mut self,
        certificate: &Certificate,
    ) -> Result<IncusResponseStatus, Error> {
        self.send_request_incus::<Certificate, serde_json::Value>(
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
        self.send_request_incus::<Certificate, serde_json::Value>(
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
        self.send_request_incus::<Certificate, serde_json::Value>(
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
        self.send_request_incus::<(), serde_json::Value>(
            &format!("/certificates/{fingerprint}"),
            Method::DELETE,
            &[],
            None,
        )
        .await?
        .status()
    }
}
