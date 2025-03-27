use crate::{Error, IncusClient, macros::build_query, types::*};
use http_client_unix_domain_socket::Method;

impl IncusClient {
    pub async fn get_image_fingerprints(
        &mut self,
        project: Option<&str>,
        filter: Option<&str>,
        all_projects: Option<bool>,
    ) -> Result<ImageFingerprints, Error> {
        Ok(self
            .send_request_incus::<()>(
                &format!(
                    "/images{}",
                    build_query!(project, filter, all_projects)
                        .replace("all_projects", "all-project")
                ),
                Method::GET,
                &[],
                None,
            )
            .await?
            .metadata()?
            .into())
    }

    // pub async fn post_image(&mut self, image: &Image) -> Result<IncusResponseStatus, Error> {
    //     self.send_request_incus::<Certificate, serde_json::Value>(
    //         "/certificates",
    //         Method::POST,
    //         &[],
    //         Some(certificate),
    //     )
    //     .await?
    //     .status()
    // }
}
