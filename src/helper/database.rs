use chrono::NaiveDate;
use mysql::*;
use mysql::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::result::Result;
use serde_json::json;
use crate::api::mods::scenario;
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


#[derive(Debug, Clone)]
pub struct Scenario {
    pub scenario_uuid: Uuid,
    pub risk_uuid: Uuid,
    pub scenario_description: String,
    pub threat_description: String,
    pub add_note: String,
}


#[derive(Debug, Clone)]
pub struct ScenarioRisk {
    pub scenario_uuid: Uuid,
    pub likehood: i32,
    pub reputation: i32,
    pub operational: i32,
    pub legal_compliance: i32,
    pub financial: i32,
}

#[derive(Debug, Clone)]
pub struct Countermeasure {
    pub ctm_uuid: Uuid,
    pub scenario_uuid: Uuid,
    pub title: String,
    pub description: String,
    pub solved: i32,
    pub solved_description: String,
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


// ------------ DATABASE RISK ------------
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

pub async fn get_risk_detail(risk_uuid:String) -> Vec<Risk> {
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
        let query = format!("SELECT * FROM risk WHERE risk_uuid = '{}' ORDER BY risk_name ASC", risk_uuid);

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

pub async fn update_risk(risk_uuid: String, risk_name: String, risk_description: String) -> Result<(), String> {
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
        let query = format!("UPDATE risk SET risk_name = '{}', risk_description = '{}' WHERE risk_uuid = '{}'", risk_name, risk_description, risk_uuid);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err("Failed to update risk".to_owned());
            }
        }
    }

    return Err("No database connection".to_owned());
}

// ------------ DATABASE SCENARIO ------------
pub async fn get_all_scenario_of_risk(risk_uuid:String) -> Vec<Scenario> {
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

    let mut scenarios: Vec<Scenario> = Vec::new();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM scenario WHERE risk_uuid = '{}' ORDER BY scenario_description ASC", risk_uuid);

        let result = conn.query_map(query, |(scenario_uuid, risk_uuid, scenario_description, threat_description, add_note): (String, String, String, String, String)| {
            Scenario {
                scenario_uuid: Uuid::parse_str(&scenario_uuid).unwrap(),
                risk_uuid: Uuid::parse_str(&risk_uuid).unwrap(),
                scenario_description,
                threat_description,
                add_note
            }
        });

        // check how many rows are returned
        match result {
            Ok(fetched_scenarios) => {
                for scenario in fetched_scenarios {
                    scenarios.push(scenario);
                }
            },
            Err(_) => {
                return scenarios;
            }
        }

        return scenarios;
    }

    println!("No database connection");
    return scenarios;
}

pub async fn get_scenario_detail(scenario_uuid:String) -> Vec<Scenario> {
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

    let mut scenarios: Vec<Scenario> = Vec::new();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM scenario WHERE scenario_uuid = '{}' ORDER BY scenario_description ASC", scenario_uuid);

        let result = conn.query_map(query, |(scenario_uuid, risk_uuid, scenario_description, threat_description, add_note): (String, String, String, String, String)| {
            Scenario {
                scenario_uuid: Uuid::parse_str(&scenario_uuid).unwrap(),
                risk_uuid: Uuid::parse_str(&risk_uuid).unwrap(),
                scenario_description,
                threat_description,
                add_note
            }
        });

        // check how many rows are returned
        match result {
            Ok(fetched_scenarios) => {
                for scenario in fetched_scenarios {
                    scenarios.push(scenario);
                }
            },
            Err(_) => {
                return scenarios;
            }
        }

        return scenarios;
    }

    println!("No database connection");
    return scenarios;
}

pub async fn create_new_scenario(risk_uuid: String, scenario_description: String, threat_description: String, add_note: String) -> Uuid {
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
        let scuuid = Uuid::new_v4();
        let query = format!("INSERT INTO scenario (scenario_uuid, risk_uuid, scenario_description, threat_description, add_note) VALUES ('{}', '{}', '{}', '{}', '{}')", scuuid, risk_uuid, scenario_description, threat_description, add_note);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return scuuid;
            },
            Err(_) => {
                return Uuid::nil();
            }
        }
    }

    return Uuid::nil();
}

pub async fn update_scenario(scenario_uuid: String, scenario_description: String, threat_description: String, add_note: String) -> Result<(), String> {
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
        let query = format!("UPDATE scenario SET scenario_description = '{}', threat_description = '{}', add_note = '{}' WHERE scenario_uuid = '{}'", scenario_description, threat_description, add_note, scenario_uuid);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err("Failed to update scenario".to_owned());
            }
        }
    }

    return Err("No database connection".to_owned());
}

pub async fn get_scenario_risk(scenario_uuid:String) -> Vec<ScenarioRisk> {
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

    let mut scenarios: Vec<ScenarioRisk> = Vec::new();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM scenario_risk WHERE scenario_uuid = '{}' ORDER BY scenario_uuid ASC", scenario_uuid);

        let result = conn.query_map(query, |(scenario_uuid, likehood, reputation, operational, legal_compliance, financial): (String, i32, i32, i32, i32, i32)| {
            ScenarioRisk {
                scenario_uuid: Uuid::parse_str(&scenario_uuid).unwrap(),
                likehood,
                reputation,
                operational,
                legal_compliance,
                financial
            }
        });

        // check how many rows are returned
        match result {
            Ok(fetched_scenarios) => {
                for scenario in fetched_scenarios {
                    scenarios.push(scenario);
                }
            },
            Err(_) => {
                return scenarios;
            }
        }

        return scenarios;
    }

    println!("No database connection");
    return scenarios;
}

