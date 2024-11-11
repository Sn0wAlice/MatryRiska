// The web controller for the C3 route

use std::fs;
use crate::helper::database::{C3Stakeholder, c3_get_all_stakeholder, c3_get_stakeholder_detail};

#[tracing::instrument(level = "info")]
pub async fn c3(path: String) -> String {
    if path == "c3/stakeholder/" {
        return main().await;
    } else if path == "c3/stakeholder/create" {
        return create().await;
    } else if path.starts_with("c3/stakeholder/") {
        let id = path.trim_start_matches("c3/stakeholder/").parse::<i32>().unwrap_or(0);
        return detail(id).await;
    } 

    "__404".to_string()
}

async fn main() -> String {
    // Fetch all stakeholders asynchronously
    let all = c3_get_all_stakeholder().await;

    // Initialize an empty string to store the generated HTML
    let mut str = String::new();

    // Read the base HTML template for each stakeholder item
    let base = fs::read_to_string("html/c3/files/stakeholder-solo.html").unwrap();

    // Loop through each stakeholder and replace placeholders with actual values
    for m in all {
        
        let exposition = m.dependance as f64 * m.penetration as f64;
        let fiabilite_cyber = m.maturite_ssi as f64 * m.confiance as f64;


        let mut niveau_de_menace = 0.0; // Initialize as a floating-point number
        if fiabilite_cyber > 0.0 {
            let menace_ratio = exposition / fiabilite_cyber;
            niveau_de_menace = (menace_ratio * 100.0).round() / 100.0;
        }

        // Replace placeholders in the base template with actual stakeholder data
        let new = base.replace("{{stakeholder_id}}", &m.stakeholder_id.to_string())
            .replace("{{category}}", &m.category)
            .replace("{{stakeholder_name}}", &m.stakeholder_name)
            .replace("{{dependance}}", &m.dependance.to_string())
            .replace("{{penetration}}", &m.penetration.to_string())
            .replace("{{maturite_ssi}}", &m.maturite_ssi.to_string())
            .replace("{{confiance}}", &m.confiance.to_string())


            .replace("{{exposition}}", exposition.to_string().as_str())
            .replace("{{fiabilite_cyber}}", fiabilite_cyber.to_string().as_str())
            .replace("{{niveau_de_menace}}", niveau_de_menace.to_string().as_str());
        
        // Append the updated template for this item to the main HTML string
        str.push_str(&new);
    }


    // Read the main stakeholder list HTML template
    fs::read_to_string("html/c3/list-stakeholders.html").unwrap()
        .replace("{{stakeholder_list}}", &str)
}

async fn create() -> String {
    // Read the HTML template for creating a stakeholder
    fs::read_to_string("html/c3/create-stakeholder.html").unwrap()
}

async fn detail(id: i32) -> String {
    let detail = c3_get_stakeholder_detail(id).await;
    
    if detail.is_empty() {
        return "__404".to_string();
    }

    let detail = detail[0].clone();

    // Read the detailed stakeholder HTML template
    fs::read_to_string("html/c3/detail-stakeholder.html").unwrap()
        .replace("{{stakeholder_id}}", &detail.stakeholder_id.to_string())
        .replace("{{category}}", &detail.category)
        .replace("{{stakeholder_name}}", &detail.stakeholder_name)
        .replace("{{dependance}}", &detail.dependance.to_string())
        .replace("{{penetration}}", &detail.penetration.to_string())
        .replace("{{maturite_ssi}}", &detail.maturite_ssi.to_string())
        .replace("{{confiance}}", &detail.confiance.to_string())
}