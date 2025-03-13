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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_instance_by_name() {
        let mut incus = IncusClient::try_default()
            .await
            .expect("IncusSdk::try_default");
        let instance = incus
            .get_instance_by_name("rust")
            .await
            .expect("incus.get_instance_by_name");
        assert_eq!(instance.metadata.name, "rust");
    }
}
