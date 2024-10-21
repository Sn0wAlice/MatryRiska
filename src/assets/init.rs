use actix_web::{web, Scope, get, HttpResponse, Responder};
use actix_web::http::StatusCode;
use std::fs;
use serde_json::Value;

// import json file: utils/types.json by reading the file and parsing it
fn read_type_file_content() -> String {
    fs::read_to_string("utils/types.json").unwrap()
}

fn deserialize_json(json_str: String) -> Value {
    match serde_json::from_str(json_str.as_str()) {
        Ok(value) => value,
        Err(_) => Value::Null, // or handle the error in some appropriate way
    }
}

//save the json value to a global variable
pub static mut TYPES: Value = Value::Null;

//initialize the global variable
pub fn init_types() {
    unsafe {
        TYPES = deserialize_json(read_type_file_content());
    }
}

#[get("/{path:.*}")]
pub async fn get_assets(path: web::Path<String>) -> impl Responder {
    
    if unsafe { TYPES.is_null() } {
        init_types();
    }
    
    // check if file ./assets/{path} exists and is a file
    if fs::metadata(format!("./assets/{}", path.clone())).is_ok() && fs::metadata(format!("./assets/{}", path.clone())).unwrap().is_file() {
        // read file as buffer
        let file = fs::read(format!("./assets/{}", path.clone())).unwrap();
        let file_extension = path.split(".").collect::<Vec<&str>>().pop().map(|ext| ext.to_lowercase()).unwrap();


        // get the type in the json file. json array is format: [{ "ext": ["txt"], "type": "text/plain" } ...]
        // so we need to get the type of the file by its extension
        let file_type = unsafe {
            TYPES.as_array().unwrap().iter().find(|x| x["ext"].as_array().unwrap().contains(&serde_json::Value::String(file_extension.to_string())))
        };

        // return file (force type gif)
        return HttpResponse::Ok().content_type(file_type.unwrap()["type"].as_str().unwrap()).body(file);
    }

    // return 404
    return HttpResponse::Ok().content_type("text/plain").status(StatusCode::NOT_FOUND).body("404 Not Found");
}

pub fn init_assets() -> Scope {
    web::scope("/assets").service(get_assets)
}
