// export the home route handler
use std::fs;

use crate::web::routes::risk::get_id;
use crate::helper::functions::is_uuid_v4;
use crate::helper::database::{get_risk_detail, get_scenario_detail,get_scenario_risk, get_all_countermeasure_of_sc};

#[tracing::instrument(level = "info")]
pub async fn create(path:String) -> String {

    // get the risk uuid
    let risk_uuid = path.replace("scenario/create/", "");

    if !is_uuid_v4(&risk_uuid) {
        return "__404".to_string();
    }
    
    // check if risk exist
    let risk_detail = get_risk_detail(risk_uuid).await;

    if risk_detail.is_empty() {
        return "__404".to_string();
    }

    let risk_detail = risk_detail.get(0).unwrap();

    let index = fs::read_to_string("html/scenario/create.html").unwrap()
        .replace("{{risk_title}}", risk_detail.risk_name.as_str())
        .replace("{{risk_uuid}}", risk_detail.risk_uuid.to_string().as_str())
        .replace("{{risk_description}}", risk_detail.risk_description.as_str());

    return index;
}


pub async fn detail(path:String) -> String {
    let scenario_uuid = path.replace("scenario/detail/", "");

    // check if scenario is a valid uuid
    if !is_uuid_v4(&scenario_uuid) {
        return "__404".to_string();
    }

    // check if scenario exist
    let scenario_detail = get_scenario_detail(scenario_uuid).await;

    if scenario_detail.is_empty() {
        return "__404".to_string();
    }

    let scenario_detail = scenario_detail.get(0).unwrap();

    let scenario_risk = get_scenario_risk(scenario_detail.scenario_uuid.to_string()).await;
    let scenario_risk = scenario_risk.get(0).unwrap();


    let countermeasure = get_all_countermeasure_of_sc(scenario_detail.scenario_uuid.to_string()).await;

    let mut countermeasure_html = String::new();
    let base_countermeasure = fs::read_to_string("html/scenario/files/countermeasure.html").unwrap();

    for cm in countermeasure {
        let cm_html = base_countermeasure.replace("{{cm_uuid}}", cm.ctm_uuid.to_string().as_str())
            .replace("{{cm_title}}", cm.title.as_str())
            .replace("{{cm_id}}", get_id(cm.ctm_uuid).to_string().as_str())
            .replace("{{cm_solved}}", cm.solved.to_string().as_str())
            .replace("{{cm_description}}", cm.description.as_str());

        countermeasure_html.push_str(cm_html.as_str());
    }


    let index = fs::read_to_string("html/scenario/detail.html").unwrap()
        .replace("{{scenario_uuid}}", scenario_detail.scenario_uuid.to_string().as_str())
        .replace("{{scenario_note}}", scenario_detail.add_note.to_string().as_str())
        .replace("{{scenario_description}}", scenario_detail.scenario_description.as_str())
        .replace("{{scenario_threat}}", scenario_detail.threat_description.to_string().as_str())
        .replace("{{sc_likehood}}", scenario_risk.likehood.to_string().as_str())
        .replace("{{sc_operational}}", scenario_risk.operational.to_string().as_str())
        .replace("{{sc_legal_compliance}}", scenario_risk.legal_compliance.to_string().as_str())
        .replace("{{sc_financial}}", scenario_risk.financial.to_string().as_str())
        .replace("{{sc_final_risk}}", calculate_risk(scenario_risk.likehood, scenario_risk.operational, scenario_risk.legal_compliance, scenario_risk.financial, scenario_risk.reputation))
        .replace("{{sc_reputation}}", scenario_risk.reputation.to_string().as_str())
        .replace("{{ctm_list}}", countermeasure_html.as_str());

    return index;
}

