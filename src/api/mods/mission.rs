// export the home route handler
use std::fs;

use actix_web::{CustomizeResponder, HttpResponse, Responder};
use serde_json::{json, Value};
use crate::helper::functions::{extract_string_from_obj_value, is_uuid_v4};
use crate::helper::database::{Risk, c1_create_mission, c1_get_all_valeurmetier, c1_get_asset_by_vmid, c1_delete_mission_by_id};

use super::asset;


pub async fn create(body:Value) -> CustomizeResponder<HttpResponse> {


    // check the body contain good key
    for key in vec!["name"] {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let doc_name = extract_string_from_obj_value(body.get("name"));

    // check if doc_name < 255 char
    if doc_name.len() > 255 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"name_too_long\"}").customize();
    }

    // sql format to cancel sql injection
    let doc_name = doc_name.replace("'", "\\'");


    let _ = c1_create_mission(doc_name).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}


pub async fn delete(body:Value) -> CustomizeResponder<HttpResponse> {
    // check the body contain good key
    for key in vec!["mission_id"] {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let mission_id = extract_string_from_obj_value(body.get("mission_id"));

    // convert mission id to i32
    let mission_id = match mission_id.parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"mission_id_not_valid\"}").customize();
        }
    };

    // get all associated vm
    let vms = c1_get_all_valeurmetier(mission_id).await;

    for vm in vms {
        let asset = c1_get_asset_by_vmid(vm.valeur_id).await;

        for a in asset {
            let _ = asset::delete(json!({"asset_id": a.support_id})).await;
        }

        let _ = asset::delete(json!({"valeurmetier_id": vm.valeur_id})).await;
    }

    let _ = c1_delete_mission_by_id(mission_id).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}