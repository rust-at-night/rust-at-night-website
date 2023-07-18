use serde::{Deserialize, Serialize};

/// A dummy post, just a placeholder for now.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    writer: String,
    content: String,
}

impl Post {
    /// Create a new post.
    pub fn new(writer: &str, content: &str) -> Self {
        Self {
            writer: writer.to_owned(),
            content: content.to_owned(),
        }
    }
}
