// export the home route handler
use std::fs;

use actix_web::{CustomizeResponder, HttpResponse, Responder};
use serde_json::{json, Value};
use crate::helper::functions::{extract_string_from_obj_value, is_uuid_v4};
use crate::helper::database::{Risk, c1_get_mission_by_id, c1_create_valeurmetier, c1_delete_asset_by_id, c1_get_asset_by_vmid, c1_delete_vm_by_id};


pub async fn create(body:Value) -> CustomizeResponder<HttpResponse> {


    // check the body contain good key
    for key in vec!["name", "description", "source", "owner", "mission_id"].iter() {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let m_name = extract_string_from_obj_value(body.get("name"));
    let m_description = extract_string_from_obj_value(body.get("description"));
    let m_source = extract_string_from_obj_value(body.get("source"));
    let m_owner = extract_string_from_obj_value(body.get("owner"));
    let m_mission_id = extract_string_from_obj_value(body.get("mission_id"));

    // check if doc_name < 255 char
    if m_name.len() > 255 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"name_too_long\"}").customize();
    }

    // sql format to cancel sql injection
    let m_name = m_name.replace("'", "\\'");

    // check description < 1000 char
    if m_description.len() > 1000 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"description_too_long\"}").customize();
    }

    // check m_source is "processus" of "information"
    if m_source != "processus" && m_source != "information" {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"source_not_valid\"}").customize();
    }

    // check m_owner is a valid String
    if m_owner.len() > 255 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"owner_too_long\"}").customize();
    }

    // convert mission id to i32
    let m_mission_id = match m_mission_id.parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"mission_id_not_valid\"}").customize();
        }
    };

    // check mission exist
    let m = c1_get_mission_by_id(m_mission_id).await;
    if m.len() == 0 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"mission_not_found\"}").customize();
    }

    // replace ' by \' for all
    let m_owner = m_owner.replace("'", "\\'");
    let m_source = m_source.replace("'", "\\'");
    let m_description = m_description.replace("'", "\\'");
    let m_name = m_name.replace("'", "\\'");

    let _ = c1_create_valeurmetier(m_mission_id, m_name, m_source, m_description, m_owner).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}


pub async fn delete(body:Value) -> CustomizeResponder<HttpResponse> {

    // check the body contain good key
    for key in vec!["vm_id"].iter() {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let vm_id = extract_string_from_obj_value(body.get("vm_id"));

    // convert mission id to i32
    let vm_id = match vm_id.parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"vm_id_not_valid\"}").customize();
        }
    };

    let all_asset = c1_get_asset_by_vmid(vm_id).await;
    for a in all_asset {
        let _ = c1_delete_asset_by_id(a.support_id).await;
    }

    let _ = c1_delete_vm_by_id(vm_id).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}

