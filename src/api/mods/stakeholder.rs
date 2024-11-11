use actix_web::{CustomizeResponder, HttpResponse, Responder};
use serde_json::{json, Value};
use crate::helper::functions::extract_string_from_obj_value;
use crate::helper::database::{c3_create_stakeholder, c3_delete_stakeholder_by_id};

pub async fn create(body: Value) -> CustomizeResponder<HttpResponse> {
    // Check if the body contains the required keys
    for key in vec!["category", "stakeholder_name", "dependance", "penetration", "maturite_ssi", "confiance"].iter() {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok()
                .content_type("application/json")
                .body("{\"error\": true, \"status\": \"missing_args\"}")
                .customize();
        }
    }

    let category = extract_string_from_obj_value(body.get("category"));
    let stakeholder_name = extract_string_from_obj_value(body.get("stakeholder_name"));
    let dependance = extract_string_from_obj_value(body.get("dependance"));
    let penetration = extract_string_from_obj_value(body.get("penetration"));
    let maturite_ssi = extract_string_from_obj_value(body.get("maturite_ssi"));
    let confiance = extract_string_from_obj_value(body.get("confiance"));

    let dependance = dependance.parse::<i32>().unwrap_or(0);
    let penetration = penetration.parse::<i32>().unwrap_or(0);
    let maturite_ssi = maturite_ssi.parse::<i32>().unwrap_or(0);
    let confiance = confiance.parse::<i32>().unwrap_or(0);

    // Ensure field lengths
    if category.len() > 255 || stakeholder_name.len() > 255 {
        return HttpResponse::Ok()
            .content_type("application/json")
            .body("{\"error\": true, \"status\": \"field_too_long\"}")
            .customize();
    }

    // Check that required fields are not empty
    if category.is_empty() || stakeholder_name.is_empty() {
        return HttpResponse::Ok()
            .content_type("application/json")
            .body("{\"error\": true, \"status\": \"field_empty\"}")
            .customize();
    }

    // Escape single quotes to prevent SQL injection
    let category = category.replace("'", "\\'");
    let stakeholder_name = stakeholder_name.replace("'", "\\'");

    // Call the function to create the stakeholder
    let _ = c3_create_stakeholder(
        category,
        stakeholder_name,
        dependance,
        penetration,
        maturite_ssi,
        confiance,
    ).await;

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json!({"status": "success"}).to_string())
        .customize()
}

pub async fn delete(body: Value) -> CustomizeResponder<HttpResponse> {
    // Check if the body contains the required key
    if let Some(id) = body.get("stakeholder_id") {
        let id = extract_string_from_obj_value(Some(id));
        let id = id.parse::<i32>().unwrap_or(0);

        // Call the function to delete the stakeholder
        let _ = c3_delete_stakeholder_by_id(id).await;

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