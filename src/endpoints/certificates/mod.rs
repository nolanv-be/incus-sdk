use crate::{Error, IncusClient, build_query};
use http_client_unix_domain_socket::Method;

impl IncusClient {
    pub async fn get_certificates(
        &mut self,
        filter: Option<&str>,
    ) -> Result<serde_json::Value, Error> {
        self.send_request_incus::<(), serde_json::Value>(
            &format!("/certificates{}", build_query!(filter)),
            Method::GET,
            &[],
            None,
        )
        .await?
        .data()
    }
}
