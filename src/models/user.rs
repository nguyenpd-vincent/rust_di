use serde::{Deserialize, Serialize};

#[derive(Clone,Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
}

#[derive(Clone)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
}