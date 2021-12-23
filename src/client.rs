use std::str::FromStr;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use anyhow::{Result, anyhow};
use reqwest::{IntoUrl, Method, RequestBuilder};

pub const BASE_API: &str = "https://api.notion.com/v1";
pub const VERSION: &str = "2021-08-16";

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    base_api: String
}

impl Client {
    pub fn new(token: &str, base_api: Option<String>, version: Option<String>) -> Result<Client> {
        if token.is_empty() {
            return Err(anyhow!("invalid token"));
        }
        let base_api = base_api.unwrap_or_else(|| BASE_API.to_string());
        let version = version.unwrap_or_else(|| VERSION.to_string());
        let headers = HeaderMap::from_iter(vec![
            (HeaderName::from_str("Authorization")?, HeaderValue::from_str(format!("Bearer {}", token).as_str())?),
            (HeaderName::from_str("Notion-Version")?, HeaderValue::from_str(version.as_str())?),
        ]);
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;
        Ok(Client {
            client,
            base_api
        })
    }

    pub fn setup() -> Result<Client> {
        let token = std::env::var("NOTION_TOKEN")?;
        Client::new(token.as_str(), None, None)
    }

    pub fn build_request<U: IntoUrl>(&self, method: Method, path: U) -> RequestBuilder {
        let url = format!("{}{}", self.base_api, path.as_str());
        self.client.request(method, url.as_str())
    }
}
