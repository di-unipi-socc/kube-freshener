use crate::{k8s_types::*, yaml_handler};
use serde::{Deserialize, Serialize};
use std::{fs, io::Write};
use walkdir::WalkDir;

const KNOWN_IMAGES_PATH: &str = "./known-images.yaml";

#[derive(Debug, Deserialize, Serialize)]
pub struct IgnoreItem {
    pub name: String,
    pub image: String,
    pub kind: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct IgnoreList {
    sidecars: Vec<IgnoreItem>,
}

/// It filters deployment or pod manifests from all the manifests declared
pub fn get_deployments_pods(manifests: &Vec<K8SManifest>) -> Vec<K8SManifest> {
    let deployment_manifests = manifests
        .into_iter()
        .filter(|man| man.kind == "Deployment" || man.kind == "Pod")
        .map(|man| man.clone())
        .collect();

    deployment_manifests
}

/// It read recursively all the k8s manifests inside the 'manifests' folder
pub fn parse_manifests(manifests: &mut Vec<K8SManifest>) {
    for entry in WalkDir::new("./manifests")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let filename = entry.file_name().to_string_lossy();

        if filename.ends_with(".yaml") {
            println!("[*] Parsing {}", filename);
            let path = entry.path();
            let ref manifest_string = fs::read_to_string(path).expect(&filename.to_string());

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

    println!("[*] Parsing done\n");
}

/// It prints out the knwon-images list
pub fn read_ignore_list() {
    let converted_ignore_list: IgnoreList = internal_read(KNOWN_IMAGES_PATH.to_owned());

    println!("{:#?}", converted_ignore_list);
}

/// It adds an IngoreItem to the known-images list
pub fn add_ignore(item: IgnoreItem) {
    let mut converted_ignore_list: IgnoreList = internal_read(KNOWN_IMAGES_PATH.to_owned());

    println!(
        "[*] Adding ({}, {}, {}) as an ignore item",
        item.name, item.image, item.kind
    );

    converted_ignore_list.sidecars.push(item);

    update_ignore(converted_ignore_list);
}

/// It deletes an IngoreItem from the known-images list
pub fn delete_ignore(name: String) {
    let mut converted_ignore_list: IgnoreList = internal_read(KNOWN_IMAGES_PATH.to_owned());

    converted_ignore_list.sidecars = converted_ignore_list.sidecars
        .into_iter()
        .filter(|item| item.name != name)
        .collect();

    update_ignore(converted_ignore_list);
}

fn update_ignore(converted_ignore_list: IgnoreList) {
    let yaml = serde_yaml::to_string(&converted_ignore_list).unwrap();
    let mut f = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(KNOWN_IMAGES_PATH)
        .expect("Unable to open the file");
    f.write_all(yaml.as_bytes()).expect("Unable to write all");
    f.flush().expect("Unable to flush");
    fs::write(KNOWN_IMAGES_PATH, yaml).expect("Unable to write file");
}

/// It reads a file and then tries to parse to a DeserializeOwned T
fn internal_read<T: serde::de::DeserializeOwned>(filename: String) -> T {
    let ref file_string = fs::read_to_string(filename).expect("Expecting known-images.yaml exists");

    let converted_object: T = serde_yaml::from_str(file_string).unwrap();

    converted_object
}

/// It takes a k8s manifest and split it into a vector whenever it founds '---' separator
fn unpack(manifest: &String) -> Vec<String> {
    /*
        Let's split the manifest using "---"
        as a separator
    */
    let split = manifest.split("---");
    let vec = split.map(|x| x.to_owned()).collect();

    vec
}
