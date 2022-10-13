use std::{collections::HashMap};

use colored::Colorize;

use crate::{k8s_types::*, yaml_handler};

pub fn check_wobbly_interaction(
    manifests: &Vec<K8SManifest>
) {
    let ref virtual_services = yaml_handler::get_virtual_services(manifests);
    let ref dest_rules = yaml_handler::get_destination_rules(manifests);
    let config = yaml_handler::get_config();

    for invoked_service in &config.invoked_services[..] {        
        // given the destination node I have to check if there is a virtual service
        // having spec.hosts = dest_node_name or a destination rule having
        // spec.host = dest_node_n
        let has_virtual_service = virtual_services
            .into_iter()
            .any(|m| {
                if let Some(hosts) = &m.spec.hosts {
                    return hosts
                    .into_iter()
                    .any(|h| &*h == invoked_service)
                }

                false
            });

        let has_outlier_detection = dest_rules
            .into_iter()
            .any(|m| {
                if let (Some(host), Some(traffic_policy)) = (&m.spec.host, &m.spec.trafficPolicy) {
                    return &*host == invoked_service && !traffic_policy.outlier_detection.is_none()
                }

                false
            });

        if !has_virtual_service && !has_outlier_detection {
            println!(
                "{}{}\nService named {} is reached by another service \
                without any circuit breaker or timeout: resolve the occurrence of wobbly \
                service interactions smells by adding circuit_breaker and/or and timeout in between .\n",
                format!("[Smell occurred - Wobbly Interaction]\n").red().bold(),
                format!("[Metadata name: {}]", invoked_service).yellow().bold(),
                invoked_service
            );
        }
    }
}

pub fn check_endpoint_based_interaction(
    manifests: &Vec<K8SManifest>
) {
    let mut microservices_hashmap: HashMap<String, Microservice> = HashMap::new();

    let config = yaml_handler::get_config();

    for invoked_service in &config.invoked_services[..] {
        if let Some(deployment) = yaml_handler::get_deployment_named(invoked_service.clone(), manifests) {
            let microservice = Microservice {
                has_service: false,
                has_direct_access: yaml_handler::deployment_has_direct_access(deployment)
            };

            microservices_hashmap.insert(invoked_service.clone(), microservice);
        }
    }

    // iterate through k8s services and link them
    // to appropriate nodes in node_hashmap
    let services = yaml_handler::get_services(manifests);

    for service in services {
        if let Some(selector) = service.spec.selector {
            if let Some(name) = selector.service {
                // if exists a service with the selector.app = tosca service name
                if let Some(node) = microservices_hashmap.get(&name) {
                    // set the bool as true so that we can identify tosca services that have
                    // an attached k8s service
                    let updated_microservice = Microservice {
                        has_service: true,
                        has_direct_access: node.has_direct_access
                    };
                    microservices_hashmap.insert(name, updated_microservice);
                }
            }
        }
    }

    for invoked_service in &config.invoked_services[..] {
        if let Some(dest_node) = microservices_hashmap.get(invoked_service) {
            // We need to assure that the only way to access
            // B is through k8s services, so we have to check that 
            // the node.has_service is true and we also have to 
            // check that the service named node_name has not in the manifest
            // any hostPort or hostNetwork
            if dest_node.has_direct_access {
                // possible smell
                println!(
                    "{}Service named {} is an invoked service, \
                    but it is direct reachable using a host port that you declared. \
                    To solve this smell please remove any host network and host port\n",
                    format!("[Smell occurred - Endpoint Based Interaction]\n").red().bold(),
                    format!("{}", invoked_service).yellow().bold()
                );
            }

            if !dest_node.has_service {
                // possible smell
                println!(
                    "{}Service named {} is reached by another microservice, \
                    but there's no k8s service associated with it. \
                    Therefore destination service could be reached with a hardcoded address. \
                    To solve this smell please remove any host network and host port and use a k8s \
                    service instead .\n",
                    format!("[Smell occurred - Endpoint Based Interaction]\n").red().bold(),
                    format!("{}", invoked_service).yellow().bold()
                );
            }
        }
    }
}

