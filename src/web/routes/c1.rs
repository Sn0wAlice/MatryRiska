// The web controler for the C1 route

use std::fs;
use crate::helper::database::{c1_get_all_missions, c1_get_mission_by_id, c1_get_all_valeurmetier, c1_get_valermetier_by_id, c1_get_asset_by_vmid, c1_get_all_valeurmetier_no_limit, c1_get_all_feared_event, c1_get_all_gaps, c1_get_gaps_by_id};


#[tracing::instrument(level = "info")]
pub async fn c1(path:String) -> String {


    if path == "c1/coremissions" {
        return coremission().await;
    } else if path == "c1/coremissions/create" {
        return coremission_create().await;
    } else if path.starts_with("c1/coremissions/detail/") {
        let mission_id = path.replace("c1/coremissions/detail/", "");
        return coremission_detail(mission_id.parse::<i32>().unwrap_or(0)).await;
    } else if path.starts_with("c1/vm/create/") {
        let mission_id = path.replace("c1/vm/create/", "");
        return vm_create(mission_id.parse::<i32>().unwrap_or(0)).await;
    } else if path.starts_with("c1/vm/detail/") {
        let vm_id = path.replace("c1/vm/detail/", "");
        return vm_detail(vm_id.parse::<i32>().unwrap_or(0)).await;
    } else if path.starts_with("c1/asset/create/") {
        let vm_id = path.replace("c1/asset/create/", "");
        return asset_create(vm_id.parse::<i32>().unwrap_or(0)).await;
    } else if path == "c1/fevnt" {
        return fevnt().await;
    } else if path == "c1/fevnt/create" {
        return fevnt_create().await;
    } else if path == "c1/gaps" {
        return gaps().await;
    } else if path == "c1/gaps/create" {
        return gaps_create().await;
    } else if path.starts_with("c1/gaps/") {
        let vm_id = path.replace("c1/gaps/", "");
        return gaps_detail(vm_id.parse::<i32>().unwrap_or(0)).await;
    }

    return "__404".to_string();
}

async fn coremission() -> String {

    // Get all missions
    let missions = c1_get_all_missions().await;

    let mut str = String::new();
    let base = fs::read_to_string("html/c1/files/m-solo.html").unwrap();

    for m in missions {
        let new = base.replace("{{mission_name}}", &m.mission_name)
            .replace("{{mission_id}}", &m.mission_id.to_string());
        str.push_str(&new);
    }


    let index = fs::read_to_string("html/c1/list-missions.html").unwrap()
        .replace("{{mission_list}}", &str);

    return index;
}

async fn coremission_create() -> String {
    return fs::read_to_string("html/c1/create-mission.html").unwrap();
}

async fn coremission_detail(mission_id:i32) -> String {

    // get mission details
    let mission = c1_get_mission_by_id(mission_id.clone()).await;
    if mission.len() == 0 {
        return "__404".to_string();
    }

    let mission = &mission[0];

    let get_all_valeurmetier = c1_get_all_valeurmetier(mission_id).await;


    let mut str = String::new();
    let base = fs::read_to_string("html/c1/files/vm-solo.html").unwrap();

    for m in get_all_valeurmetier {
        let new = base.replace("{{vm_name}}", &m.valeur_name)
            .replace("{{vm_id}}", &m.valeur_id.to_string())
            .replace("{{vm_description}}", &m.valeur_description)
            .replace("{{vm_nature}}", &m.valeur_nature.to_string())
            .replace("{{vm_resp}}", &m.responsable.to_string());

        str.push_str(&new);
    }


    return fs::read_to_string("html/c1/detail-mission.html").unwrap()
        .replace("{{mission_title}}", &mission.mission_name)
        .replace("{{job_list}}", &str)
        .replace("{{mission_id}}", mission_id.to_string().as_str());
}

