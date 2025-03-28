use std::fs;
use mysql::*;
use mysql::prelude::*;
use uuid::Uuid;
use std::result::Result;
use crate::helper::trace::trace_logs;

use std::sync::{Arc, Mutex};
use std::{sync::mpsc, thread};
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
    pub likelihood: i32,
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

// C1
#[derive(Debug, Clone)]
pub struct Mission {
    pub mission_id: i32,
    pub mission_name: String,
}

#[derive(Debug, Clone)]
pub struct ValeurMetier {
    pub valeur_id: i32,
    pub mission_id: i32,
    pub valeur_name: String,
    pub valeur_nature: String,
    pub valeur_description: String,
    pub responsable: String,
}

#[derive(Debug, Clone)]
pub struct BienSupport {
    pub support_id: i32,
    pub valeur_id: i32,
    pub support_name: String,
    pub support_description: String,
    pub support_responsable: String,
}


#[derive(Debug, Clone)]
pub struct FearedEvent {
    pub event_id: i32,
    pub valeur_metier: i32,
    pub evenement_redoute: String,
    pub impact: String,
    pub gravite: i32,
}


#[derive(Debug, Clone)]
pub struct Gap {
    pub gap_id: i32,
    pub referential_type: String,
    pub referential_name: String,
    pub application_state: i32,
    pub gap: String,
    pub gap_justification: String,
    pub proposed_measures: String,
}

#[derive(Debug, Clone)]
pub struct C2RiskSources {
    pub risk_id: i32,
    pub source_risque: String,
    pub objectifs_vises: String,
    pub motivation: Option<String>,
    pub ressources: Option<String>,
    pub pertinence_sr_ov: Option<i32>,
    pub priorite: Option<i32>,
    pub retenu: bool,
    pub justification_exclusion_sr_ov: Option<String>,
}

#[derive(Debug, Clone)]
pub struct C3Stakeholder {
    pub stakeholder_id: i32,
    pub category: String,
    pub stakeholder_name: String,
    pub dependance: i32,
    pub penetration: i32,
    pub maturite_ssi: i32,
    pub confiance: i32,
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

    let mut h = "127.0.0.1";

    let config = fs::read_to_string("config/default.json").unwrap();
    let config: serde_json::Value = serde_json::from_str(config.as_str()).unwrap();

    let port: u16 = config.get("db_port").unwrap().as_u64().unwrap() as u16;
    let host:String = config.get("db_host").unwrap().as_str().unwrap().to_owned();

    // check if process arg --prod is used
    if std::env::args().any(|arg| arg == "--prod") {
        h = host.as_str();
    }

    // Define MySQL connection options
    let opts = mysql::OptsBuilder::new()
        .ip_or_hostname(Some(h))
        .tcp_port(port)
        .db_name(Some("matryriska"))
        .user(Some("matryriska"))
        .pass(Some("StrongPassword123"));

    // hcekc if DB_CLIENT.lock().unwrap().is_none() return any poison error
    if mysql::Pool::new(opts.clone()).is_err() {
        return ;
    }

    // Create a new MySQL connection pool
    let pool = mysql::Pool::new(opts).unwrap();

    unsafe {
        let mut db_client = DB_CLIENT.lock().unwrap();
        *db_client = Some(pool);
    }

}

