// The web controler for the C1 route

use std::fs;
use crate::helper::database::{c1_get_all_missions, c1_get_mission_by_id, c1_get_all_valeurmetier, c1_get_valermetier_by_id, c1_get_asset_by_vmid};


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
    }else if path.starts_with("c1/asset/create/") {
        let vm_id = path.replace("c1/asset/create/", "");
        return asset_create(vm_id.parse::<i32>().unwrap_or(0)).await;
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
