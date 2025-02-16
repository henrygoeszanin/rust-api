use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct CreateTaskSchema {
    pub title: String,
    pub content: String,
}