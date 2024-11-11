extern crate matryriska;
use std::fs;
use actix_cors::Cors;
use actix_web::{HttpServer,App};
use matryriska::{api,web,assets};

// init the tracing module
use matryriska::helper::trace::{init_trace,trace_logs};
use matryriska::helper::start::startup;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{}", fs::read_to_string("utils/ascii.art").unwrap().as_str());
    init_trace();
    trace_logs("Server is starting...".to_string());

    startup().await;

    let config = fs::read_to_string("config/default.json").unwrap();
    let config: serde_json::Value = serde_json::from_str(config.as_str()).unwrap();

    let port: u16 = config.get("web_port").unwrap().as_u64().unwrap() as u16;
    trace_logs(format!("Server is running on port: {}",port));
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();
        App::new().wrap(cors).service(api::init::init_api()).service(assets::init::init_assets()).service(web::init::init_web())
    }).bind(("0.0.0.0",port))?.run().await
}
