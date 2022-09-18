use std::{fs};
use walkdir::WalkDir;
use crate::{k8s_types::*, yaml_handler};

pub fn parse_manifests(manifests: &mut Vec<K8SManifest>) {
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
}

fn unpack(manifest: &String) -> Vec<String> {
    /*
        Let's split the manifest using "---"
        as a separator
    */
    let split = manifest.split("---");
    let vec = split
        .map(|x| x.to_owned())
        .collect();

    vec
}
