use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Article {
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub date: String,
    pub url: String,
    pub origin: String,
}
