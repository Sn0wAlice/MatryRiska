
use std::fs;

use crate::helper::database::{check_if_table_exist, create_table, check_column_exist, add_column, check_db_is_up};
use crate::helper::trace::trace_logs;

pub async fn startup() {
    // check all the necessary database archi

    // check file config/default.json exist
    if !fs::metadata("config/default.json").is_ok() {
        fs::write("config/default.json", r#"{
            "db_port": 3306,
            "db_host": "172.20.0.202",
            "web_port": 8080
        }"#).unwrap();
    }


    wait_for_the_db_to_up().await;
   

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

    trace_logs("Database setup completed !".to_owned());

}


async fn wait_for_the_db_to_up() {
    // check if the database is up
    let mut done = false;
    while !done {
        trace_logs("Waiting for the database to be up".to_owned());
        if check_db_is_up().await {
            done = true;
        }
        // wait 2 seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    
}