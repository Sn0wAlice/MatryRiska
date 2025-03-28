// The web controler for the C1 route
use std::fs;
use crate::helper::database::C2RiskSources;


#[tracing::instrument(level = "info")]
pub async fn c2(path:String) -> String {


    if path == "c2/" {
        return main().await;
    } else if path == "c2/create" {
        return create().await;
    } else if path.starts_with("c2/") {
        let id = path.trim_start_matches("c2/").parse::<i32>().unwrap_or(0);
        return detail(id).await;
    } 

    return "__404".to_string();
}


async fn main() -> String {
    // Fetch all feared events asynchronously
    let all = C2RiskSources::c2_get_all_risk().await;

    // Initialize an empty string to store the generated HTML
    let mut str = String::new();

    // Read the base HTML template for each feared event item
    let base = fs::read_to_string("html/c2/files/risk-solo.html").unwrap();

    // Loop through each feared event and replace placeholders with actual values
    for m in all {
        // Replace placeholders in the base template with actual feared event data
        // Replace placeholders in the base template with actual risk source data
        let new = base.replace("{{risk_id}}", &m.risk_id.to_string())
            .replace("{{source_risque}}", &m.source_risque)
            .replace("{{objectifs_vises}}", &m.objectifs_vises)
            .replace("{{motivation}}", &m.motivation.as_deref().unwrap_or(""))
            .replace("{{ressources}}", &m.ressources.as_deref().unwrap_or(""))
            .replace("{{pertinence_sr_ov}}", &m.pertinence_sr_ov.map_or("".to_string(), |v| v.to_string()))
            .replace("{{priorite}}", &m.priorite.map_or("".to_string(), |v| v.to_string()))
            .replace("{{retenu}}", if m.retenu { "Yes" } else { "No" })
            .replace("{{justification_exclusion_sr_ov}}", &m.justification_exclusion_sr_ov.as_deref().unwrap_or(""));
        // Append the updated template for this item to the main HTML string
        str.push_str(&new);
    }

    // Read the main feared events list HTML template
    return fs::read_to_string("html/c2/list-risk.html").unwrap()
        .replace("{{risk_list}}", &str);
}

async fn create() -> String {
    // Read the main feared events list HTML template
    return fs::read_to_string("html/c2/create-risk.html").unwrap();
}



async fn detail(id:i32) -> String {


    let detail = C2RiskSources::c2_get_risk_detail(id).await;
    
    if detail.len() == 0 {
        return "__404".to_string();
    }

    let detail = detail[0].clone();

    // Read the main feared events list HTML template
    return fs::read_to_string("html/c2/detail-risk.html").unwrap()
        .replace("{{risk_id}}", &detail.risk_id.to_string())
        .replace("{{source_risque}}", &detail.source_risque)
        .replace("{{objectifs_vises}}", &detail.objectifs_vises)
        .replace("{{motivation}}", detail.motivation.as_deref().unwrap_or(""))
        .replace("{{ressources}}", detail.ressources.as_deref().unwrap_or(""))
        .replace("{{pertinence_sr_ov}}", &detail.pertinence_sr_ov.map_or("".to_string(), |v| v.to_string()))
        .replace("{{priorite}}", &detail.priorite.map_or("".to_string(), |v| v.to_string()))
        .replace("{{retenu}}", if detail.retenu { "Yes" } else { "No" })
        .replace("{{justification_exclusion_sr_ov}}", detail.justification_exclusion_sr_ov.as_deref().unwrap_or(""));
}