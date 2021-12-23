/// https://developers.notion.com/docs/working-with-page-content
/// Pages are also blocks

use super::client::Client;
use anyhow::{anyhow, Result};
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub object: String,
    pub id: String,
    #[serde(rename = "type")]
    pub block_type: String,
    pub created_time: String,
    pub last_edited_time: String,
    pub archived: bool,
    pub has_children: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blocks {
    pub object: String,
    pub results: Vec<Block>,
    pub next_cursor: Option<String>,
    pub has_more: bool
}

impl Client {
    pub async fn get_block(&self, id: &str) -> Result<Block> {
        let path = format!("/block/{}", id);
        let res = self.build_request(Method::GET, path).send().await?;
        if res.status().eq(&StatusCode::OK) {
            Ok(res.json::<Block>().await?)
        } else {
            Err(anyhow!("failed to get block: {}", res.text().await?))
        }
    }

    pub async fn update_block(&self, id: String, block: Block) -> Result<()> {
        Ok(())
    }

    pub async fn delete_block(&self, id: &str) -> Result<()> {
        let path = format!("/block/{}", id);
        let res = self.build_request(Method::DELETE, path).send().await?;
        if res.status().eq(&StatusCode::OK) {
            Ok(())
        } else {
            Err(anyhow!("failed to delete block: {}", res.text().await?))
        }
    }

    pub async fn get_block_children(&self, id: &str, start_cursor: Option<String>, page_size: Option<u32>) -> Result<Blocks> {
        let path = format!("/block/{}/children", id);
        let mut params = Vec::new();
        if let Some(start_cursor) = start_cursor {
            params.push(("start_cursor", start_cursor));
        }
        if let Some(page_size) = page_size {
            params.push(("page_size", page_size.to_string()));
        }
        let res = self.build_request(Method::GET, path).query(&params).send().await?;
        if res.status().eq(&StatusCode::OK) {
            Ok(res.json::<Blocks>().await?)
        } else {
            Err(anyhow!("failed to get block children: {}", res.text().await?))
        }
    }
}