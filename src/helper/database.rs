use chrono::NaiveDate;
use mysql::*;
use mysql::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::result::Result;
use serde_json::json;
use crate::helper::trace::trace_logs;

use std::sync::{Arc, Mutex};
use std::{ptr::addr_of, sync::mpsc, thread};
use tokio::time::{interval, Duration};

use once_cell::sync::Lazy;

// ------------ ALL STRUCTURE ------------

#[derive(Debug, Clone)]
pub struct Risk {
    pub risk_uuid: Uuid,
    pub risk_name: String,
    pub risk_description: String,
}

// ------------ DATABASE SYSTEM ------------

static mut DB_CLIENT: Lazy<Arc<Mutex<Option<mysql::Pool>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

async fn new_client() {

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            periodic_database().await;
            tx.send(()).unwrap(); // Signal that work is done
        });
    });

    reset_database().await
}

async fn periodic_database() {
    let mut interval = interval(Duration::from_secs(300));
    loop {
        interval.tick().await;
        reset_database().await;
    }
}

async fn reset_database() {
    // Define MySQL connection options
    let opts = mysql::OptsBuilder::new()
        .ip_or_hostname(Some("127.0.0.1"))
        .db_name(Some("matryriska"))
        .user(Some("matryriska"))
        .pass(Some("StrongPassword123"));

    // Create a new MySQL connection pool
    let pool = mysql::Pool::new(opts).unwrap();

    unsafe {
        let mut db_client = DB_CLIENT.lock().unwrap();
        *db_client = Some(pool);
    }

}


pub async fn select_all_risk() -> Vec<Risk> {
    
        // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
        let lock_result = unsafe { DB_CLIENT.lock() };
    
        if lock_result.is_err() {
            // kill script
            trace_logs("Error: DB_CLIENT.lock().unwrap() is_none() return any poison".to_owned());
            std::process::exit(1);
        }
    
        // check if need to create new client
        if lock_result.unwrap().is_none() {
            new_client().await;
        }
    
        // perform database operations
        let db_client = unsafe { DB_CLIENT.lock().unwrap() };
    
        let db_client = db_client.as_ref();

        let mut risks: Vec<Risk> = Vec::new();

        if let Some(pool) = db_client {
            let mut conn = pool.get_conn().unwrap();
            let query = format!("SELECT * FROM risk ORDER BY risk_name ASC");

            let result = conn.query_map(query, |(risk_uuid, risk_name, risk_description): (String, String, String)| {
                Risk {
                    risk_uuid: Uuid::parse_str(&risk_uuid).unwrap(),
                    risk_name,
                    risk_description
                }
            });

            // check how many rows are returned
            match result {
                Ok(fetched_risks) => {
                    for risk in fetched_risks {
                        risks.push(risk);
                    }
                },
                Err(_) => {
                    return risks;
                }
            }

            return risks;
        }

        println!("No database connection");
        return risks;
}

pub async fn create_new_risk(risk_name: String, risk_description: String) -> Result<(), String> {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };

    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap() is_none() return any poison".to_owned());
        std::process::exit(1);
    }

    // check if need to create new client
    if lock_result.unwrap().is_none() {
        new_client().await;
    }

    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    let db_client = db_client.as_ref();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        let query = format!("INSERT INTO risk (risk_uuid, risk_name, risk_description) VALUES ('{}', '{}', '{}')", Uuid::new_v4(), risk_name, risk_description);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err("Failed to insert new risk".to_owned());
            }
        }
    }

    return Err("No database connection".to_owned());
}



