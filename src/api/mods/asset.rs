// export the home route handler
use std::fs;

use actix_web::{CustomizeResponder, HttpResponse, Responder};
use serde_json::{json, Value};
use crate::helper::functions::{extract_string_from_obj_value, is_uuid_v4};
use crate::helper::database::{Risk, c1_get_valermetier_by_id, c1_create_asset, c1_delete_asset_by_id};


pub async fn create(body:Value) -> CustomizeResponder<HttpResponse> {

    // check the body contain good key
    for key in vec!["name", "description", "owner", "vm_id"].iter() {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let m_name = extract_string_from_obj_value(body.get("name"));
    let m_description = extract_string_from_obj_value(body.get("description"));
    let m_owner = extract_string_from_obj_value(body.get("owner"));
    let m_vm_id = extract_string_from_obj_value(body.get("vm_id"));

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


    // check m_owner is a valid String
    if m_owner.len() > 255 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"owner_too_long\"}").customize();
    }

    // convert mission id to i32
    let m_vm_id = match m_vm_id.parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"vm_id_not_valid\"}").customize();
        }
    };

    // check mission exist
    let m = c1_get_valermetier_by_id(m_vm_id).await;
    if m.len() == 0 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"vm_not_found\"}").customize();
    }

    // replace ' by \' for all
    let m_owner = m_owner.replace("'", "\\'");
    let m_description = m_description.replace("'", "\\'");
    let m_name = m_name.replace("'", "\\'");

    let _ = c1_create_asset(m_vm_id, m_name, m_description, m_owner).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}




pub async fn delete(body:Value) -> CustomizeResponder<HttpResponse> {

    // check the body contain good key
    for key in vec!["asset_id"].iter() {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let asset_id = extract_string_from_obj_value(body.get("asset_id"));

    // convert mission id to i32
    let asset_id = match asset_id.parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"asset_id_not_valid\"}").customize();
        }
    };

    // check mission exist
    let _ = c1_delete_asset_by_id(asset_id).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}

