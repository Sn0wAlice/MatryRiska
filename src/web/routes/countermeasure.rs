// export the home route handler
use std::fs;

use uuid::Uuid;

use crate::helper::functions::is_uuid_v4;
use crate::helper::database::{get_scenario_detail, get_ctm_by_id};

#[tracing::instrument(level = "info")]
pub async fn create(path:String) -> String {

    // get the risk uuid
    let risk_uuid = path.replace("countermeasure/create/", "");

    if !is_uuid_v4(&risk_uuid) {
        return "__404".to_string();
    }
    
    // check if risk exist
    let scenario_detail = get_scenario_detail(risk_uuid).await;

    if scenario_detail.is_empty() {
        return "__404".to_string();
    }

    let scenario_detail = scenario_detail.get(0).unwrap();

    let index = fs::read_to_string("html/countermeasure/create.html").unwrap()
        .replace("{{scenario_uuid}}", scenario_detail.scenario_uuid.to_string().as_str());

    return index;
}


pub async fn detail(path: String) -> String {
    let sc_uuid = path.replace("countermeasure/detail/", "");
  
    if !is_uuid_v4(&sc_uuid) {
      return "__404".to_string();
    }
  
    // get risk detail
    let ctm_detail = get_ctm_by_id(sc_uuid.clone()).await;
    if ctm_detail.is_empty() {
      return "__404".to_string();
    }

    let ctm = ctm_detail.get(0).unwrap();


    let index = fs::read_to_string("html/countermeasure/detail.html").unwrap()
        .replace("{{ctm_uuid}}", ctm.ctm_uuid.to_string().as_str())
        .replace("{{scenario_uuid}}", ctm.scenario_uuid.to_string().as_str())
        .replace("{{title}}", ctm.title.as_str())
        .replace("{{description}}", ctm.description.as_str())
        .replace("{{solved}}", ctm.solved.to_string().as_str())
        .replace("{{solved_description}}", ctm.solved_description.as_str());
    
    return index;
}

pub async fn update(path: String) -> String {
    let sc_uuid = path.replace("countermeasure/update/", "");
  
    if !is_uuid_v4(&sc_uuid) {
      return "__404".to_string();
    }
  
    // get risk detail
    let ctm_detail = get_ctm_by_id(sc_uuid.clone()).await;
    if ctm_detail.is_empty() {
      return "__404".to_string();
    }

    let ctm = ctm_detail.get(0).unwrap();

    let index = fs::read_to_string("html/countermeasure/update.html").unwrap()
        .replace("{{ctm_uuid}}", ctm.ctm_uuid.to_string().as_str())
        .replace("{{scenario_uuid}}", ctm.scenario_uuid.to_string().as_str())
        .replace("{{title}}", ctm.title.as_str())
        .replace("{{description}}", ctm.description.as_str())
        .replace("{{solved}}", ctm.solved.to_string().as_str())
        .replace("{{solved_description}}", ctm.solved_description.as_str());

    return index;
}


pub async fn delete(path: String) -> String {
  let sc_uuid = path.replace("countermeasure/delete/", "");

  if !is_uuid_v4(&sc_uuid) {
    return "__404".to_string();
  }

  // get risk detail
  let ctm_detail = get_ctm_by_id(sc_uuid.clone()).await;
  if ctm_detail.is_empty() {
    return "__404".to_string();
  }

  let ctm = ctm_detail.get(0).unwrap();

  let index = fs::read_to_string("html/countermeasure/delete.html").unwrap()
      .replace("{{ctm_uuid}}", ctm.ctm_uuid.to_string().as_str())
      .replace("{{scenario_uuid}}", ctm.scenario_uuid.to_string().as_str())
      .replace("{{title}}", ctm.title.as_str())
      .replace("{{description}}", ctm.description.as_str())
      .replace("{{solved}}", ctm.solved.to_string().as_str())
      .replace("{{solved_description}}", ctm.solved_description.as_str());

  return index;
}