pub fn check_no_apigateway(manifests: &Vec<K8SManifest>, solve: bool) {
    let deployment_manifest = yaml_handler::get_deployments_pods(manifests);

    for mut manifest in deployment_manifest {
        /* 
        if hostNetwork is set as true or inside a container there's ports.-hostPort,
        and there's no image that represent an official Docker image that implements
        message routing components then a horizontal scalability violation can occur
        */
        let containers = &manifest.spec.containers;
        
        let host_network: bool = if let Some(hn) = &manifest.spec.hostNetwork { *hn } else { false };

        if solve && host_network {
            manifest.spec.hostNetwork = None;
            let filename = format!("{}{}", manifest.metadata.name, ".yaml");
            // println!("{} has been modified with\n{:#?}", filename, man);
            yaml_handler::update_manifest(manifest.clone(), filename); 
        }

        // if manifest represents a pod
        if let Some(conts) = containers {
            let result = analyze_containers_nag(&manifest, &conts, host_network);

            if result.1 && solve {
                let filename = format!("{}{}", manifest.metadata.name, ".yaml");
                let mut manifest_cpy = manifest.clone();
                manifest_cpy.spec.containers = Some(result.0);
                // println!("{} has been modified with\n{:#?}", filename, man);
                yaml_handler::update_manifest(manifest_cpy, filename); 
            }

            return
        }
        
        // if manifest represents a deployment
        if let Some(template) = &manifest.spec.template {
            if let Some(nested_containers) = &template.spec.containers {
                let result = analyze_containers_nag(&manifest, &nested_containers, host_network);
                
                if result.1 && solve {
                    let filename = format!("{}{}", manifest.metadata.name, ".yaml");
                    let mut manifest_cpy = manifest.clone();
                    
                    if let Some(template) = manifest.spec.template {
                        let _templae_spec = TemplateSpec {
                            initContainers: template.spec.initContainers,
                            containers: Some(result.0),
                            volumes: template.spec.volumes
                        };
                        let _template = Template { spec: _templae_spec };
                        manifest_cpy.spec.template = Some(_template);
                        manifest_cpy.spec.containers = None;
                    }
                    
                    // println!("{} has been modified with\n{:#?}", filename, man);
                    yaml_handler::update_manifest(manifest_cpy, filename); 
                }
            }
        }
    }
}

pub fn check_independent_depl(manifests: &Vec<K8SManifest>, is_to_refactor: bool) {

    let deployment_manifests = yaml_handler::get_deployments_pods(manifests);

    for manifest in deployment_manifests {
        if manifest.metadata.name != "catalogue" { continue; }
        let mut manifest_cpy = manifest.clone();
        let filename = format!("{}{}", manifest.metadata.name, ".yaml");
        let containers = &manifest.spec.containers;

        // checking independent deployability
        if manifest.kind == "Pod" {
            if let Some(mut containers) = containers.clone() {
                let refactored_containers = analyze_multiple_containers(&mut containers, manifest.metadata.name.clone(), is_to_refactor);
                manifest_cpy.spec.containers = Some(refactored_containers);
             }
        } else {
            if let Some(template) = manifest.spec.template {
                if let Some(mut nested_containers) = template.spec.containers {
                    let refactored_containers = analyze_multiple_containers(&mut nested_containers, manifest.metadata.name, is_to_refactor);
                    let _spec = TemplateSpec {
                        initContainers: template.spec.initContainers,
                        containers: Some(refactored_containers),
                        volumes: template.spec.volumes
                    };
                    let _template = Template {
                        spec: _spec
                    };
                    manifest_cpy.spec.template = Some(_template);
                    manifest_cpy.spec.containers = None;
                }
            }
        }

        // manifest_cpy is the refactored manifest
        // now if solve flag is set as true we can
        // override the "manifest.metadata.name".yaml
        // with the refactored version
        if is_to_refactor {
            yaml_handler::update_manifest(manifest_cpy, filename);
        }

    }
}

