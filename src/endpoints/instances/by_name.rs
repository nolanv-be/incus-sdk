use crate::types::*;
use crate::{Error, IncusClient};
use http_client_unix_domain_socket::Method;

impl IncusClient {
    pub async fn get_instance_by_name(
        &mut self,
        name: &str,
    ) -> Result<IncusResponse<serde_json::Value>, Error> {
        self.send_request_incus::<(), IncusResponse<serde_json::Value>>(
            &format!("/instances/{name}"),
            Method::GET,
            &[],
            None,
        )
        .await
    }
}
