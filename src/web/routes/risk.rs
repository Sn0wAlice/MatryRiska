// export the home route handler
use std::fs;

use uuid::Uuid;

use crate::helper::functions::is_uuid_v4;
use crate::helper::database::{get_risk_detail, get_all_scenario_of_risk};

use super::scenario;

#[tracing::instrument(level = "info")]
pub async fn create() -> String {

  let index = fs::read_to_string("html/risk/create.html").unwrap();

  return index;
}

pub async fn detail(path: String) -> String {
  let risk_uuid = path.replace("risk/detail/", "");

  if !is_uuid_v4(&risk_uuid) {
    return "__404".to_string();
  }

  // get risk detail
  let risk_detail = get_risk_detail(risk_uuid.clone()).await;
  if risk_detail.is_empty() {
    return "__404".to_string();
  }

  let risk_detail = risk_detail.get(0).unwrap();


  let scenario_list = get_all_scenario_of_risk(risk_uuid.clone()).await;
  let scenario_count = scenario_list.len();

  let mut str = String::new();
  let base_scenario = fs::read_to_string("html/risk/files/scenario.html").unwrap();

  for scenario in scenario_list {
    let scenario = base_scenario
      .replace("{{scenario_id}}", get_id(scenario.scenario_uuid).as_str())
      .replace("{{scenario_uuid}}", scenario.scenario_uuid.to_string().as_str())
      .replace("{{scenario_description}}", &scenario.scenario_description.as_str())
      .replace("{{threat_description}}", scenario.threat_description.as_str());

    str.push_str(scenario.as_str());
  }


  let index = fs::read_to_string("html/risk/detail.html").unwrap()
    .replace("{{risk_title}}", risk_detail.risk_name.as_str())
    .replace("{{risk_uuid}}", risk_detail.risk_uuid.to_string().as_str())
    .replace("{{risk_description}}", risk_detail.risk_description.as_str())
    .replace("{{scenario_count}}", scenario_count.to_string().as_str())
    .replace("{{sc_list}}", str.as_str());

  return index;
}


pub fn get_id(uuid:Uuid) -> String {
  // only keep the first 8 characters
  let id = uuid.to_string();
  return id.chars().take(8).collect();
}