pub async fn check_db_is_up() -> bool {

    reset_database().await;

    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    if db_client.is_none() {
        return false;
    }

    let db_client = db_client.as_ref().unwrap();

    let mut conn = db_client.get_conn().unwrap();

    let query = "SELECT 1";

    let result = conn.query_map(query, |_: i32| {
        ()
    });

    match result {
        Ok(_) => {
            return true;
        },
        Err(_) => {
            return false;
        }
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

pub async fn delete_risk(risk_uuid: String) -> Result<(), String> {
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
        let query = format!("DELETE FROM risk WHERE risk_uuid = '{}'", risk_uuid);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err("Failed to delete risk".to_owned());
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

pub async fn delete_scenario(scenario_uuid: String) -> Result<(), String> {
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
        let query = format!("DELETE FROM scenario WHERE scenario_uuid = '{}'", scenario_uuid);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err("Failed to delete scenario".to_owned());
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

        let result = conn.query_map(query, |(scenario_uuid, likelihood, reputation, operational, legal_compliance, financial): (String, i32, i32, i32, i32, i32)| {
            ScenarioRisk {
                scenario_uuid: Uuid::parse_str(&scenario_uuid).unwrap(),
                likelihood,
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

pub async fn create_scenario_risk(scenario_uuid: String, likelihood: i32, reputation: i32, operational: i32, legal_compliance: i32, financial: i32) -> Result<(), String> {
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
        let query = format!("INSERT INTO scenario_risk (scenario_uuid, likelihood, reputation, operational, legal_compliance, financial) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')", scenario_uuid, likelihood, reputation, operational, legal_compliance, financial);

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

pub async fn update_scenario_risk(scenario_uuid: String, likelihood: i32, reputation: i32, operational: i32, legal_compliance: i32, financial: i32) -> Result<(), String> {
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
        let query = format!("UPDATE scenario_risk SET likelihood = '{}', reputation = '{}', operational = '{}', legal_compliance = '{}', financial = '{}' WHERE scenario_uuid = '{}'", likelihood, reputation, operational, legal_compliance, financial, scenario_uuid);

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

pub async fn delete_scenario_risk(scenario_uuid: String) -> Result<(), String> {
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
        let query = format!("DELETE FROM scenario_risk WHERE scenario_uuid = '{}'", scenario_uuid);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err("Failed to delete scenario risk".to_owned());
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

pub async fn delete_countermeasure(ctm_uuid: String) -> Result<(), String> {
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
        let query = format!("DELETE FROM countermeasure WHERE ctm_uuid = '{}'", ctm_uuid);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err("Failed to delete countermeasure".to_owned());
            }
        }
    }

    return Err("No database connection".to_owned());
}

pub async fn delete_countermeasure_from_sc(scenario_uuid: String) -> Result<(), String> {
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
        let query = format!("DELETE FROM countermeasure WHERE scenario_uuid = '{}'", scenario_uuid);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err("Failed to delete countermeasure".to_owned());
            }
        }
    }

    return Err("No database connection".to_owned());
}


// ------------ DATABASE C1 ------------
pub async fn c1_get_all_missions() -> Vec<Mission> {
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

    let mut missions: Vec<Mission> = Vec::new();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM c1_mission ORDER BY mission_id ASC");

        let result = conn.query_map(query, |(mission_id, mission_name): (i32, String)| {
            Mission {
                mission_id,
                mission_name
            }
        });

        // check how many rows are returned
        match result {
            Ok(fetched_missions) => {
                for mission in fetched_missions {
                    missions.push(mission);
                }
            },
            Err(_) => {
                return missions;
            }
        }

        return missions;
    }

    println!("No database connection");
    return missions;
}

pub async fn c1_create_mission(mission_name: String) {
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
        let query = format!("INSERT INTO c1_mission (mission_name) VALUES ('{}')", mission_name);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }

    println!("No database connection");
    return;
}

pub async fn c1_get_mission_by_id(mission_id:i32) -> Vec<Mission> {
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

    let mut missions: Vec<Mission> = Vec::new();

    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    let db_client = db_client.as_ref();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM c1_mission WHERE mission_id = '{}' ORDER BY mission_id ASC", mission_id);

        let result = conn.query_map(query, |(mission_id, mission_name): (i32, String)| {
            Mission {
                mission_id,
                mission_name
            }
        });

        // check how many rows are returned
        match result {
            Ok(fetched_missions) => {
                for mission in fetched_missions {
                    missions.push(mission);
                }
            },
            Err(_) => {
                return missions;
            }
        }

        return missions;
    }

    println!("No database connection");
    return missions;
}

pub async fn c1_get_all_valeurmetier(mission_id:i32) -> Vec<ValeurMetier> {
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

    let mut valeurs: Vec<ValeurMetier> = Vec::new();

    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    let db_client = db_client.as_ref();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM c1_valeur_metier WHERE mission_id = '{}' ORDER BY valeur_id ASC", mission_id);

        let result = conn.query_map(query, |(valeur_id, mission_id, valeur_name, valeur_nature, valeur_description, responsable): (i32, i32, String, String, String, String)| {
            ValeurMetier {
                valeur_id,
                mission_id,
                valeur_name,
                valeur_nature,
                valeur_description,
                responsable,
            }
        });

        // check how many rows are returned
        match result {
            Ok(fetched_valeurs) => {
                for valeur in fetched_valeurs {
                    valeurs.push(valeur);
                }
            },
            Err(_) => {
                return valeurs;
            }
        }

        return valeurs;
    }

    println!("No database connection");
    return valeurs;
}

