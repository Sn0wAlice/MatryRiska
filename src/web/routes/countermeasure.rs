// export the home route handler
use std::fs;

use uuid::Uuid;

use crate::helper::functions::is_uuid_v4;
use crate::helper::database::{get_scenario_detail};

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
