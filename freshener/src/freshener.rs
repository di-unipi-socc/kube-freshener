use std::{collections::HashMap};

use crate::{k8s_types::*, tosca_types::*, yaml_handler};

const DATASTORE_TYPE: &str = "micro.nodes.Datastore";

struct K8sToscaNode {
    kind: String,
    has_service: bool
}

pub fn check_endpoint_based_interaction(
    manifests: &Vec<K8SManifest>, 
    nodes: &Vec<NodeTemplate> 
) {
    let mut node_hashmap: HashMap<String, (String, bool)> = HashMap::new();

    // iterate over nodes
    // save nodes types
    for node in nodes {
        if let Some(name) = &node.name {
            if let Some(kind) = &node.kind {
                node_hashmap.insert(name.to_string(), (kind.to_string(), false));
            }
        }
    }

    // iterate through k8s services and link them
    // to appropriate nodes in node_hashmap
    let services = yaml_handler::get_services(manifests);

    for service in services {
        if let Some(selector) = service.spec.selector {
            if let Some(name) = selector.app {
                // if exists a service with the selector.app = tosca service name
                if let Some(node) = node_hashmap.get(&name) {
                    // set the bool as true so that we can identify tosca services that have
                    // an attached k8s service
                    node_hashmap.insert(name, (node.0.clone(), true));
                }
            }
        }
    }

    // re-iterate over nodes
    for node in nodes {

        // iterate over interactions
        if let Some(requirements) = &node.requirements {
            for requirement in requirements {
                let mut node_name = String::new();

                if let Some(interaction) = &requirement.interaction {
                    match interaction {
                        Interaction::String(val) => node_name = val.to_string(),
                        Interaction::DetailedInteraction(val) => {
                            if let Some(detailed_node) = &val.node {
                                node_name = detailed_node.to_string();
                            }
                        }
                    }
                }
                
                // if an interaction is in the hashmap, then insert 
                // and verify that it is not a micro.nodes.Datastore node
                if let Some(node_name) = &node.name {
                    if let Some(node) = node_hashmap.get(&node_name.to_string()) {
                        if node.0 != DATASTORE_TYPE {
                            // now we know that this interaction is not
                            // a Datastore object

                            // next step: We need to ensure that the only way to access
                            // B is through k8s services, so we have to check that 
                            // the node.has_service (node.1) is true and we also have to 
                            // check that the service
                            
                        }
                    }
                }
                
            }
        }
    }
}

pub fn check_no_apigateway(manifests: &Vec<K8SManifest>) {
    let deployment_manifest = yaml_handler::get_deployments_pods(manifests);

    for manifest in deployment_manifest.into_iter() {
        /* 
        if hostNetwork is set as true or inside a container there's ports.-hostPort,
        and there's no image that represent an official Docker image that implements
        message routing components then a horizontal scalability violation can occur
        */
        let containers = &manifest.spec.containers;
        // TODO: do the host_network check
        let host_network: bool = if let Some(hn) = &manifest.spec.hostNetwork { *hn } else { false };

        if let Some(conts) = containers {
            analyze_containers_nag(&conts, host_network);
        }
        
        if let Some(template) = manifest.spec.template {
            if let Some(spec) = template.spec {
                if let Some(nested_containers) = spec.containers {
                    analyze_containers_nag(&nested_containers, host_network);
                }
            }
        }
        
    }
    println!("\n");
}

pub fn check_independent_depl(manifests: &Vec<K8SManifest>) {

    let deployment_manifests = yaml_handler::get_deployments_pods(manifests);

    for manifest in deployment_manifests {
        let containers = &manifest.spec.containers;

        // checking independent deployability
        if let Some(containers) = containers {
            analyze_containers_mspc(containers);
        }

        if let Some(template) = manifest.spec.template {
            if let Some(spec) = template.spec {
                if let Some(nested_containers) = spec.containers {
                    analyze_containers_mspc(&nested_containers);
                }
            }
        }
    }
}

fn analyze_containers_mspc(containers: &Vec<Container>) {
    let mut main_container_name = String::new();
    for container in containers {
        let has_pattern = get_patterns().iter()
            .any(|pattern| -> bool {
                container.name.contains(pattern) || container.image.contains(pattern)
            });
    
        let has_known_sidecar =  yaml_handler::get_known_imgaes().iter()
            .any(|i| -> bool {
                i.kind == "sidecar" && container.image.contains(&i.image)
            });
                
        if !(has_pattern || has_known_sidecar) {
            if !main_container_name.is_empty() {
                println!(
                    "[Smell occurred - Independent Deployability]\nContainer named {} may not be a sidecar, \
                    because it has {} as an image,\nso we cannot ensure that this container is a proper sidecar. \
                    Therefore it can potentially violate the Independent Deployability rule\n",
                    container.name, container.image
                );
                continue;
            } 
            main_container_name = container.name.clone();
        }
    }
}

fn analyze_containers_nag(containers: &Vec<Container>, host_network: bool) {
    for container in containers {
        if host_network && !implements_message_routing(container.image.clone()) {
            println!(
                "[Smell occurred - No API Gateway]\nHostNetwork is set to true and container's (named '{}'), \
                image '{}' may not be a proper message routing implementation and \
                this could be a potential no api gateway smell.\nIf you were to be sure that \
                your image implements message routing, then we suggest you to add the image \
                in the ignore list using cargo run add-ignore <name> <image> <kind>.\n",
                container.name, container.image
            );
        }

        if let Some(ports) = &container.ports {
            // check if the current container has at least one host port
            let has_host_port = ports.into_iter().any(|port| !port.hostPort.is_none());

            // if it's true, then we have to verify that the current container is running
            // an official Docker image that implements message routing
            if has_host_port && !implements_message_routing(container.image.clone()) {
                println!(
                    "[Smell occurred - No API Gateway]\nContainer named '{}' has an hostPort associated, \
                    the container's image '{}' may not be a proper message routing implementation and \
                    this could be a potential no api gateway smell.\nIf you were to be sure that \
                    your image implements message routing, then we suggest you to add the image \
                    in the ignore list using cargo run add-ignore <name> <image> <kind>.\n",
                    container.name, container.image
                );
            }
        }
    }
}

fn implements_message_routing(image_name: String) -> bool {
    yaml_handler::get_known_imgaes().into_iter().any(|i| i.kind == "mr" && i.image == image_name)
}