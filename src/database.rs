/// https://developers.notion.com/reference/database

use std::collections::HashMap;
use super::client::Client;
use anyhow::{anyhow, Result};
use reqwest::{Method, StatusCode};
use serde_json::Value;
use crate::common::{File, Icon, RichText};
use crate::page::{Page, Parent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub object: String,
    pub id: String,
    pub cover: Option<File>,
    pub icon: Option<Icon>,
    pub created_time: String,
    pub last_edited_time: String,
    pub title: Vec<RichText>,
    pub properties: HashMap<String, Value>,
    pub parent: Parent,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,

    /// https://developers.notion.com/reference/post-database-query#post-database-query-filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorts: Option<Vec<DatabaseSort>>,
}

/// https://developers.notion.com/reference/post-database-query#post-database-query-sort
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseSort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    /// created_time or last_edited_time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// ascending or descending
    pub direction: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pages {
    pub object: String,
    pub next_cursor: Option<String>,
    pub has_more: bool,
    pub results: Vec<Page>
}

impl Client {
    /// Query a database
    /// https://developers.notion.com/reference/post-database-query
    pub async fn query_database(&self, id: String, query: Option<DatabaseQuery>) -> Result<Pages> {
        let path = format!("/databases/{}/query", id);
        let mut req = self.build_request(Method::POST, path);
        if let Some(query) = query {
            req = req.json(&query);
        }
        let res = req.send().await?;
        if res.status().eq(&StatusCode::OK) {
            Ok(res.json::<Pages>().await?)
        } else {
            Err(anyhow!("failed to query database: {}", res.text().await?))
        }
    }

    pub async fn update_database(&self, id: String) -> Result<()> {
        Ok(())
    }

    pub async fn create_database(&self) -> Result<()> {
        Ok(())
    }

    /// Retrieve a database
    /// https://developers.notion.com/reference/retrieve-a-database
    pub async fn get_database(&self, id: String) -> Result<Database> {
        let path = format!("/databases/{}", id);
        let res = self.build_request(Method::GET, path).send().await?;
        if res.status().eq(&StatusCode::OK) {
            Ok(res.json::<Database>().await?)
        } else {
            Err(anyhow!("failed to get database: {}", res.text().await?))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::{json, Value};

    #[tokio::test]
    async fn query_database() {
        let client = crate::Client::setup().unwrap();
        let database_id = String::from("94ce3f36582747ce971eb0159873648f");
        let mut filter: HashMap<String, Value> = HashMap::new();
        filter.insert(String::from("and"), json!([
            json!({
                "property": "Name",
                "rich_text": {
                    "equals": "hello"
                }
            })
        ]));
        let query = crate::database::DatabaseQuery {
            start_cursor: None,
            page_size: None,
            filter: Some(filter),
            sorts: None,
        };
        let pages = client.query_database(database_id, Some(query)).await.unwrap();
        println!("{:#?}", pages);
    }
}