async fn vm_detail(vm_id:i32) -> String {

    let vm = c1_get_valermetier_by_id(vm_id).await;

    if vm.len() == 0 {
        return "__404".to_string();
    }

    let vm = &vm[0];

    let get_all_asset = c1_get_asset_by_vmid(vm_id).await;

    let mut str = String::new();
    let base = fs::read_to_string("html/c1/files/asset-solo.html").unwrap();

    for m in get_all_asset {
        let new = base.replace("{{asset_name}}", &m.support_name)
            .replace("{{asset_id}}", &m.support_id.to_string())
            .replace("{{asset_description}}", &m.support_description)
            .replace("{{asset_owner}}", &m.support_responsable);

        str.push_str(&new);
    }


    return fs::read_to_string("html/c1/detail-vm.html").unwrap()
        .replace("{{mission_id}}", &vm.mission_id.to_string())
        .replace("{{vm_id}}", &vm.valeur_id.to_string())
        .replace("{{vm_source}}", &vm.valeur_nature)
        .replace("{{vm_description}}", &vm.valeur_description)
        .replace("{{vm_responsable}}", &vm.responsable)
        .replace("{{vm_name}}", &vm.valeur_name)
        .replace("{{asset_list}}", &str);
}

async fn vm_create(mission_id:i32) -> String {
    return fs::read_to_string("html/c1/create-vm.html").unwrap()
        .replace("{{mission_id}}", mission_id.to_string().as_str());
}

async fn asset_create(vm_id:i32) -> String {
    return fs::read_to_string("html/c1/create-asset.html").unwrap()
        .replace("{{vm_id}}", vm_id.to_string().as_str());
}

async fn fevnt() -> String {

    let all = c1_get_all_feared_event().await;

    let mut str = String::new();
    let base = fs::read_to_string("html/c1/files/fevnt-solo.html").unwrap();

    for m in all {
        let new = base.replace("{{fevnt_name}}", &m.evenement_redoute)
            .replace("{{fevnt_id}}", &m.event_id.to_string())
            .replace("{{fevnt_impacts}}", &m.impact.replace("\n", "<br>"))
            .replace("{{fevnt_vm}}", &m.valeur_metier.to_string())
            .replace("{{fevnt_gravity}}", &m.gravite.to_string());

        str.push_str(&new);
    }

    return fs::read_to_string("html/c1/list-fevnt.html").unwrap()
        .replace("{{fevnt_list}}", &str);
}

async fn fevnt_create() -> String {

    let vm = c1_get_all_valeurmetier_no_limit().await;

    let mut str = String::new();
    
    for m in vm {
        let new = format!(" <option value=\"{}\">#{} {}</option>", m.valeur_id, m.mission_id, m.valeur_name);
        str.push_str(&new);
    }


    return fs::read_to_string("html/c1/create-fevnt.html").unwrap()
        .replace("{{vm_list}}", &str);
}

async fn gaps() -> String {

    let all = c1_get_all_gaps().await;

    let mut str = String::new();
    let base = fs::read_to_string("html/c1/files/gaps-solo.html").unwrap();

    for m in all {
        let new = base.replace("{{gap_id}}", &m.gap_id.to_string())
            .replace("{{referential_type}}", &m.referential_type)
            .replace("{{referential_name}}", &m.referential_name.to_string())
            .replace("{{application_state}}", &m.application_state.to_string())
            .replace("{{gap}}", &m.gap)
            .replace("{{gap_justification}}", &m.gap_justification)
            .replace("{{proposed_measures}}", &m.proposed_measures);

        str.push_str(&new);
    }


    return fs::read_to_string("html/c1/list-gaps.html").unwrap()
        .replace("{{gaps_list}}", &str);
}

async fn gaps_create() -> String {
    return fs::read_to_string("html/c1/create-gaps.html").unwrap();
}

async fn gaps_detail(gaps_id:i32) -> String {
    let g = c1_get_gaps_by_id(gaps_id).await;

    if g.len() == 0 {
        return "__404".to_string();
    }

    let g = &g[0];

    return fs::read_to_string("html/c1/detail-gaps.html").unwrap()
        .replace("{{referential_type}}", &g.referential_type)
        .replace("{{referential_name}}", &g.referential_name)
        .replace("{{application_state}}", &g.application_state.to_string())
        .replace("{{gap}}", &g.gap)
        .replace("{{gap_justification}}", &g.gap_justification.replace("\n", "<br>"))
        .replace("{{proposed_measures}}", &g.proposed_measures.replace("\n", "<br>"))
        .replace("{{gap_id}}", &gaps_id.to_string());
}



