use std::collections::HashMap;
use std::hash::Hash;
use super::client::Client;
use super::common::File;
use anyhow::{anyhow, Result};
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::common::Icon;

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub object: String,
    pub id: String,
    pub created_time: String,
    pub last_edited_time: String,
    pub archived: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<File>,
    pub url: String,
    pub parent: Option<Parent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parent {
    #[serde(rename = "type")]
    pub parent_type: String,
    pub database_id: Option<String>,
    pub page_id: Option<String>,
    pub workspace: Option<bool>
}

impl Parent {
    pub fn new_database_parent(database_id: String) -> Self {
        Self {
            parent_type: String::from("database_id"),
            database_id: Some(database_id),
            page_id: None,
            workspace: None
        }
    }

    pub fn new_page_parent(page_id: String) -> Self {
        Self {
            parent_type: String::from("page_id"),
            database_id: None,
            page_id: Some(page_id),
            workspace: None
        }
    }

    pub fn new_workspace_parent() -> Self {
        Self {
            parent_type: String::from("workspace"),
            database_id: None,
            page_id: None,
            workspace: Some(true)
        }
    }
}

impl Client {
    /// Retrieve a page
    /// https://developers.notion.com/reference/retrieve-a-page
    pub async fn get_page(&self, id: &str) -> Result<Page> {
        let path = format!("/pages/{}", id);
        let res = self.build_request(Method::GET, path).send().await?;
        if res.status().eq(&StatusCode::OK) {
            Ok(res.json::<Page>().await?)
        } else {
            Err(anyhow!("failed to get page: {}", res.text().await?))
        }
    }

    pub async fn create_page(&self) -> Result<()> {
        Ok(())
    }

    /// Update page
    /// https://developers.notion.com/reference/patch-page
    pub async fn update_page(&self, id: &str, page: Page) -> Result<()> {
        let path = format!("/pages/{}", id);
        let res = self.build_request(Method::PATCH, path)
            .json(&page)
            .send()
            .await?;
        if res.status().eq(&StatusCode::OK) {
            Ok(())
        } else {
            Err(anyhow!("failed to update page: {}", res.text().await?))
        }
    }
}
