use actix_web::{web, Scope, post, HttpResponse, HttpRequest, Responder, CustomizeResponder};
use crate::helper::trace::trace_logs;
use serde_json::json;
use futures::StreamExt;
use serde_json::Value;


use crate::helper::database::{Risk};
use crate::api::mods::*;


const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[post("/{path:.*}")]
pub async fn handler(path: web::Path<String>, mut payload: web::Payload, req: HttpRequest) -> impl Responder {

    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = match chunk {
            Ok(chunk) => chunk,
            Err(_) => {
                return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"error\"}").customize();
            }
        };
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"error\"}").customize();
        }
        body.extend_from_slice(&chunk);
    }

    // Get the expected data
    let str_data = std::str::from_utf8(&body).expect("Invalid UTF-8");
    let parsed_json: Value = serde_json::from_str(str_data).expect("Failed to parse JSON");

    match path.to_string().as_str() {
        "" => {
            return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"OK\"}").customize();
        },
        "risk/create" => {
            return risk::create(parsed_json).await;
        }
        "risk/update" => {
            return risk::update(parsed_json).await;
        }
        "risk/delete" => {
            return risk::delete(parsed_json).await;
        }
        "scenario/create" => {
            return scenario::create(parsed_json).await;
        }
        "scenario/update" => {
            return scenario::update(parsed_json).await;
        }
        "scenario/delete" => {
            return scenario::delete(parsed_json).await;
        }
        "countermeasure/create" => {
            return countermeasure::create(parsed_json).await;
        }
        "countermeasure/update" => {
            return countermeasure::update(parsed_json).await;
        }
        "countermeasure/delete" => {
            return countermeasure::delete(parsed_json).await;
        }
        _ => {
            trace_logs("Path not found".to_string());
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": \"path not found\"}").customize();
        }
    }
    
}


pub fn init_api() -> Scope {
    web::scope("/api").service(handler)
}
