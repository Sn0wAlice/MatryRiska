// export the home route handler
use std::fs;

use actix_web::{CustomizeResponder, HttpResponse, Responder};
use serde_json::{json, Value};
use crate::helper::functions::{extract_string_from_obj_value, is_uuid_v4};
use crate::helper::database::{Risk, create_countermeasure, get_scenario_detail};


pub async fn create(body:Value) -> CustomizeResponder<HttpResponse> {


    // check the body contain good key
    for key in vec!["name", "description", "scenario_uuid"] {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let doc_name = extract_string_from_obj_value(body.get("name"));
    let doc_description = extract_string_from_obj_value(body.get("description"));
    let scenario_uuid = extract_string_from_obj_value(body.get("scenario_uuid"));

    // check if doc_name < 255 char
    if doc_name.len() > 255 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"name_too_long\"}").customize();
    }

    // check scenario_uuid is a valid uuid
    if !is_uuid_v4(&scenario_uuid) {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_uuid\"}").customize();
    }

    // check if scenario exist
    let scenario_detail = get_scenario_detail(scenario_uuid.clone()).await;

    if scenario_detail.is_empty() {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"scenario_not_found\"}").customize();
    }

    // sql format to cancel sql injection
    let doc_name = doc_name.replace("'", "\\'");
    let doc_description = doc_description.replace("'", "\\'");


    let _ = create_countermeasure(scenario_uuid, doc_name, doc_description).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}

