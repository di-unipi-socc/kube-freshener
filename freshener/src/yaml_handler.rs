use crate::{k8s_types::*, yaml_handler};
use crate::{config_type::*};
use std::fs::File;
use std::{fs, io::Write};
use walkdir::WalkDir;
use colored::Colorize;

const CONFIG_PATH: &str = "./config.yaml";

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

    if let Some(template) = deployment.spec.template {
        if let Some(containers) = template.spec.containers {
            for container in containers {
                if let Some(ports) = container.ports {
                    let has_host_port = ports.into_iter().any(|port| !port.hostPort.is_none());
                    if has_host_port {
                        return true
                    }
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
            return *d.metadata.name == name
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

pub fn get_config() -> Config {
    let converted_config: Config = internal_read(CONFIG_PATH.to_owned());

    converted_config
}

pub fn get_ignored_manifests() -> Vec<String> {
    get_config().ignored_manifests
}

pub fn create_virtual_service(depl_name: String) {
    let mut path = String::from("./manifests/Istio/");
    path.push_str(&depl_name);
    path.push_str("-virtual-service.yaml");

    let mut file = File::create(path)
        .expect("Error encountered while creating a new destination rule!");

    let vs = K8SManifest {
        api_version: String::from("networking.istio.io/v1alpha3"),
        kind: String::from("VirtualService"),
        metadata: Metadata { name: depl_name.clone() },
        spec: Spec { 
            initContainers: None,
            containers: None,
            volumes: None,
            template: None,
            hostNetwork: None,
            selector: None,
            hosts: Some(vec![depl_name.clone()]),
            host: None,
            trafficPolicy: None,
            replicas: None,
            restartPolicy: None
        }
    };

    let yaml = serde_yaml::to_string(&vs).unwrap();

    let res = file.write_all(yaml.as_bytes());

    if res.is_err() {
        println!("Error while writing a new virtual service");
    }
}

pub fn create_pod_from(container: &Container) {
    let mut path = String::from("./manifests/");
    path.push_str(&container.name);
    path.push_str(".yaml");
    
    let mut file = File::create(path)
        .expect("Error encountered while creating a new pod!");

    let manifest = K8SManifest {
        api_version: String::from("apps/v1"),
        kind: String::from("Pod"),
        metadata: Metadata { name: container.name.clone() },
        spec: Spec { 
            initContainers: None,
            containers: Some(vec![container.clone()]),
            volumes: None,
            template: None,
            hostNetwork: None,
            selector: None,
            hosts: None,
            host: None,
            trafficPolicy: None,
            replicas: None,
            restartPolicy: None
        }
    };

    let yaml = serde_yaml::to_string(&manifest).unwrap();

    let res = file.write_all(yaml.as_bytes());

    if res.is_err() {
        println!("Error while writing a new pod");
    }
}

pub fn create_service_from(name: String) {
    let mut path = String::from("./manifests/");
    path.push_str(&name);
    path.push_str("-srv");
    path.push_str(".yaml");

    let mut file = File::create(path)
        .expect("Error encountered while creating a new service!");

    let service_manifest = K8SManifest {
        api_version: "v1".to_string(),
        kind: "Service".to_string(),
        metadata: Metadata { name: name.clone() },
        spec: Spec { 
            initContainers: None,
            containers: None,
            volumes: None,
            template: None,
            hostNetwork: None,
            selector: Some(Selector {
                service: Some(name),
            }),
            hosts: None,
            host: None,
            trafficPolicy: None,
            replicas: None,
            restartPolicy: None
        }
    };

    let yaml = serde_yaml::to_string(&service_manifest).unwrap();
    
    let res = file.write_all(yaml.as_bytes());

    if res.is_err() {
        println!("Error raised while writing a new k8s service");
    }
}

pub fn update_manifest(manifest: &K8SManifest, filename: String) {

    for entry in WalkDir::new("./manifests")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_name = entry.file_name().to_string_lossy();
        let f = file_name.to_string();
        let path = entry.path().to_str();

        if f == filename {
            if let Some(path) = path {
                let yaml = serde_yaml::to_string(&manifest).unwrap();
                let mut f = fs::OpenOptions::new()
                    .write(true)
                        .truncate(true)
                        .open(path)
                        .expect("Unable to open the file");
                    f.write_all(yaml.as_bytes()).expect("Unable to write all");
                    f.flush().expect("Unable to flush");
                fs::write(path, yaml).expect("Unable to write file");
            
            }
        }
        
    }
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
