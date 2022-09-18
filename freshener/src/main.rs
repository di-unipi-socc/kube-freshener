mod k8s_types;
mod yaml_handler;

use std::{path::{Path}, fs};
use crate::k8s_types::*;
use walkdir::WalkDir;

fn main() {
    let mut manifests: Vec<K8SManifest> = Vec::new();

    // TODO: read all manifests inside the folder "manifests"
    // and parse them into manifests vector so that we can analyze them

    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
            let filename = entry.file_name().to_string_lossy();

            if filename.ends_with(".yaml") {
                println!("[*] Parsing {}", filename);
                let path = entry.path();
                let ref manifest_string = fs::read_to_string(path)
                    .expect(&filename.to_string());

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
            }
    }

    println!("[*] Parsing done");
    // let containers = &manifest.spec.containers;
    // if let Some(containers) = containers {
    //     println!("CONTAINERS:\n{:?}", *containers);
    // }
    println!("{:#?}", manifests);
    
}

