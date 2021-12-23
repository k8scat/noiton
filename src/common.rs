use serde::{Deserialize, Serialize};

/// https://developers.notion.com/reference/file-object
#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub url: String,
    pub expiry_time: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RichText {
    pub rich_text_type: String,
    pub href: Option<String>,
    pub plain_text: String,
    pub annotations: Annotations,
    pub text: Option<Text>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    pub content: String,
    pub link: Option<Link>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub link_type: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Icon {
    #[serde(rename = "type")]
    pub icon_type: String,
    pub file: Option<File>,
    pub emoji: Option<String>,
}