/// it returns the refactored vector of containers
fn analyze_multiple_containers(containers: &Vec<Container>, metadata_name: String, is_to_refactor: bool) -> Vec<Container> {
    let mut main_container_name = String::new();
    let mut result_containers: Vec<Container> = containers.clone();

    // check config file
    let config = yaml_handler::get_config();

    for container in containers {
        let node_config_element = config.smells.multiple_container.iter().find(|c| c.name == metadata_name);
        let has_pattern = get_patterns().iter()
            .any(|pattern| -> bool {
                container.name.contains(pattern) || container.image.contains(pattern)
            });
    
        let mut has_known_sidecar: bool = false;
        
        if let Some(node_element) = node_config_element {
            if node_element.containers.is_none() { has_known_sidecar = false }
            else {
                has_known_sidecar = node_element.containers.as_ref().unwrap()
                .iter()
                .any(|c| *c == container.name);
            }
        }
                
        if !(has_pattern || has_known_sidecar) {
            if !main_container_name.is_empty() {
                println!(
                    "{}{}\nContainer named {} may not be a sidecar, \
                    because it has {} as an image,\nso we cannot ensure that this container is a proper sidecar. \
                    Therefore it can potentially violate the Independent Deployability rule\n",
                    format!("[Smell occurred - Multiple containers per Deployment]\n").red().bold(),
                    format!("[Metadata name: {}]", metadata_name).yellow().bold(),
                    container.name, container.image
                );

                // solving by creating a new pod named as the "wrong" container name
                // and with the same image
                if is_to_refactor {
                    yaml_handler::create_pod_from(container);
                }

                // then remove the "wrong" container from the current pod/deployment
                result_containers = result_containers
                    .into_iter()
                    .filter(|c| c.name != container.name)
                    .collect();
            
                continue;
            } 
            main_container_name = container.name.clone();
        }
    }

    result_containers
}

fn analyze_containers_nag(manifest: &K8SManifest, containers: &Vec<Container>, host_network: bool) -> (Vec<Container>, bool) {
    let mut result_containers: Vec<Container> = Vec::new();
    let mut has_to_update = false;

    for container in containers {
        let mut c = container.clone();
        if host_network && !implements_message_routing(manifest.metadata.name.clone(), container.image.clone()) {
            println!(
                "{}{}\nHostNetwork is set to true and container's (named '{}'), \
                image '{}' may not be a proper message routing implementation and \
                this could be a potential no api gateway smell.\n",
                format!("[Smell occurred - No API Gateway]\n").red().bold(),
                format!("[Metadata name: {}]", &manifest.metadata.name).yellow().bold(),
                container.name, 
                container.image
            );
        }

        if let Some(ports) = &container.ports {
            // check if the current container has at least one host port
            let has_host_port = ports.into_iter().any(|port| !port.hostPort.is_none());

            // if it's true, then we have to verify that the current container is running
            // an official Docker image that implements message routing
            if has_host_port && !implements_message_routing(manifest.metadata.name.clone(), container.image.clone()) {
                println!(
                    "{}{}\nContainer named '{}' has an hostPort associated, \
                    the container's image '{}' may not be a proper message routing implementation and \
                    this could be a potential no api gateway smell.\n",
                    format!("[Smell occurred - No API Gateway]\n").red().bold(),
                    format!("[Metadata name: {}]", &manifest.metadata.name).yellow().bold(),
                    container.name,
                    container.image,
                );

                c.ports = None;

                has_to_update = true;
            }
        }

        result_containers.push(c);
    }

    (result_containers, has_to_update)

}

fn implements_message_routing(pod_name: String, image_name: String) -> bool {
    if let Some(node_config_element) = yaml_handler::get_config()
        .smells
        .noapigateway
        .iter()
        .find(|c| c.name == pod_name) {
            // return true because this is the case when you have 
            // - name: catalogue, without containers
            // this means that we have to ignore all containers inside the manifest
            // called catalogue
            if node_config_element.containers.is_none() { return true }
            return node_config_element.containers.as_ref().unwrap()
                .iter()
                .any(|c| image_name.contains(&*c)) 
    }

    false
}