pub async fn c1_get_all_valeurmetier_no_limit() -> Vec<ValeurMetier> {
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

    let mut valeurs: Vec<ValeurMetier> = Vec::new();

    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    let db_client = db_client.as_ref();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM c1_valeur_metier ORDER BY valeur_id ASC");

        let result = conn.query_map(query, |(valeur_id, mission_id, valeur_name, valeur_nature, valeur_description, responsable): (i32, i32, String, String, String, String)| {
            ValeurMetier {
                valeur_id,
                mission_id,
                valeur_name,
                valeur_nature,
                valeur_description,
                responsable,
            }
        });

        // check how many rows are returned
        match result {
            Ok(fetched_valeurs) => {
                for valeur in fetched_valeurs {
                    valeurs.push(valeur);
                }
            },
            Err(_) => {
                return valeurs;
            }
        }

        return valeurs;
    }

    println!("No database connection");
    return valeurs;
}

pub async fn c1_create_valeurmetier(mission_id: i32, valeur_name: String, valeur_nature: String, valeur_description: String, responsable: String) {
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
        let query = format!("INSERT INTO c1_valeur_metier (mission_id, valeur_name, valeur_nature, valeur_description, responsable) VALUES ('{}', '{}', '{}', '{}', '{}')", mission_id, valeur_name, valeur_nature, valeur_description, responsable);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }

    println!("No database connection");
    return;
}

pub async fn c1_get_valermetier_by_id(vm_id:i32) -> Vec<ValeurMetier> {
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

    let mut valeurs: Vec<ValeurMetier> = Vec::new();

    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    let db_client = db_client.as_ref();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM c1_valeur_metier WHERE valeur_id = '{}' ORDER BY valeur_id ASC", vm_id);

        let result = conn.query_map(query, |(valeur_id, mission_id, valeur_name, valeur_nature, valeur_description, responsable): (i32, i32, String, String, String, String)| {
            ValeurMetier {
                valeur_id,
                mission_id,
                valeur_name,
                valeur_nature,
                valeur_description,
                responsable,
            }
        });

        // check how many rows are returned
        match result {
            Ok(fetched_valeurs) => {
                for valeur in fetched_valeurs {
                    valeurs.push(valeur);
                }
            },
            Err(_) => {
                return valeurs;
            }
        }

        return valeurs;
    }

    println!("No database connection");
    return valeurs;
}

pub async fn c1_create_asset(vm_id: i32, asset_name: String, asset_description: String, owner: String) {
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
        let query = format!("INSERT INTO c1_bien_support (valeur_id, support_name, support_description, support_responsable) VALUES ('{}', '{}', '{}', '{}')", vm_id, asset_name, asset_description, owner);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }

    println!("No database connection");
    return;
}

pub async fn c1_get_asset_by_vmid(vm_id:i32) -> Vec<BienSupport> {
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

    let mut assets: Vec<BienSupport> = Vec::new();

    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    let db_client = db_client.as_ref();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM c1_bien_support WHERE valeur_id = '{}' ORDER BY support_id ASC", vm_id);

        let result = conn.query_map(query, |(support_id, valeur_id, support_name, support_description, support_responsable): (i32, i32, String, String, String)| {
            BienSupport {
                support_id,
                valeur_id,
                support_name,
                support_description,
                support_responsable
            }
        });

        // check how many rows are returned
        match result {
            Ok(fetched_assets) => {
                for asset in fetched_assets {
                    assets.push(asset);
                }
            },
            Err(_) => {
                return assets;
            }
        }

        return assets;
    }

    println!("No database connection");
    return assets;
}

pub async fn c1_delete_asset_by_id(asset_id:i32) {
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
        let query = format!("DELETE FROM c1_bien_support WHERE support_id = '{}'", asset_id);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }

    println!("No database connection");
    return;
}

pub async fn c1_delete_vm_by_id(vm_id: i32) {
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
        let query = format!("DELETE FROM c1_valeur_metier WHERE valeur_id = '{}'", vm_id);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }

    println!("No database connection");
    return;
}

pub async fn c1_delete_mission_by_id(mission_id: i32) {
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
        let query = format!("DELETE FROM c1_mission WHERE mission_id = '{}'", mission_id);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }

    println!("No database connection");
    return;
}

pub async fn c1_feared_event_create(event_name:String, impacts:String, valeur_metier_id:i32, gravity:i32) {
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
        let query = format!("INSERT INTO c1_feared_event (evenement_redoute, impact, valeur_metier, gravite) VALUES ('{}', '{}', '{}', '{}')", event_name, impacts, valeur_metier_id, gravity);


        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }

    println!("No database connection");
    return;
}