pub async fn create_scenario_risk(scenario_uuid: String, likehood: i32, reputation: i32, operational: i32, legal_compliance: i32, financial: i32) -> Result<(), String> {
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
        let query = format!("INSERT INTO scenario_risk (scenario_uuid, likehood, reputation, operational, legal_compliance, financial) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')", scenario_uuid, likehood, reputation, operational, legal_compliance, financial);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err("Failed to insert new scenario risk".to_owned());
            }
        }
    }

    return Err("No database connection".to_owned());
}

pub async fn update_scenario_risk(scenario_uuid: String, likehood: i32, reputation: i32, operational: i32, legal_compliance: i32, financial: i32) -> Result<(), String> {
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
        let query = format!("UPDATE scenario_risk SET likehood = '{}', reputation = '{}', operational = '{}', legal_compliance = '{}', financial = '{}' WHERE scenario_uuid = '{}'", likehood, reputation, operational, legal_compliance, financial, scenario_uuid);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err("Failed to update scenario risk".to_owned());
            }
        }
    }

    return Err("No database connection".to_owned());
}

// ------------ DATABASE countermeasure ------------
pub async fn get_all_countermeasure_of_sc(scenario_uuid:String) -> Vec<Countermeasure> {
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

    let mut countermeasures: Vec<Countermeasure> = Vec::new();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM countermeasure WHERE scenario_uuid = '{}' ORDER BY title ASC", scenario_uuid);

        let result = conn.query_map(query, |(ctm_uuid, scenario_uuid, title, description, solved, solved_description): (String, String, String, String, i32, String)| {
            Countermeasure {
                ctm_uuid: Uuid::parse_str(&ctm_uuid).unwrap(),
                scenario_uuid: Uuid::parse_str(&scenario_uuid).unwrap(),
                title,
                description,
                solved,
                solved_description
            }
        });

        // check how many rows are returned
        match result {
            Ok(fetched_countermeasures) => {
                for countermeasure in fetched_countermeasures {
                    countermeasures.push(countermeasure);
                }
            },
            Err(_) => {
                return countermeasures;
            }
        }

        return countermeasures;
    }

    println!("No database connection");
    return countermeasures;
}

pub async fn create_countermeasure(scenario_uuid: String, title: String, description: String) -> Result<(), String> {
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
        let ctm_uuid = Uuid::new_v4();
        let query = format!("INSERT INTO countermeasure (ctm_uuid, scenario_uuid, title, description, solved, solved_description) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')", ctm_uuid, scenario_uuid, title, description, 0, "");

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err("Failed to insert new countermeasure".to_owned());
            }
        }
    }

    return Err("No database connection".to_owned());
}

pub async fn get_all_countermeasure_from_risk_uuid(risk_uuid:String) -> Vec<Countermeasure> {
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

    let mut countermeasures: Vec<Countermeasure> = Vec::new();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM countermeasure WHERE scenario_uuid IN (SELECT scenario_uuid FROM scenario WHERE risk_uuid = '{}') ORDER BY title ASC", risk_uuid);

        let result = conn.query_map(query, |(ctm_uuid, scenario_uuid, title, description, solved, solved_description): (String, String, String, String, i32, String)| {
            Countermeasure {
                ctm_uuid: Uuid::parse_str(&ctm_uuid).unwrap(),
                scenario_uuid: Uuid::parse_str(&scenario_uuid).unwrap(),
                title,
                description,
                solved,
                solved_description
            }
        });

        // check how many rows are returned
        match result {
            Ok(fetched_countermeasures) => {
                for countermeasure in fetched_countermeasures {
                    countermeasures.push(countermeasure);
                }
            },
            Err(_) => {
                return countermeasures;
            }
        }

        return countermeasures;
    }

    println!("No database connection");
    return countermeasures;
}

pub async fn get_ctm_by_id(ctm_uuid:String) -> Vec<Countermeasure> {
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

    let mut countermeasures: Vec<Countermeasure> = Vec::new();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM countermeasure WHERE ctm_uuid = '{}' ORDER BY title ASC", ctm_uuid);

        let result = conn.query_map(query, |(ctm_uuid, scenario_uuid, title, description, solved, solved_description): (String, String, String, String, i32, String)| {
            Countermeasure {
                ctm_uuid: Uuid::parse_str(&ctm_uuid).unwrap(),
                scenario_uuid: Uuid::parse_str(&scenario_uuid).unwrap(),
                title,
                description,
                solved,
                solved_description
            }
        });

        // check how many rows are returned
        match result {
            Ok(fetched_countermeasures) => {
                for countermeasure in fetched_countermeasures {
                    countermeasures.push(countermeasure);
                }
            },
            Err(_) => {
                return countermeasures;
            }
        }

        return countermeasures;
    }

    println!("No database connection");
    return countermeasures;
}

pub async fn update_countermeasure(ctm_uuid: String, title: String, description: String, solved: i32, solved_description: String) -> Result<(), String> {
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
        let query = format!("UPDATE countermeasure SET title = '{}', description = '{}', solved = '{}', solved_description = '{}' WHERE ctm_uuid = '{}'", title, description, solved, solved_description, ctm_uuid);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err("Failed to update countermeasure".to_owned());
            }
        }
    }

    return Err("No database connection".to_owned());
}


