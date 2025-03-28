// export the home route handler
use actix_web::{CustomizeResponder, HttpResponse, Responder};
use serde_json::{json, Value};
use crate::helper::functions::extract_string_from_obj_value;
use crate::helper::database::{FearedEvent, ValeurMetier};


pub async fn create(body:Value) -> CustomizeResponder<HttpResponse> {


    // check the body contain good key
    for key in vec!["name", "impacts", "bv", "gravity"].iter() {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let m_name = extract_string_from_obj_value(body.get("name"));
    let m_impacts = extract_string_from_obj_value(body.get("impacts"));
    let m_bv = extract_string_from_obj_value(body.get("bv"));
    let m_gravity = extract_string_from_obj_value(body.get("gravity"));

    // check if doc_name < 255 char
    if m_name.len() > 255 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"name_too_long\"}").customize();
    }

    // sql format to cancel sql injection
    let m_name = m_name.replace("'", "\\'");

    // check description < 1000 char
    if m_impacts.len() > 2000 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"impacts_too_long\"}").customize();
    }


    // convert mission id to i32
    let m_bv = match m_bv.parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"m_bv_not_valid\"}").customize();
        }
    };

    let m_gravity = match m_gravity.parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"m_gravity_not_valid\"}").customize();
        }
    };

    // gravity must be between 1 and 4 (included)
    if m_gravity < 1 || m_gravity > 4 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"gravity_not_valid\"}").customize();
    }

    

    // check mission exist
    let m = ValeurMetier::c1_get_all_valeurmetier(m_bv).await;
    if m.len() == 0 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"business_value_not_found\"}").customize();
    }

    // replace ' by \' for all
    let m_impacts = m_impacts.replace("'", "\\'");
    let m_name = m_name.replace("'", "\\'");

    let _ = FearedEvent::c1_feared_event_create(m_name, m_impacts, m_bv, m_gravity).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}




pub async fn delete(body:Value) -> CustomizeResponder<HttpResponse> {

    // check the body contain good key
    for key in vec!["event_id"].iter() {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let event_id = extract_string_from_obj_value(body.get("event_id"));

    // convert mission id to i32
    let event_id = match event_id.parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"event_id_not_valid\"}").customize();
        }
    };


    let _ = FearedEvent::c1_delete_feared_event(event_id).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}

