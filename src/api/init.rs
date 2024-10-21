use actix_web::{web, Scope,get, HttpResponse, Responder};
use crate::helper::trace::{trace_logs,trace_warn};


#[get("/{path:.*}")]
pub async fn handler(path: web::Path<String>) -> impl Responder {

    trace_logs(format!("api: {}", path.to_string()));

    match path.to_string().as_str() {
        "" => {
            return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"OK\"}");
        }

        _ => {
            trace_warn(format!("404 Not Found: {}", path.to_string()));
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": \"path not found\"}");
        }
    }
}

pub fn init_api() -> Scope {
    web::scope("/api").service(handler)
}
