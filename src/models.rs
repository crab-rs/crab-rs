use rust_utils::data::IData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Post{
    pub title: String,
    pub content: String,
    pub author_id: String,
    pub published: bool,
    pub created_at: i64,
    pub updated_at: i64,
}