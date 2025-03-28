use actix_web::{CustomizeResponder, HttpResponse, Responder};
use serde_json::{json, Value};
use crate::helper::functions::extract_string_from_obj_value;
use crate::helper::database::C2RiskSources;

pub async fn create(body: Value) -> CustomizeResponder<HttpResponse> {
    // Check if the body contains the required keys
    for key in vec!["source_risque", "objectifs_vises", "motivation", "ressources", "pertinence_sr_ov", "priorite", "retenu", "justification_exclusion_sr_ov"].iter() {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok()
                .content_type("application/json")
                .body("{\"error\": true, \"status\": \"missing_args\"}")
                .customize();
        }
    }

    let source_risque = extract_string_from_obj_value(body.get("source_risque"));
    let objectifs_vises = extract_string_from_obj_value(body.get("objectifs_vises"));
    let motivation = extract_string_from_obj_value(body.get("motivation"));
    let ressources = extract_string_from_obj_value(body.get("ressources"));
    let pertinence_sr_ov = extract_string_from_obj_value(body.get("pertinence_sr_ov")).parse::<i32>().unwrap_or(0);
    let priorite = extract_string_from_obj_value(body.get("priorite")).parse::<i32>().unwrap_or(0);
    let retenu = extract_string_from_obj_value(body.get("retenu")) == "true";
    let justification_exclusion_sr_ov = extract_string_from_obj_value(body.get("justification_exclusion_sr_ov"));

    // Ensure field lengths
    if source_risque.len() > 255 || objectifs_vises.len() > 255 || motivation.len() > 1000 || ressources.len() > 1000 || justification_exclusion_sr_ov.len() > 1000 {
        return HttpResponse::Ok()
            .content_type("application/json")
            .body("{\"error\": true, \"status\": \"field_too_long\"}")
            .customize();
    }

    // all fields are required
    if source_risque.is_empty() || objectifs_vises.is_empty() || motivation.is_empty() || ressources.is_empty() || justification_exclusion_sr_ov.is_empty() {
        return HttpResponse::Ok()
            .content_type("application/json")
            .body("{\"error\": true, \"status\": \"field_empty\"}")
            .customize();
    }

    // Escape single quotes to prevent SQL injection
    let source_risque = source_risque.replace("'", "\\'");
    let objectifs_vises = objectifs_vises.replace("'", "\\'");
    let motivation = motivation.replace("'", "\\'");
    let ressources = ressources.replace("'", "\\'");
    let justification_exclusion_sr_ov = justification_exclusion_sr_ov.replace("'", "\\'");

    // Call the function to create the risk source
    let _ = C2RiskSources::c2_create_risk(
        source_risque,
        objectifs_vises,
        motivation,
        ressources,
        pertinence_sr_ov,
        priorite,
        retenu,
        justification_exclusion_sr_ov,
    ).await;

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json!({"status": "success"}).to_string())
        .customize()
}

pub async fn delete(body: Value) -> CustomizeResponder<HttpResponse> {
    // Check if the body contains the required keys
    if let Some(id) = body.get("risk_id") {
        let id = extract_string_from_obj_value(Some(id));
        let id = id.parse::<i32>().unwrap_or(0);

        // Call the function to delete the risk source
        let _ = C2RiskSources::c2_delete_risk_by_id(id).await;

        return HttpResponse::Ok()
            .content_type("application/json")
            .body(json!({"status": "success"}).to_string())
            .customize();
    } else {
        return HttpResponse::Ok()
            .content_type("application/json")
            .body("{\"error\": true, \"status\": \"missing_args\"}")
            .customize();
    }
}