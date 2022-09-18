mod k8s_types;
mod yaml_handler;

use crate::{k8s_types::*, yaml_handler::parse_manifests};


fn main() {
    let mut manifests: Vec<K8SManifest> = Vec::new();

    parse_manifests(&mut manifests);

    println!("[*] Parsing done");
    // let containers = &manifest.spec.containers;
    // if let Some(containers) = containers {
    //     println!("CONTAINERS:\n{:?}", *containers);
    // }
    println!("{:#?}", manifests);
    
    }
