pub mod access;
/// Bring Book struct into scope
pub mod models;

use self::models::{
  article::{ApprovalState, Article, Category, Comment},
  book::Book,
};

/// use chrono for date time
use chrono::Utc;

/// Use once_cell for creating a global variable e.g. our DATA data.
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Use Mutex for thread-safe access to a variable e.g. our DATA data.
use std::sync::Mutex;

/// Create a data store as a global variable with `Lazy` and `Mutex`.
/// This demo implementation uses a `HashMap` for ease and speed.
/// The map key is a primary key for lookup; the map value is a Book.
pub static DATA: Lazy<Mutex<HashMap<u32, Book>>> = Lazy::new(|| {
  Mutex::new(HashMap::from([
    (
      1,
      Book {
        id: 1,
        title: "Antigone".into(),
        author: "Sophocles".into(),
      },
    ),
    (
      2,
      Book {
        id: 2,
        title: "Beloved".into(),
        author: "Toni Morrison".into(),
      },
    ),
    (
      3,
      Book {
        id: 3,
        title: "Candide".into(),
        author: "Voltaire".into(),
      },
    ),
  ]))
});

pub static ARTICLE_DATA: Lazy<Mutex<HashMap<u32, Article>>> = Lazy::new(|| {
  Mutex::new(HashMap::from([
    (
      1,
      Article {
        id: 1,
        title: String::from("Introduction to Rust Programming"),
        content: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit."),
        category_id: 1,
        author_id: 1001,
        version: 1,
        approval_state: ApprovalState::Approved,
        approved_by: Some(2001),
        publication_date: Some(Utc::now()),
        tags: vec![
          "Rust".to_string(),
          "Programming".to_string(),
          "Introduction".to_string(),
        ],
        is_featured: true,
        views: 1500,
        comments: vec![Comment {
          id: 1,
          user_id: 1002,
          content: String::from("Great article!"),
          created_at: Utc::now(),
        }],
        image_url: Some(String::from("https://example.com/rust.jpg")),
        meta_description: Some(String::from(
          "Learn the basics of Rust programming language.",
        )),
        meta_keywords: Some(vec![
          "Rust".to_string(),
          "Programming".to_string(),
          "Beginner".to_string(),
        ]),
        likes: 120,
      },
    ),
    (
      2,
      Article {
        id: 2,
        title: String::from("Breaking News: Advances in Quantum Computing"),
        content: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit."),
        category_id: 1,
        author_id: 1003,
        version: 1,
        approval_state: ApprovalState::Approved,
        approved_by: Some(2002),
        publication_date: Some(Utc::now()),
        tags: vec![
          "Quantum Computing".to_string(),
          "Technology".to_string(),
          "Science".to_string(),
        ],
        is_featured: false,
        views: 800,
        comments: vec![Comment {
          id: 2,
          user_id: 1004,
          content: String::from("Exciting times for technology!"),
          created_at: Utc::now(),
        }],
        image_url: Some(String::from("https://example.com/quantum.jpg")),
        meta_description: Some(String::from(
          "Discover the latest breakthroughs in quantum computing.",
        )),
        meta_keywords: Some(vec![
          "Quantum".to_string(),
          "Technology".to_string(),
          "Science".to_string(),
        ]),
        likes: 90,
      },
    ),
    (
      3,
      Article {
        id: 3,
        title: String::from("The Future of Artificial Intelligence"),
        content: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit."),
        category_id: 1,
        author_id: 1005,
        version: 1,
        approval_state: ApprovalState::Approved,
        approved_by: Some(2003),
        publication_date: Some(Utc::now()),
        tags: vec![
          "Artificial Intelligence".to_string(),
          "Technology".to_string(),
          "Future".to_string(),
        ],
        is_featured: true,
        views: 1200,
        comments: vec![Comment {
          id: 3,
          user_id: 1006,
          content: String::from("Excited to see what the future holds!"),
          created_at: Utc::now(),
        }],
        image_url: Some(String::from("https://example.com/ai.jpg")),
        meta_description: Some(String::from(
          "Exploring the potential and future of artificial intelligence.",
        )),
        meta_keywords: Some(vec![
          "AI".to_string(),
          "Technology".to_string(),
          "Future".to_string(),
        ]),
        likes: 150,
      },
    ),
  ]))
});

// Dummy category
pub static _DUMMY_CATEGORY: Lazy<HashMap<u32, Category>> = Lazy::new(|| {
  HashMap::from([(
    1,
    Category {
      id: 1,
      name: String::from("Tech"),
      description: Some(String::from("Technology-related news and updates")),
      parent_id: None,
      is_featured: true,
    },
  )])
});
