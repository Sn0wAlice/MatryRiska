use actix_web::{web, Scope};
use super::dispatch;


pub fn init_web() -> Scope {
    web::scope("").service(dispatch::dispatch)
}