pub async fn c1_delete_feared_event(event_id:i32) {
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
        let query = format!("DELETE FROM c1_feared_event WHERE event_id = '{}'", event_id);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }

    println!("No database connection");
    return;
}

pub async fn c1_get_all_feared_event() -> Vec<FearedEvent> {
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

    let mut events: Vec<FearedEvent> = Vec::new();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();

        let query = format!("SELECT event_id, evenement_redoute, impact, valeur_metier, gravite FROM c1_feared_event ORDER BY event_id ASC");
   
        let result = conn.query_map(query, |(event_id, evenement_redoute, impact, valeur_metier, gravite): (i32, String, String, i32, i32)| {
            FearedEvent {
                event_id,
                evenement_redoute,
                impact,
                valeur_metier,
                gravite
            }
        });

        // check how many rows are returned
        match result {
            Ok(fetched_events) => {
                for event in fetched_events {
                    events.push(event);
                }
            },
            Err(_) => {
                return events;
            }
        }

        return events;
    }

    println!("No database connection");
    return events;
}

pub async fn c1_create_gap(g_ref_type:String, g_ref_name:String, g_state:i32, g_gap:String, g_gap_why:String, g_gap_counter:String) {
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

        let query = format!("INSERT INTO c1_gaps (referential_type, referential_name, application_state, gap, gap_justification, proposed_measures) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')", g_ref_type, g_ref_name, g_state, g_gap, g_gap_why, g_gap_counter);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }
    
    println!("No database connection");
    return;
}

pub async fn c1_delete_gap(gap_id:i32) {
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
        
        let query = format!("DELETE FROM c1_gaps WHERE gap_id = '{}'", gap_id);
        
        let result = conn.query_drop(query);
        
        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }
    
    println!("No database connection");
    return;
}

pub async fn c1_get_all_gaps() -> Vec<Gap> {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };

    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap().is_none() return any poison".to_owned());
        std::process::exit(1);
    }
    
    // check if need to create new client
    if lock_result.unwrap().is_none() {
        new_client().await;
    }
    
    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    let db_client = db_client.as_ref();
    
    let mut gaps: Vec<Gap> = Vec::new();
    
    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        
        let query = format!("SELECT gap_id, referential_type, referential_name, application_state, gap, gap_justification, proposed_measures FROM c1_gaps ORDER BY gap_id ASC");
        
        let result = conn.query_map(query, |(gap_id, referential_type, referential_name, application_state, gap, gap_justification, proposed_measures): (i32, String, String, i32, String, String, String)| {
            Gap {
                gap_id,
                referential_type,
                referential_name,
                application_state,
                gap,
                gap_justification,
                proposed_measures
            }
        });
        
        // check how many rows are returned
        match result {
            Ok(fetched_gaps) => {
                for gap in fetched_gaps {
                    gaps.push(gap);
                }
            },
            Err(_) => {
                return gaps;
            }
        }
        
        return gaps;
    }
    
    println!("No database connection");
    return gaps;
}

pub async fn c1_get_gaps_by_id(gap_id:i32) -> Vec<Gap> {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };

    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap().is_none() return any poison".to_owned());
        std::process::exit(1);
    }
    
    // check if need to create new client
    if lock_result.unwrap().is_none() {
        new_client().await;
    }
    
    let mut gaps: Vec<Gap> = Vec::new();
    
    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    let db_client = db_client.as_ref();
    
    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        
        let query = format!("SELECT gap_id, referential_type, referential_name, application_state, gap, gap_justification, proposed_measures FROM c1_gaps WHERE gap_id = '{}' ORDER BY gap_id ASC", gap_id);
        
        let result = conn.query_map(query, |(gap_id, referential_type, referential_name, application_state, gap, gap_justification, proposed_measures): (i32, String, String, i32, String, String, String)| {
            Gap {
                gap_id,
                referential_type,
                referential_name,
                application_state,
                gap,
                gap_justification,
                proposed_measures
            }
        });
        
        // check how many rows are returned
        match result {
            Ok(fetched_gaps) => {
                for gap in fetched_gaps {
                    gaps.push(gap);
                }
            },
            Err(_) => {
                return gaps;
            }
        }
        
        return gaps;
    }
    
    println!("No database connection");
    return gaps;
}

