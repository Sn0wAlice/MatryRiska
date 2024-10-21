// export the home route handler
use std::fs;

use crate::helper::database::{Risk, select_all_risk};

#[tracing::instrument(level = "info")]
pub async fn main() -> String {
  let risk = select_all_risk().await;


  let mut str = String::new();
  let base = fs::read_to_string("html/main/files/sample.html").unwrap();

  for r in risk {
    let new = base.replace("{{risk_name}}", &r.risk_name)
      .replace("{{risk_uuid}}", &r.risk_uuid.to_string());
    str.push_str(&new);
  }


  let index = fs::read_to_string("html/main/index.html").unwrap()
    .replace("{{risk_list}}", &str);

  return index;
}