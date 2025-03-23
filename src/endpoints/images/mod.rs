use crate::{Error, Image, ImageFingerprints, IncusClient, build_query};
use http_client_unix_domain_socket::Method;

impl IncusClient {
    pub async fn get_image_fingerprints(
        &mut self,
        project: Option<&str>,
        filter: Option<&str>,
        all_projects: Option<bool>,
    ) -> Result<ImageFingerprints, Error> {
        self.send_request_incus::<(), ImageFingerprints>(
            &format!(
                "/images{}",
                build_query!(project, filter, all_projects).replace("all_projects", "all-project")
            ),
            Method::GET,
            &[],
            None,
        )
        .await?
        .data()
    }

    pub async fn post_image(&mut self, image: &Image) -> Result<IncusResponseStatus, Error> {
        self.send_request_incus::<Certificate, serde_json::Value>(
            "/certificates",
            Method::POST,
            &[],
            Some(certificate),
        )
        .await?
        .status()
    }
}