// ------------ C2RiskSources ------------
pub async fn c2_get_all_risk() -> Vec<C2RiskSources> {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };

    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap().is_none() return any poison".to_owned());
        std::process::exit(1);
    }
    
    // check if need to create new client
    if lock_result.unwrap().is_none() {
        new_client().await;
    }

    
    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    let db_client = db_client.as_ref();

    let mut risks: Vec<C2RiskSources> = Vec::new();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        
        let query = format!("SELECT risk_id, source_risque, objectifs_vises, motivation, ressources, pertinence_sr_ov, priorite, retenu, justification_exclusion_sr_ov FROM c2_risk_sources ORDER BY risk_id ASC");

        let result = conn.query_map(query, |(risk_id, source_risque, objectifs_vises, motivation, ressources, pertinence_sr_ov, priorite, retenu, justification_exclusion_sr_ov): (i32, String, String, Option<String>, Option<String>, Option<i32>, Option<i32>, bool, Option<String>)| {
            C2RiskSources {
                risk_id,
                source_risque,
                objectifs_vises,
                motivation,
                ressources,
                pertinence_sr_ov,
                priorite,
                retenu,
                justification_exclusion_sr_ov
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

pub async fn c2_create_risk(source_risque:String, objectifs_vises:String, motivation:String, ressources:String, pertinence_sr_ov:i32, priorite:i32, retenu:bool, justification_exclusion_sr_ov:String) {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };

    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap().is_none() return any poison".to_owned());
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
        
        let query = format!("INSERT INTO c2_risk_sources (source_risque, objectifs_vises, motivation, ressources, pertinence_sr_ov, priorite, retenu, justification_exclusion_sr_ov) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', {}, '{}')", source_risque, objectifs_vises, motivation, ressources, pertinence_sr_ov, priorite, retenu, justification_exclusion_sr_ov);
        
        let result = conn.query_drop(query);
        
        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }
    
    println!("No database connection");
    return;
}

pub async fn c2_delete_risk_by_id(risk_id:i32) {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };
    
    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap().is_none() return any poison".to_owned());
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
        
        let query = format!("DELETE FROM c2_risk_sources WHERE risk_id = '{}'", risk_id);
        
        let result = conn.query_drop(query);
        
        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }
    
    println!("No database connection");
    return;
}

pub async fn c2_get_risk_detail(risk_id:i32) -> Vec<C2RiskSources> {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };
    
    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap().is_none() return any poison".to_owned());
        std::process::exit(1);
    }
    
    // check if need to create new client
    if lock_result.unwrap().is_none() {
        new_client().await;
    }
    
    let mut risks: Vec<C2RiskSources> = Vec::new();
    
    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };
    
    let db_client = db_client.as_ref();
    
    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        
        let query = format!("SELECT risk_id, source_risque, objectifs_vises, motivation, ressources, pertinence_sr_ov, priorite, retenu, justification_exclusion_sr_ov FROM c2_risk_sources WHERE risk_id = '{}' ORDER BY risk_id ASC", risk_id);
        
        let result = conn.query_map(query, |(risk_id, source_risque, objectifs_vises, motivation, ressources, pertinence_sr_ov, priorite, retenu, justification_exclusion_sr_ov): (i32, String, String, Option<String>, Option<String>, Option<i32>, Option<i32>, bool, Option<String>)| {
            C2RiskSources {
                risk_id,
                source_risque,
                objectifs_vises,
                motivation,
                ressources,
                pertinence_sr_ov,
                priorite,
                retenu,
                justification_exclusion_sr_ov
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


// ------------ C3Stakeholder ------------
pub async fn c3_get_all_stakeholder() -> Vec<C3Stakeholder> {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };
    
    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap().is_none() return any poison".to_owned());
        std::process::exit(1);
    }
    
    // check if need to create new client
    if lock_result.unwrap().is_none() {
        new_client().await;
    }
    
    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };
    
    let db_client = db_client.as_ref();
    
    let mut stakeholders: Vec<C3Stakeholder> = Vec::new();
    
    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        
        let query = format!("SELECT stakeholder_id, category, stakeholder_name, dependance, penetration, maturite_ssi, confiance FROM c3_stakeholders ORDER BY stakeholder_id ASC");

        let result = conn.query_map(query, |(stakeholder_id, category, stakeholder_name, dependance, penetration, maturite_ssi, confiance): (i32, String, String, i32, i32, i32, i32)| {
            C3Stakeholder {
                stakeholder_id,
                category,
                stakeholder_name,
                dependance,
                penetration,
                maturite_ssi,
                confiance
            }
        });
        
        // check how many rows are returned
        match result {
            Ok(fetched_stakeholders) => {
                for stakeholder in fetched_stakeholders {
                    stakeholders.push(stakeholder);
                }
            },
            Err(_) => {
                return stakeholders;
            }
        }
        
        return stakeholders;
    }
    
    println!("No database connection");
    return stakeholders;
}

