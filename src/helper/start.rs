
use std::fs;

use crate::helper::database::{check_if_table_exist, create_table, check_column_exist, add_column};


pub async fn startup() {
    // check all the necessary database archi

    if !std::env::args().any(|arg| arg == "--no-wait") {
        // wait 5 secondes
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
   

    let file = fs::read_to_string("assets/_internals/db.json").unwrap();
    // convert the string to json
    let json: Vec<serde_json::Value> = serde_json::from_str(&file).unwrap();

    for table in json.iter() {
        if check_if_table_exist(table["name"].as_str().unwrap().to_owned()).await {

            //check if column exist
            for col in table["columns"].as_array().unwrap().iter() {
                if check_column_exist(table["name"].as_str().unwrap().to_owned(), col["name"].as_str().unwrap().to_owned()).await {
                    //println!("Column {} exist", col["name"].as_str().unwrap());
                } else {
                    add_column(table["name"].as_str().unwrap().to_owned(), col["name"].as_str().unwrap().to_owned(), col["type"].as_str().unwrap().to_owned()).await;
                }
            }

        } else {
            //println!("Table {} does not exist", table["name"].as_str().unwrap());
            create_table(table["name"].as_str().unwrap().to_owned(), table["columns"].as_array().unwrap().to_owned()).await;
        }
    }

    println!("Database setup completed");

}