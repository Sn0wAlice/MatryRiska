// export the home route handler
use std::fs;

use crate::helper::functions::is_uuid_v4;
use crate::helper::database::get_risk_detail;

#[tracing::instrument(level = "info")]
pub async fn create(path:String) -> String {

    // get the risk uuid
    let risk_uuid = path.replace("scenario/create/", "");

    if !is_uuid_v4(&risk_uuid) {
        return "__404".to_string();
    }
    
    // check if risk exist
    let risk_detail = get_risk_detail(risk_uuid).await;

    if risk_detail.is_empty() {
        return "__404".to_string();
    }

    let risk_detail = risk_detail.get(0).unwrap();

    let index = fs::read_to_string("html/scenario/create.html").unwrap()
        .replace("{{risk_title}}", risk_detail.risk_name.as_str())
        .replace("{{risk_uuid}}", risk_detail.risk_uuid.to_string().as_str())
        .replace("{{risk_description}}", risk_detail.risk_description.as_str());

    return index;
}