pub async fn update(path:String) -> String {
    let scenario_uuid = path.replace("scenario/update/", "");

    // check if scenario is a valid uuid
    if !is_uuid_v4(&scenario_uuid) {
        return "__404".to_string();
    }

    // check if scenario exist
    let scenario_detail = get_scenario_detail(scenario_uuid).await;

    if scenario_detail.is_empty() {
        return "__404".to_string();
    }

    let scenario_detail = scenario_detail.get(0).unwrap();

    let scenario_risk = get_scenario_risk(scenario_detail.scenario_uuid.to_string()).await;
    let scenario_risk = scenario_risk.get(0).unwrap();


    let index = fs::read_to_string("html/scenario/update.html").unwrap()
        .replace("{{scenario_uuid}}", scenario_detail.scenario_uuid.to_string().as_str())
        .replace("{{scenario_note}}", scenario_detail.add_note.to_string().as_str())
        .replace("{{scenario_description}}", scenario_detail.scenario_description.as_str())
        .replace("{{scenario_threat}}", scenario_detail.threat_description.to_string().as_str())
        .replace("{{sc_likehood}}", scenario_risk.likehood.to_string().as_str())
        .replace("{{sc_operational}}", scenario_risk.operational.to_string().as_str())
        .replace("{{sc_legal_compliance}}", scenario_risk.legal_compliance.to_string().as_str())
        .replace("{{sc_financial}}", scenario_risk.financial.to_string().as_str())
        .replace("{{sc_reputation}}", scenario_risk.reputation.to_string().as_str());

    return index;

}

pub async fn delete(path:String) -> String {
    let scenario_uuid = path.replace("scenario/delete/", "");

    // check if scenario is a valid uuid
    if !is_uuid_v4(&scenario_uuid) {
        return "__404".to_string();
    }

    // check if scenario exist
    let scenario_detail = get_scenario_detail(scenario_uuid).await;

    if scenario_detail.is_empty() {
        return "__404".to_string();
    }

    let scenario_detail = scenario_detail.get(0).unwrap();


    let index = fs::read_to_string("html/scenario/delete.html").unwrap()
        .replace("{{scenario_uuid}}", scenario_detail.scenario_uuid.to_string().as_str())
        .replace("{{scenario_note}}", scenario_detail.add_note.to_string().as_str())
        .replace("{{scenario_description}}", scenario_detail.scenario_description.as_str())
        .replace("{{scenario_threat}}", scenario_detail.threat_description.to_string().as_str());

    return index;

}

// ----- Utils -----
pub fn calculate_risk(e5: i32, f5: i32, g5: i32, h5: i32, i5: i32) -> &'static str {
    // Calculate the maximum of the inputs F5, G5, H5, I5
    let max_val = f5.max(g5).max(h5).max(i5);

    // First condition: if E5 * MAX(F5, G5, H5, I5) <= 0, return "N/A"
    if e5 * max_val <= 0 {
        return "N/A";
    }

    // LOW conditions
    if (e5 == 1 && (1..=3).contains(&max_val)) || 
       (e5 == 2 && (1..=2).contains(&max_val)) ||
       (e5 == 3 && (1..=2).contains(&max_val)) ||
       (e5 == 4 && (1..=2).contains(&max_val)) ||
       (e5 == 5 && max_val == 1) {
        return "LOW";
    }

    // MEDIUM conditions
    if (e5 == 1 && (4..=5).contains(&max_val)) || 
       (e5 == 2 && (3..=4).contains(&max_val)) ||
       (e5 == 3 && max_val == 3) ||
       (e5 == 4 && max_val == 3) ||
       (e5 == 5 && max_val == 2) ||
       (e5 == 6 && max_val == 1) {
        return "MEDIUM";
    }

    // HIGH conditions
    if (e5 == 1 && max_val == 6) || 
       (e5 == 2 && max_val == 5) ||
       (e5 == 3 && (4..=5).contains(&max_val)) ||
       (e5 == 4 && max_val == 4) ||
       (e5 == 5 && max_val == 3) ||
       (e5 == 6 && max_val == 2) {
        return "HIGH";
    }

    // CRITICAL conditions
    if (e5 == 2 && max_val == 6) || 
       (e5 == 3 && max_val == 6) ||
       (e5 == 4 && (5..=6).contains(&max_val)) ||
       (e5 == 5 && (4..=6).contains(&max_val)) ||
       (e5 == 6 && (3..=5).contains(&max_val)) {
        return "CRITICAL";
    }

    // EXTREME condition
    if e5 == 6 && max_val == 6 {
        return "EXTREME";
    }

    "N/A"
}
