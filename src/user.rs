use super::client::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest::Method;

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    object: String,
    results: Vec<User>,
    next_cursor: Option<String>,
    has_more: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub object: String,
    pub id: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    #[serde(rename = "type")]
    pub user_type: Option<String>,
    pub person: Option<Person>,
    pub bot: Option<Bot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bot {
    pub owner: Option<Owner>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "type")]
    pub owner_type: String,
    pub workspace: Option<bool>,
    pub user: Box<Option<User>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub email: String
}

impl Client {
    pub async fn list_users(&self) -> Result<Users> {
        let path = "/users";
        let res = self.build_request(Method::GET, path).send().await?;
        Ok(res.json::<Users>().await?)
    }

    pub async fn get_user(&self, id: &str) -> Result<User> {
        let path = format!("/users/{}", id);
        let res = self.build_request(Method::GET, path).send().await?;
        Ok(res.json::<User>().await?)
    }

    pub async fn me(&self) -> Result<User> {
        self.get_user("me").await
    }
}
