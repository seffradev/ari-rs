use std::collections::HashMap;

use derive_getters::Getters;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use url::Url;

use crate::*;

#[derive(Debug, Getters)]
pub struct AriClient {
    url: Url,
    api_key: String,
    inner: reqwest::Client,
}

#[derive(Debug, Error)]
pub enum AriClientError {
    #[error("encountered inner HTTP client error")]
    Reqwest(#[from] reqwest::Error),
}

impl AriClient {
    pub(crate) fn new(url: Url, api_key: String) -> Self {
        Self {
            url,
            api_key,
            inner: reqwest::Client::new(),
        }
    }

    pub(crate) async fn authorized_get<T: Serialize, R: DeserializeOwned>(
        &self,
        path: impl AsRef<[&str]>,
        params: T,
    ) -> AriClientResult<R> {
        let url = self.authorized_url(path, params);
        let response = self.inner.get(url).send().await?.json().await?;
        Ok(response)
    }

    pub(crate) async fn authorized_post<T: Serialize>(&self, path: impl AsRef<[&str]>, params: T) -> AriClientResult<()> {
        let url = self.authorized_url(path, params);
        self.inner.post(url).send().await?;
        Ok(())
    }

    pub(crate) async fn authorized_post_json_response<T: Serialize, R: DeserializeOwned>(
        &self,
        path: impl AsRef<[&str]>,
        params: T,
    ) -> AriClientResult<R> {
        let url = self.authorized_url(path, params);
        let response = self.inner.post(url).send().await?.json().await?;
        Ok(response)
    }

    pub(crate) async fn authorized_post_variables<T: Serialize, R: DeserializeOwned>(
        &self,
        path: impl AsRef<[&str]>,
        params: T,
        variables: &HashMap<&str, &str>,
    ) -> AriClientResult<R> {
        let url = self.authorized_url(path, params);
        let response = self
            .inner
            .post(url)
            .json(&serde_json::json!({
                "variables": variables
            }))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub(crate) async fn authorized_delete<T: Serialize>(&self, path: impl AsRef<[&str]>, params: T) -> AriClientResult<()> {
        let url = self.authorized_url(path, params);
        self.inner.delete(url).send().await?;
        Ok(())
    }

    fn authorized_url<'a, T: Serialize>(&self, path: impl AsRef<[&'a str]>, params: T) -> Url {
        Authorization::build_url(&self.url, path, &self.api_key, params).expect("failed to create internally built url")
    }
}

impl Default for AriClient {
    fn default() -> Self {
        Self::new(
            "http://localhost:8088/".parse().expect("failed to parse url"),
            Authorization::api_key("asterisk", "asterisk"),
        )
    }
}
