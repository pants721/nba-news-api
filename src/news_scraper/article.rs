use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, ToSchema)]
pub struct Article {
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub date: String,
    pub url: String,
    pub source: String,
}
