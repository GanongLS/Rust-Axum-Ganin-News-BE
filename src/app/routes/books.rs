use axum::{response, extract};
use std::thread;

use crate::data::{DATA, models::book::Book};

/// axum handler for "GET /books" which responds with a resource page.
/// This demo uses our DATA; a production app could use a database.
/// This demo must clone the DATA in order to sort items by title.
pub async fn get() -> response::Html<String> {
  thread::spawn(move || {
    let data = DATA.lock().unwrap();
    let mut books = data.values().collect::<Vec<_>>().clone();
    books.sort_by(|a, b| a.title.cmp(&b.title));
    books
      .iter()
      .map(|&book| format!("<p>{}</p>\n", &book))
      .collect::<String>()
  })
  .join()
  .unwrap()
  .into()
}

/// axum handler for "GET /books/:id" which responds with one resource HTML page.
/// This demo app uses our DATA variable, and iterates on it to find the id.
pub async fn get_by_id(extract::Path(id): extract::Path<u32>) -> response::Html<String> {
  thread::spawn(move || {
    let data = DATA.lock().unwrap();
    match data.get(&id) {
      Some(book) => format!("<p>{}</p>\n", &book),
      None => format!("<p>Book id {} not found</p>", id),
    }
  })
  .join()
  .unwrap()
  .into()
}

/// axum handler for "PUT /books" which creates a new book resource.
/// This demo shows how axum can extract JSON data into a Book struct.
pub async fn put(extract::Json(book): extract::Json<Book>) -> response::Html<String> {
  thread::spawn(move || {
    let mut data = DATA.lock().unwrap();
    data.insert(book.id, book.clone());
    format!("Put book: {}", &book)
  })
  .join()
  .unwrap()
  .into()
}

/// axum handler for "GET /books/:id/form" which responds with a form.
/// This demo shows how to write a typical HTML form with input fields.
pub async fn get_form_by_id(extract::Path(id): extract::Path<u32>) -> response::Html<String> {
  thread::spawn(move || {
    let data = DATA.lock().unwrap();
    match data.get(&id) {
      Some(book) => format!(
        concat!(
          "<form method=\"post\" action=\"/books/{}/form\">\n",
          "<input type=\"hidden\" name=\"id\" value=\"{}\">\n",
          "<p><input name=\"title\" value=\"{}\"></p>\n",
          "<p><input name=\"author\" value=\"{}\"></p>\n",
          "<input type=\"submit\" value=\"Save\">\n",
          "</form>\n"
        ),
        &book.id, &book.id, &book.title, &book.author
      ),
      None => format!("<p>Book id {} not found</p>", id),
    }
  })
  .join()
  .unwrap()
  .into()
}

/// axum handler for "POST /books/:id/form" which submits an HTML form.
/// This demo shows how to do a form submission then update a resource.
pub async fn post_form_by_id(form: extract::Form<Book>) -> response::Html<String> {
  let new_book: Book = form.0;
  thread::spawn(move || {
    let mut data = DATA.lock().unwrap();
    if data.contains_key(&new_book.id) {
      data.insert(new_book.id, new_book.clone());
      format!("<p>{}</p>\n", &new_book)
    } else {
      format!("Book id not found: {}", &new_book.id)
    }
  })
  .join()
  .unwrap()
  .into()
}

/// axum handler for "DELETE /books/:id" which destroys a resource.
/// This demo extracts an id, then mutates the book in the DATA store.
pub async fn delete_by_id(
  axum::extract::Path(id): axum::extract::Path<u32>,
) -> axum::response::Html<String> {
  thread::spawn(move || {
    let mut data = DATA.lock().unwrap();
    if data.contains_key(&id) {
      data.remove(&id);
      format!("Delete book id: {}", &id)
    } else {
      format!("Book id not found: {}", &id)
    }
  })
  .join()
  .unwrap()
  .into()
}