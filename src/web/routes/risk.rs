// export the home route handler
use std::fs;

#[tracing::instrument(level = "info")]
pub async fn create() -> String {

  let index = fs::read_to_string("html/risk/create.html").unwrap();

  return index;
}