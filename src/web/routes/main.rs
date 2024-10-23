// export the home route handler
use std::fs;

use crate::web::routes::risk::get_id;
use crate::helper::database::{Countermeasure, select_all_risk, get_all_scenario_of_risk, get_all_countermeasure_from_risk_uuid};

#[tracing::instrument(level = "info")]
pub async fn main() -> String {
  let risk = select_all_risk().await;


  let mut str = String::new();
  let base = fs::read_to_string("html/main/files/sample.html").unwrap();

  for r in risk {

    let sc= get_all_scenario_of_risk(r.risk_uuid.clone().to_string()).await;
    let ctm = get_all_countermeasure_from_risk_uuid(r.risk_uuid.clone().to_string()).await;

    let mut avg = average_resolution(ctm.clone());

    if avg.is_nan() {
      avg = 0.0;
    }

    let new = base.replace("{{risk_name}}", &r.risk_name)
      .replace("{{sc_count}}", sc.len().to_string().as_str())
      .replace("{{ctm_count}}", ctm.len().to_string().as_str())
      .replace("{{avg_solved}}", avg.to_string().as_str())
      .replace("{{risk_id}}", get_id(r.risk_uuid).as_str())
      .replace("{{risk_uuid}}", &r.risk_uuid.to_string());
    str.push_str(&new);
  }



  let index = fs::read_to_string("html/main/index.html").unwrap()
    .replace("{{risk_list}}", &str);

  return index;
}

fn average_resolution(ctm_list: Vec<Countermeasure>) -> f64 {
  let mut total = 0.0;
  for ctm in &ctm_list {
    total += ctm.solved as f64;
  }

  return total / ctm_list.len() as f64;
}