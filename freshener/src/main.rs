mod k8s_types;

use std::{path::Path, fs};
use crate::k8s_types::*;

fn main() {
    // Create a path to the desired file
    let path = Path::new("./test-manifests/indepDepl.yaml");
    let manifest_string = fs::read_to_string(path)
        .expect("Unable to read manifest");


    // deserializing the manifest
    let ref new_manifest: K8SManifest = serde_yaml::from_str(&manifest_string).unwrap();
    let containers = &new_manifest.spec.containers;
    if let Some(containers) = containers {
        println!("CONTAINERS:\n{:?}", *containers);
    }
    println!("{:?}", new_manifest);
    
}

