// export the home route handler
use std::fs;

#[tracing::instrument(level = "info")]
pub async fn home() -> String {
  let index = fs::read_to_string("html/home/index.html").unwrap();

  return index;
}