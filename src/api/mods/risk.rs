// export the home route handler
use std::fs;

use actix_web::{CustomizeResponder, HttpResponse, Responder};
use serde_json::{json, Value};
use crate::helper::functions::{extract_string_from_obj_value, is_uuid_v4};
use crate::helper::database::{Risk, create_new_risk, update_risk, delete_risk, get_all_scenario_of_risk, delete_scenario, delete_scenario_risk, delete_countermeasure_from_sc};


pub async fn create(body:Value) -> CustomizeResponder<HttpResponse> {


    // check the body contain good key
    for key in vec!["name", "description"] {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let doc_name = extract_string_from_obj_value(body.get("name"));
    let doc_description = extract_string_from_obj_value(body.get("description"));

    // check if doc_name < 255 char
    if doc_name.len() > 255 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"name_too_long\"}").customize();
    }

    // sql format to cancel sql injection
    let doc_name = doc_name.replace("'", "\\'");
    let doc_description = doc_description.replace("'", "\\'");


    let _ = create_new_risk(doc_name, doc_description).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}

pub async fn update(body:Value) -> CustomizeResponder<HttpResponse> {
    // check the body contain good key
    for key in vec!["uuid", "name", "description"] {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let doc_uuid = extract_string_from_obj_value(body.get("uuid"));
    let doc_name = extract_string_from_obj_value(body.get("name"));
    let doc_description = extract_string_from_obj_value(body.get("description"));

    // check if doc_name < 255 char
    if doc_name.len() > 255 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"name_too_long\"}").customize();
    }

    // sql format to cancel sql injection
    let doc_name = doc_name.replace("'", "\\'");
    let doc_description = doc_description.replace("'", "\\'");

    // check if the uuid is a valid uuid
    if !is_uuid_v4(&doc_uuid) {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_uuid\"}").customize();
    }

    let _ = update_risk(doc_uuid, doc_name, doc_description).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}

pub async fn delete(body:Value) -> CustomizeResponder<HttpResponse> {
    // check the body contain good key
    for key in vec!["uuid"] {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let doc_uuid = extract_string_from_obj_value(body.get("uuid"));

    // check if the uuid is a valid uuid
    if !is_uuid_v4(&doc_uuid) {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_uuid\"}").customize();
    }

    // delete the risk
    let _ = delete_risk(doc_uuid.clone()).await;
    let all_sc = get_all_scenario_of_risk(doc_uuid.clone()).await;

    for sc in all_sc {
        let _ = delete_scenario(sc.scenario_uuid.to_string()).await;
        let _ = delete_scenario_risk(sc.scenario_uuid.to_string()).await;
        let _ = delete_countermeasure_from_sc(sc.scenario_uuid.to_string()).await;
    }

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}