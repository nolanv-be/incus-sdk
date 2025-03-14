use http_client_unix_domain_socket::Method;

use crate::types::Instance;
use crate::{Error, IncusClient};

impl IncusClient {
    pub async fn get_instance_by_name(&mut self, name: &str) -> Result<Instance, Error> {
        self.send_request_incus::<(), Instance>(
            &format!("instances/{name}"),
            Method::GET,
            &[],
            None,
        )
        .await
    }
}
