use crate::{Error, IncusClient, build_query, types::*};
use http_client_unix_domain_socket::Method;
use std::collections::HashMap;

impl IncusClient {
    pub async fn get_certificates(
        &mut self,
        filter: Option<&str>,
    ) -> Result<IncusResponse<Vec<String>>, Error> {
        self.send_request_incus::<(), IncusResponse<Vec<String>>>(
            &format!("/certificates{}", build_query!(filter)),
            Method::GET,
            &[],
            None,
        )
        .await
    }
}
