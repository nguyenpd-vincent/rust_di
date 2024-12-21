use serde::{Deserialize, Serialize};

#[derive(Clone,Serialize, Deserialize, PartialEq, Eq)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
}