pub async fn c3_create_stakeholder(category:String, stakeholder_name:String, dependance:i32, penetration:i32, maturite_ssi:i32, confiance:i32) {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };
    
    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap().is_none() return any poison".to_owned());
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
        
        let query = format!("INSERT INTO c3_stakeholders (category, stakeholder_name, dependance, penetration, maturite_ssi, confiance) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')", category, stakeholder_name, dependance, penetration, maturite_ssi, confiance);
        
        let result = conn.query_drop(query);
        
        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }
    
    println!("No database connection");
    return;
}

pub async fn c3_delete_stakeholder_by_id(stakeholder_id:i32) {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };
    
    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap().is_none() return any poison".to_owned());
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
        
        let query = format!("DELETE FROM c3_stakeholders WHERE stakeholder_id = '{}'", stakeholder_id);
        
        let result = conn.query_drop(query);
        
        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }
    
    println!("No database connection");
    return;
}

pub async fn c3_get_stakeholder_detail(stakeholder_id:i32) -> Vec<C3Stakeholder> {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };
    
    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap().is_none() return any poison".to_owned());
        std::process::exit(1);
    }
    
    // check if need to create new client
    if lock_result.unwrap().is_none() {
        new_client().await;
    }
    
    let mut stakeholders: Vec<C3Stakeholder> = Vec::new();
    
    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };
    
    let db_client = db_client.as_ref();
    
    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();
        
        let query = format!("SELECT stakeholder_id, category, stakeholder_name, dependance, penetration, maturite_ssi, confiance FROM c3_stakeholders WHERE stakeholder_id = '{}' ORDER BY stakeholder_id ASC", stakeholder_id);
        
        let result = conn.query_map(query, |(stakeholder_id, category, stakeholder_name, dependance, penetration, maturite_ssi, confiance): (i32, String, String, i32, i32, i32, i32)| {
            C3Stakeholder {
                stakeholder_id,
                category,
                stakeholder_name,
                dependance,
                penetration,
                maturite_ssi,
                confiance
            }
        });
        
        // check how many rows are returned
        match result {
            Ok(fetched_stakeholders) => {
                for stakeholder in fetched_stakeholders {
                    stakeholders.push(stakeholder);
                }
            },
            Err(_) => {
                return stakeholders;
            }
        }
        
        return stakeholders;
    }
    
    println!("No database connection");
    return stakeholders;
}


// ------------ DATABASE UTILS ------------
pub async fn check_if_table_exist(table_name:String) -> bool {
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

        let query = format!("SELECT table_name FROM information_schema.tables WHERE table_name = '{}' LIMIT 1", table_name);

        let result = conn.query_map(query, |(table_name): (String)| {
            table_name
        });

        // check how many rows are returned
        match result {
            Ok(fetched_table) => {
                if fetched_table.len() > 0 {
                    return true;
                }
            },
            Err(_) => {
                return false;
            }
        }

        return false;
    }

    println!("No database connection");
    return false;
}

pub async fn create_table(table_name:String, column:Vec<serde_json::Value>) {
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

        let mut query = format!("CREATE TABLE {} (", table_name);

        for (i, col) in column.iter().enumerate() {
            if i == column.len() - 1 {
                query.push_str(&format!("{} {})", col["name"], col["type"]));
            } else {
                query.push_str(&format!("{} {}, ", col["name"], col["type"]));
            }
        }

        query = query.replace("\"", "");

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }

    println!("No database connection");
    return;
}

pub async fn check_column_exist(table_name:String, column_name:String) -> bool {
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

        let query = format!("SELECT column_name FROM information_schema.columns WHERE table_name = '{}' AND column_name = '{}' LIMIT 1", table_name, column_name);

        let result = conn.query_map(query, |(column_name): (String)| {
            column_name
        });

        // check how many rows are returned
        match result {
            Ok(fetched_column) => {
                if fetched_column.len() > 0 {
                    return true;
                }
            },
            Err(_) => {
                return false;
            }
        }

        return false;
    }

    println!("No database connection");
    return false;
}

pub async fn add_column(table_name:String, column_name:String, column_type:String) {
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

        let query = format!("ALTER TABLE {} ADD COLUMN {} {}", table_name, column_name, column_type);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }

    println!("No database connection");
    return;
}









