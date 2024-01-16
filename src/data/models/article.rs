// src/models.rs
use chrono::DateTime;
use serde::{Deserialize, Serialize};

// Type aliases for clarity
pub type AuthorId = u16;
pub type EditorId = u16;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Article {
  pub id: u32,
  pub title: String,
  pub content: String,
  pub category_id: usize,
  pub author_id: AuthorId,
  pub version: u8,
  pub approval_state: ApprovalState,
  pub approved_by: Option<EditorId>,
  pub publication_date: Option<DateTime<chrono::Utc>>,
  pub tags: Vec<String>,
  pub is_featured: bool,
  pub views: usize,
  pub comments: Vec<Comment>,
  pub image_url: Option<String>,
  pub meta_description: Option<String>,
  pub meta_keywords: Option<Vec<String>>,
  pub likes: usize,
}

/// Display the book using the format "{title} by {author}".
/// This is a typical Rust trait and is not axum-specific.
impl std::fmt::Display for Article {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "article with title:{} \n content: {}",
      self.title, self.content
    )
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub enum ApprovalState {
  Draft,
  RequestApproval,
  ApprovalPending,
  Approved,
  // Add other states as needed
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Comment {
  pub id: usize,
  pub user_id: AuthorId,
  pub content: String,
  pub created_at: DateTime<chrono::Utc>,
  // Add other comment fields as needed
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Category {
  pub id: usize,
  pub name: String,
  pub description: Option<String>,
  pub parent_id: Option<usize>,
  pub is_featured: bool,
}
