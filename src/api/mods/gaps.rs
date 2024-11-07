// export the home route handler
use std::fs;

use actix_web::{CustomizeResponder, HttpResponse, Responder};
use serde_json::{json, Value};
use crate::helper::functions::{extract_string_from_obj_value, is_uuid_v4};
use crate::helper::database::{Risk, c1_create_gap, c1_delete_gap};


pub async fn create(body:Value) -> CustomizeResponder<HttpResponse> {


    // check the body contain good key
    for key in vec!["g_ref_type", "g_ref_name", "g_state", "g_gap", "g_gap_why", "g_gap_counter"].iter() {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let g_ref_type = extract_string_from_obj_value(body.get("g_ref_type"));
    let g_ref_name = extract_string_from_obj_value(body.get("g_ref_name"));
    let g_state = extract_string_from_obj_value(body.get("g_state"));
    let g_gap = extract_string_from_obj_value(body.get("g_gap"));
    let g_gap_why = extract_string_from_obj_value(body.get("g_gap_why"));
    let g_gap_counter = extract_string_from_obj_value(body.get("g_gap_counter"));

    // check if g_ref_type < 255 char
    if g_ref_type.len() > 255 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"g_ref_type_too_long\"}").customize();
    }

    // check if g_ref_name < 255 char
    if g_ref_name.len() > 255 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"g_ref_name_too_long\"}").customize();
    }

    // check if gstate is a numeric valeu between 0 and 100 included
    let g_state = match g_state.parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"g_state_not_valid\"}").customize();
        }
    };

    // get other is < 2000 char
    if g_gap.len() > 2000 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"g_gap_too_long\"}").customize();
    }

    // get other is < 2000 char
    if g_gap_why.len() > 2000 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"g_gap_why_too_long\"}").customize();
    }

    // get other is < 2000 char
    if g_gap_counter.len() > 2000 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"g_gap_counter_too_long\"}").customize();
    }

    // replace ' by \' for all
    let g_ref_type = g_ref_type.replace("'", "\\'");
    let g_ref_name = g_ref_name.replace("'", "\\'");
    let g_gap = g_gap.replace("'", "\\'");
    let g_gap_why = g_gap_why.replace("'", "\\'");
    let g_gap_counter = g_gap_counter.replace("'", "\\'");

    let _ = c1_create_gap(g_ref_type, g_ref_name, g_state, g_gap, g_gap_why, g_gap_counter).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}


pub async fn delete(body:Value) -> CustomizeResponder<HttpResponse> {

    // check the body contain good key
    for key in vec!["gaps_id"].iter() {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let gaps_id = extract_string_from_obj_value(body.get("gaps_id"));

    // convert mission id to i32
    let gaps_id = match gaps_id.parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"gaps_id_not_valid\"}").customize();
        }
    };


    let _ = c1_delete_gap(gaps_id).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}

