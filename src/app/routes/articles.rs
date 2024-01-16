use axum::{extract, response};
use std::thread;

use crate::data::{models::article::Article, articles::ARTICLE_DATA};

/// axum handler for "GET /articles" which responds with a resource page.
/// This demo uses our ARTICLE_DATA; a production app could use a database.
/// This demo must clone the ARTICLE_DATA in order to sort items by title.
pub async fn get() -> response::Html<String> {
  thread::spawn(move || {
    let data = ARTICLE_DATA.lock().unwrap();
    let mut articles = data.values().collect::<Vec<_>>().clone();
    articles.sort_by(|a, b| a.title.cmp(&b.title));
    articles
      .iter()
      .map(|&article| format!("<p>{}</p>\n", &article))
      .collect::<String>()
  })
  .join()
  .unwrap()
  .into()
}

/// axum handler for "GET /articles/:id" which responds with one resource HTML page.
/// This demo app uses our ARTICLE_DATA variable, and iterates on it to find the id.
pub async fn get_by_id(extract::Path(id): extract::Path<u32>) -> response::Html<String> {
  thread::spawn(move || {
    let data = ARTICLE_DATA.lock().unwrap();
    match data.get(&id) {
      Some(article) => format!("<p>{}</p>\n", &article),
      None => format!("<p>Article id {} not found</p>", id),
    }
  })
  .join()
  .unwrap()
  .into()
}

/// axum handler for "PUT /articles" which creates a new article resource.
/// This demo shows how axum can extract JSON data into a Article struct.
pub async fn put(extract::Json(article): extract::Json<Article>) -> response::Html<String> {
  thread::spawn(move || {
    let mut data = ARTICLE_DATA.lock().unwrap();
    data.insert(article.id, article.clone());
    format!("Put article: {}", &article)
  })
  .join()
  .unwrap()
  .into()
}

/// axum handler for "GET /articles/:id/form" which responds with a form.
/// This demo shows how to write a typical HTML form with input fields.
pub async fn get_form_by_id(extract::Path(id): extract::Path<u32>) -> response::Html<String> {
  thread::spawn(move || {
    let data = ARTICLE_DATA.lock().unwrap();
    match data.get(&id) {
      Some(article) => format!(
        concat!(
          "<form method=\"post\" action=\"/articles/{}/form\">\n",
          "<input type=\"hidden\" name=\"id\" value=\"{}\">\n",
          "<p><input name=\"title\" value=\"{}\"></p>\n",
          "<p><input name=\"author\" value=\"{}\"></p>\n",
          "<input type=\"submit\" value=\"Save\">\n",
          "</form>\n"
        ),
        &article.id, &article.id, &article.title, &article.author_id
      ),
      None => format!("<p>Article id {} not found</p>", id),
    }
  })
  .join()
  .unwrap()
  .into()
}

/// axum handler for "POST /articles/:id/form" which submits an HTML form.
/// This demo shows how to do a form submission then update a resource.
pub async fn post_form_by_id(form: extract::Form<Article>) -> response::Html<String> {
  let new_book: Article = form.0;
  thread::spawn(move || {
    let mut data = ARTICLE_DATA.lock().unwrap();
    if data.contains_key(&new_book.id) {
      data.insert(new_book.id, new_book.clone());
      format!("<p>{}</p>\n", &new_book)
    } else {
      format!("Article id not found: {}", &new_book.id)
    }
  })
  .join()
  .unwrap()
  .into()
}

/// axum handler for "DELETE /articles/:id" which destroys a resource.
/// This demo extracts an id, then mutates the article in the ARTICLE_DATA store.
pub async fn delete_by_id(
  axum::extract::Path(id): axum::extract::Path<u32>,
) -> axum::response::Html<String> {
  thread::spawn(move || {
    let mut data = ARTICLE_DATA.lock().unwrap();
    if data.contains_key(&id) {
      data.remove(&id);
      format!("Delete article id: {}", &id)
    } else {
      format!("Article id not found: {}", &id)
    }
  })
  .join()
  .unwrap()
  .into()
}
