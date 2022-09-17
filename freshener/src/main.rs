mod k8s_types;
mod yaml_handler;

use std::{path::Path, fs};
use crate::k8s_types::*;
use crate::yaml_handler::*;

fn main() {
    let mut manifests: Vec<K8SManifest> = Vec::new();

    // Create a path to the desired file
    let path = Path::new("./test-manifests/indepDepl.yaml");
    let ref manifest_string = fs::read_to_string(path)
        .expect("Unable to read manifest");
    
    /*
    Case when we have a manifest which declares different k8s components
    separated by "---"
    */ 
    let ref sub_manifests = yaml_handler::unpack(manifest_string);
    
    // deserializing manifests
    for m in sub_manifests {
        let converted_manifest: K8SManifest = serde_yaml::from_str(&m).unwrap();
        manifests.push(converted_manifest)
    }

    
    // let containers = &manifest.spec.containers;
    // if let Some(containers) = containers {
    //     println!("CONTAINERS:\n{:?}", *containers);
    // }
    println!("{:?}", manifests);
    
}

