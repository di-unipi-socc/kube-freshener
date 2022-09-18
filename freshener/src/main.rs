mod k8s_types;
mod yaml_handler;

use crate::{k8s_types::K8SManifest};

fn main() {
    let mut manifests: Vec<K8SManifest> = Vec::new();

    yaml_handler::parse_manifests(&mut manifests);

    println!("[*] Parsing done");
    // let containers = &manifest.spec.containers;
    // if let Some(containers) = containers {
    //     println!("CONTAINERS:\n{:?}", *containers);
    // }

    check_independent_depl(manifests);
}

// TODO: Multiple services per container
fn check_independent_depl(manifests: Vec<K8SManifest>) {

    // filtering manifests, taking only the ones with kind as "Deployment" or "Pod"
    let deployment_manifests = manifests.iter()
    .filter(|man| man.kind == "Deployment" || man.kind == "Pod")
    .collect::<Vec<_>>();
    
    println!("{:#?}", deployment_manifests);
}
