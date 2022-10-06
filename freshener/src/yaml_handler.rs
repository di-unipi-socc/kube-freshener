use crate::{k8s_types::*, yaml_handler};
use crate::{tosca_types::*};
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::Path};
use walkdir::WalkDir;
use colored::Colorize;

const IGNORE_LIST_PATH: &str = "./ignore-list.yaml";
const TOSCA_PATH: &str = "./mTOSCA/mtosca.yaml";

#[derive(Debug, Deserialize, Serialize)]
pub struct KnownImage {
    pub name: String,
    pub image: String,
    pub kind: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct IgnoreList {
    images: Vec<KnownImage>,
    manifests: Vec<String>,
}

pub fn deployment_has_direct_access(deployment: K8SManifest) -> bool {

    if let Some(host_network) = deployment.spec.hostNetwork {
        if host_network {
            return true;
        }
    }

    if let Some(containers) = deployment.spec.containers {
        for container in containers {
            if let Some(ports) = container.ports {
                let has_host_port = ports.into_iter().any(|port| !port.hostPort.is_none());
                if has_host_port {
                    return true
                }
            }
        }
    }
    
    false
}

pub fn get_deployment_named(name: String, manifests: &Vec<K8SManifest>) -> Option<K8SManifest> {
    let deployments = get_deployments_pods(manifests);

    deployments
        .into_iter()
        .find(|d| {
            if let Some(m) = &d.metadata {
                return m.name == name
            }
            false
        })
}

/// It filters destination rules from all the manifests declared
pub fn get_destination_rules(manifest: &Vec<K8SManifest>) -> Vec<K8SManifest> {
    let v_services = manifest
        .into_iter()
        .filter(|man| man.kind == "DestinationRule")
        .map(|man| man.clone())
        .collect();

    v_services
}

/// It filters virtual services from all the manifests declared
pub fn get_virtual_services(manifest: &Vec<K8SManifest>) -> Vec<K8SManifest> {
    let v_services = manifest
        .into_iter()
        .filter(|man| man.kind == "VirtualService")
        .map(|man| man.clone())
        .collect();

    v_services
}

/// It filters services from all the manifests declared
pub fn get_services(manifests: &Vec<K8SManifest>) -> Vec<K8SManifest> {
    let services = manifests
        .into_iter()
        .filter(|man| man.kind == "Service")
        .map(|man| man.clone())
        .collect();

    services
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
        let f = filename.to_string();

        // Discard all manifests delcared in ignore-list.yaml
        if filename.ends_with(".yaml") && !get_ignored_manifests().contains(&f) {
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

    println!("{}", format!("[*] Parsing done\n").green().bold());
}

pub fn parse_tosca(nodes: &mut Vec<NodeTemplate>) {
    let path = Path::new(TOSCA_PATH);
    let ref tosca_string = fs::read_to_string(path).unwrap();
    let tosca_json = serde_yaml::from_str::<serde_json::Value>(&tosca_string).unwrap();

    if let Some(topology_template) = tosca_json
        .as_object()
        .unwrap()
        .get("topology_template") {

        if let Some(node_templates) = topology_template
            .as_object()
            .unwrap()
            .get("node_templates") {

                for (key, value) in node_templates.as_object().unwrap() {
                    let mut node_template: NodeTemplate = serde_json::from_value(value.clone()).unwrap();
                    node_template.name = Some(key.to_string());
                    nodes.push(node_template);
                }

            }
    }

}

/// It prints out the knwon-images list
pub fn read_known_imgaes() {
    let converted_sidecar_list: IgnoreList = internal_read(IGNORE_LIST_PATH.to_owned());

    println!("{:#?}", converted_sidecar_list.images);
}

/// It prints out the manifests ignore list
pub fn read_manifest_ignore() {
    let converted_manifests_list: IgnoreList = internal_read(IGNORE_LIST_PATH.to_owned());

    println!("{:#?}", converted_manifests_list.manifests);
}

pub fn get_known_imgaes() -> Vec<KnownImage> {
    let converted_manifests_list: IgnoreList = internal_read(IGNORE_LIST_PATH.to_owned());

    converted_manifests_list.images
}

pub fn get_ignored_manifests() -> Vec<String> {
    let converted_manifests_list: IgnoreList = internal_read(IGNORE_LIST_PATH.to_owned());

    converted_manifests_list.manifests
}

/// It adds an IngoreItem to the known-images list
pub fn add_known_image(item: KnownImage) {
    let mut converted_ignore_list: IgnoreList = internal_read(IGNORE_LIST_PATH.to_owned());

    println!(
        "[*] Adding ({}, {}, {}) as a known image",
        item.name, item.image, item.kind
    );

    converted_ignore_list.images.push(item);

    update_ignore(converted_ignore_list);
}

pub fn add_manifest_ignore(filename: String) {
    let mut converted_ignore_list: IgnoreList = internal_read(IGNORE_LIST_PATH.to_owned());

    println!("[*] Adding '{}' as a manifest to ignore", filename);

    converted_ignore_list.manifests.push(filename);

    update_ignore(converted_ignore_list);
}

/// It deletes an IngoreItem from the known-images list
pub fn delete_known_image(name: String) {
    let mut converted_ignore_list: IgnoreList = internal_read(IGNORE_LIST_PATH.to_owned());

    converted_ignore_list.images = converted_ignore_list.images
        .into_iter()
        .filter(|item| item.name != name)
        .collect();

    update_ignore(converted_ignore_list);
}

pub fn delete_manifest_ignore(name: String) {
    let mut converted_ignore_list: IgnoreList = internal_read(IGNORE_LIST_PATH.to_owned());

    converted_ignore_list.manifests = converted_ignore_list.manifests
        .into_iter()
        .filter(|item| *item != name)
        .collect();

    update_ignore(converted_ignore_list);
}

fn update_ignore(converted_ignore_list: IgnoreList) {
    let yaml = serde_yaml::to_string(&converted_ignore_list).unwrap();
    let mut f = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(IGNORE_LIST_PATH)
        .expect("Unable to open the file");
    f.write_all(yaml.as_bytes()).expect("Unable to write all");
    f.flush().expect("Unable to flush");
    fs::write(IGNORE_LIST_PATH, yaml).expect("Unable to write file");
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
