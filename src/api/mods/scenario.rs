// export the home route handler
use actix_web::{CustomizeResponder, HttpResponse, Responder};
use serde_json::{json, Value};
use crate::helper::functions::{extract_string_from_obj_value,};
use crate::helper::database::{Risk, Scenario, ScenarioRisk, get_risk_detail, create_new_scenario, create_scenario_risk, update_scenario, update_scenario_risk, get_scenario_detail};


pub async fn create(body:Value) -> CustomizeResponder<HttpResponse> {

    // check the body contain good key
    for key in vec!["risk_uuid", "sc_scenario_description", "sc_threat_description", "sc_likehood", "sc_reputational", "sc_operational", "sc_legal_compliance", "sc_financial", "sc_custom_note"] {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let risk_uuid = extract_string_from_obj_value(body.get("risk_uuid"));
    let sc_scenario_description = extract_string_from_obj_value(body.get("sc_scenario_description"));
    let sc_threat_description = extract_string_from_obj_value(body.get("sc_threat_description"));
    let sc_likehood = extract_string_from_obj_value(body.get("sc_likehood"));
    let sc_reputational = extract_string_from_obj_value(body.get("sc_reputational"));
    let sc_operational = extract_string_from_obj_value(body.get("sc_operational"));
    let sc_legal_compliance = extract_string_from_obj_value(body.get("sc_legal_compliance"));
    let sc_financial = extract_string_from_obj_value(body.get("sc_financial"));
    let sc_custom_note = extract_string_from_obj_value(body.get("sc_custom_note"));


    // check risk_uuid is a valid uuid
    let risk_uuid = match uuid::Uuid::parse_str(&risk_uuid) {
        Ok(uuid) => uuid,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_uuid\"}").customize();
        }
    };

    // check risk exist
    let risk = get_risk_detail(risk_uuid.to_string()).await;
    if risk.is_empty() {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"risk_not_found\"}").customize();
    }

    // check all the value is a valid integer between 1 and 6 (included)
    let sc_likehood = match sc_likehood.parse::<i32>() {
        Ok(value) => {
            if value >= 1 && value <= 6 {
                value
            } else {
                return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_likehood\"}").customize();
            }
        },
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_likehood\"}").customize();
        }
    };

    let sc_reputational = match sc_reputational.parse::<i32>() {
        Ok(value) => {
            if value >= 1 && value <= 6 {
                value
            } else {
                return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_reputational\"}").customize();
            }
        },
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_reputational\"}").customize();
        }
    };

    let sc_operational = match sc_operational.parse::<i32>() {
        Ok(value) => {
            if value >= 1 && value <= 6 {
                value
            } else {
                return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_operational\"}").customize();
            }
        },
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_operational\"}").customize();
        }
    };

    let sc_legal_compliance = match sc_legal_compliance.parse::<i32>() {
        Ok(value) => {
            if value >= 1 && value <= 6 {
                value
            } else {
                return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_legal_compliance\"}").customize();
            }
        },
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_legal_compliance\"}").customize();
        }
    };

    let sc_financial = match sc_financial.parse::<i32>() {
        Ok(value) => {
            if value >= 1 && value <= 6 {
                value
            } else {
                return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_financial\"}").customize();
            }
        },
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_financial\"}").customize();
        }
    };

    // check security of string
    let sc_scenario_description = sc_scenario_description.replace("\"", "").replace("'", "");
    let sc_threat_description = sc_threat_description.replace("\"", "").replace("'", "");
    let sc_custom_note = sc_custom_note.replace("\"", "").replace("'", "");


    // create the scenario
    let scenario_uuid = create_new_scenario(risk_uuid.to_string(), sc_scenario_description, sc_threat_description, sc_custom_note).await;
    
    // create the scenario risk
    let _ = create_scenario_risk(scenario_uuid.to_string(), sc_likehood, sc_reputational, sc_operational, sc_legal_compliance, sc_financial).await;
    
    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}


pub async fn update(body:Value) -> CustomizeResponder<HttpResponse> {

    // check the body contain good key
    for key in vec!["uuid", "sc_scenario_description", "sc_threat_description", "sc_likehood", "sc_reputational", "sc_operational", "sc_legal_compliance", "sc_financial", "sc_custom_note"] {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let scenario_uuid = extract_string_from_obj_value(body.get("uuid"));
    let sc_scenario_description = extract_string_from_obj_value(body.get("sc_scenario_description"));
    let sc_threat_description = extract_string_from_obj_value(body.get("sc_threat_description"));
    let sc_likehood = extract_string_from_obj_value(body.get("sc_likehood"));
    let sc_reputational = extract_string_from_obj_value(body.get("sc_reputational"));
    let sc_operational = extract_string_from_obj_value(body.get("sc_operational"));
    let sc_legal_compliance = extract_string_from_obj_value(body.get("sc_legal_compliance"));
    let sc_financial = extract_string_from_obj_value(body.get("sc_financial"));
    let sc_custom_note = extract_string_from_obj_value(body.get("sc_custom_note"));

    // check scenario_uuid is a valid uuid
    let scenario_uuid = match uuid::Uuid::parse_str(&scenario_uuid) {
        Ok(uuid) => uuid,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_uuid\"}").customize();
        }
    };

    // check scenario exist
    let scenario = get_scenario_detail(scenario_uuid.to_string()).await;

    if scenario.is_empty() {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"scenario_not_found\"}").customize();
    }

    // check all the value is a valid integer between 1 and 6 (included)
    let sc_likehood = match sc_likehood.parse::<i32>() {
        Ok(value) => {
            if value >= 1 && value <= 6 {
                value
            } else {
                return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_likehood\"}").customize();
            }
        },
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_likehood\"}").customize();
        }
    };

    let sc_reputational = match sc_reputational.parse::<i32>() {
        Ok(value) => {
            if value >= 1 && value <= 6 {
                value
            } else {
                return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_reputational\"}").customize();
            }
        },
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_reputational\"}").customize();
        }
    };

    let sc_operational = match sc_operational.parse::<i32>() {
        Ok(value) => {
            if value >= 1 && value <= 6 {
                value
            } else {
                return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_operational\"}").customize();
            }
        },
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_operational\"}").customize();
        }
    };

    let sc_legal_compliance = match sc_legal_compliance.parse::<i32>() {
        Ok(value) => {
            if value >= 1 && value <= 6 {
                value
            } else {
                return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_legal_compliance\"}").customize();
            }
        },
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_legal_compliance\"}").customize();
        }
    };

    let sc_financial = match sc_financial.parse::<i32>() {
        Ok(value) => {
            if value >= 1 && value <= 6 {
                value
            } else {
                return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_financial\"}").customize();
            }
        },
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"invalid_financial\"}").customize();
        }
    };

    // check security of string
    let sc_scenario_description = sc_scenario_description.replace("\"", "").replace("'", "");
    let sc_threat_description = sc_threat_description.replace("\"", "").replace("'", "");
    let sc_custom_note = sc_custom_note.replace("\"", "").replace("'", "");

    // update the scenario
    let _ = update_scenario(scenario_uuid.to_string(), sc_scenario_description, sc_threat_description, sc_custom_note).await;

    // update the scenario risk
    let _ = update_scenario_risk(scenario_uuid.to_string(), sc_likehood, sc_reputational, sc_operational, sc_legal_compliance, sc_financial).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}