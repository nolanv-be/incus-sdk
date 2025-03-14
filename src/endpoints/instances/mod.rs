pub mod by_name;

use crate::{Error, IncusClient, types::IncusStrings};
use http_client_unix_domain_socket::Method;

impl IncusClient {
    pub async fn get_instances(
        &mut self,
        project: Option<&str>,
        filter: Option<&str>,
        is_all_projects: Option<bool>,
    ) -> Result<IncusStrings, Error> {
        let mut queries = Vec::new();
        if let Some(project) = project {
            queries.push(format!("project={project}"));
        }
        if let Some(filter) = filter {
            queries.push(format!("filter={filter}"));
        }
        if let Some(is_all_projects) = is_all_projects {
            queries.push(format!("all-projects={is_all_projects}"));
        }

        let query_string = if !queries.is_empty() {
            format!("?{}", queries.join("&"))
        } else {
            "".into()
        };

        self.send_request_incus::<(), IncusStrings>(
            &format!("instances{query_string}"),
            Method::GET,
            &[],
            None,
        )
        .await
    }
}
