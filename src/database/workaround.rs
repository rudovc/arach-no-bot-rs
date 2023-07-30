use color_eyre::Result;

use async_trait::async_trait;
use firebase_rs as firebase;

#[derive(Debug)]
pub struct Response {
    data: String,
}

// Workaround for the firebase-rs library not implementing functionality to perform a PUT request, nor exposing an interface to do it manually
#[async_trait]
pub trait WithPutRequest {
    async fn put<'a>(
        &self,
        data: &(impl serde::Serialize
              + serde::Deserialize<'a>
              + std::fmt::Debug
              + std::marker::Send
              + std::marker::Sync),
    ) -> Result<Response>;
    async fn put_request<'a>(
        &self,
        data: &(impl serde::Serialize
              + serde::Deserialize<'a>
              + std::fmt::Debug
              + std::marker::Send
              + std::marker::Sync),
    ) -> Result<Response>;
}

#[async_trait]
impl WithPutRequest for firebase::Firebase {
    async fn put<'a>(
        &self,
        data: &(impl serde::Serialize
              + serde::Deserialize<'a>
              + std::fmt::Debug
              + std::marker::Send
              + std::marker::Sync),
    ) -> Result<Response> {
        self.put_request(data).await
    }

    async fn put_request<'a>(
        &self,
        data: &(impl serde::Serialize + std::fmt::Debug + std::marker::Send + std::marker::Sync),
    ) -> Result<Response> {
        let client = reqwest::Client::new();
        let response: reqwest::Response = client.put(self.get_uri()).json(data).send().await?;
        let data = response.text().await.unwrap();

        Ok(Response { data })
    }
}
