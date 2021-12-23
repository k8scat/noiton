use super::client::Client;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchParams {
    pub query: String,
    pub start_cursor: String,
    pub page_size: u32,
    pub sort: Option<SearchSort>,
    pub filter: Option<SearchFilter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchSort {
    pub direction: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchFilter {
    pub value: String,
    pub property: String,
}

impl Client {
    pub async fn search(&self, params: Option<SearchParams>) -> Result<()> {
        // let url = format!("{}/search", self.base_api);
        // let mut resp = self.client.post(url);
        // if params.is_some() {
        //     resp = resp.json(&params);
        // }
        // let resp = resp.send().await?;
        Ok(())
    }
}