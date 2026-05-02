use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub done: bool,
    pub tag: Option<String>,
    pub created_at: String,
}

impl Todo{
    pub fn new(id: u32, title: String, tag: Option<String>) -> Self {
        Todo {
            id,
            title,
            done: false,
            tag,
            created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }

    pub fn matches_tag(&self, filter_tag: &Option<String>) -> bool {
        match filter_tag {
            None => true,
            Some(filter) => match &self.tag {
                None => false,
                Some(our_tag) => our_tag.to_lowercase() == filter.to_lowercase(),
            },
        }
    }
}