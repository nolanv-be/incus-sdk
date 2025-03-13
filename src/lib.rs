mod endpoints;
mod error;
pub mod types;
pub mod utils;

pub use error::Error;
use http_client_unix_domain_socket::{ClientUnix, ErrorAndResponseJson, Method};
use serde::{Serialize, de::DeserializeOwned};
use types::HttpError;

pub const VERSION: &str = "1.0";
pub const SOCKET_PATH: &str = "/run/incus/unix.socket";
pub struct IncusClient {
    version: String,
    client: ClientUnix,
}

impl IncusClient {
    pub async fn try_default() -> Result<Self, Error> {
        Ok(IncusClient {
            version: VERSION.into(),
            client: ClientUnix::try_new(SOCKET_PATH).await?,
        })
    }

    async fn send_request_incus<IN: Serialize, OUT: DeserializeOwned>(
        &mut self,
        endpoint: &str,
        method: Method,
        headers: &[(&str, &str)],
        body_request: Option<&IN>,
    ) -> Result<OUT, Error> {
        let mut headers_concat = vec![("Host", "localhost")];
        for header in headers {
            headers_concat.push(header.clone());
        }

        match self
            .client
            .send_request_json::<IN, OUT, HttpError>(
                &format!("/{}/{}", self.version, endpoint),
                method,
                &headers_concat,
                body_request,
            )
            .await
        {
            Ok((_, response)) => Ok(response),
            Err(ErrorAndResponseJson::ResponseUnsuccessful(_, response)) => {
                Err(Error::HttpError(response))
            }
            Err(ErrorAndResponseJson::InternalError(e)) => Err(e.into()),
        }
    }
}

// pub async fn instance() -> Instance {
//     let mut client = ClientUnix::try_new("/run/incus/unix.socket")
//         .await
//         .expect("Try new");
//
//     let result = get_instance(&mut client, "rust").await;
//
//     dbg!(&result);
//
//     result.unwrap().1
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test]
//     async fn it_works() {
//         let instance = instance().await;
//         dbg!(&instance);
//         assert_eq!(instance.metadata.name, "rust");
//     }
// }
