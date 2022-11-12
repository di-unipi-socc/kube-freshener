use std::{collections::HashMap, borrow::Borrow};

use colored::Colorize;

use crate::{k8s_types::*, yaml_handler};

pub fn check_wobbly_interaction(
    manifests: &Vec<K8SManifest>,
    is_to_refactor: bool
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
                "{}\n(*) Service named {} is reached by another service \n\
                without any circuit breaker or timeout. \n\
                {} solve it by adding circuit_breaker and/or and timeout in between .\n",
                format!("! [Wobbly Interaction]").red().bold(),
                format!("{}", invoked_service).cyan().bold(),
                format!("\nHint:").yellow().italic(),
            );

            if is_to_refactor {
                yaml_handler::create_virtual_service(invoked_service.clone());
            }
        }
    }
}

pub fn check_endpoint_based_interaction(
    manifests: &Vec<K8SManifest>,
    is_to_refactor: bool
) {
    let mut microservices_hashmap: HashMap<String, Microservice> = HashMap::new();

    let config = yaml_handler::get_config();

    for invoked_service in &config.invoked_services[..] {
        if let Some(deployment) = yaml_handler::
            get_deployment_named(
                invoked_service.clone(),
                manifests
        ) {

            let microservice = Microservice {
                has_service: false,
                has_direct_access: yaml_handler::
                    deployment_has_direct_access(deployment)
            };

            microservices_hashmap.insert(
                invoked_service.clone(),
                microservice
            );
        }
    }

    // iterate through k8s services and link them
    // to appropriate nodes in node_hashmap
    let services_manifests = yaml_handler::
        get_services(manifests);
    let deployments_manifests = yaml_handler::
        get_deployments_pods(manifests);

    for service_manifest in &services_manifests {
        if let Some(selector) = &service_manifest.spec.selector {

            let app_name = selector.app.clone();
            let service_name = selector.service.clone();
            let mut name = String::from("");

            if !app_name.is_none() {
                name = app_name.unwrap();
            } else if !service_name.is_none() {
                name = service_name.unwrap();
            }

            if !name.is_empty() { 
                // if exists a service with the selector.app = tosca service name
                
                if let Some(node) = microservices_hashmap
                    .get(&*name) {
                    // set the bool as true so that we can identify tosca services that have
                    // an attached k8s service
                    let updated_microservice = Microservice {
                        has_service: true,
                        has_direct_access: node.has_direct_access
                    };
                    microservices_hashmap.insert(
                        name.to_string(),
                        updated_microservice
                    );
                }
            }
        }
    }

    for invoked_service in &config.invoked_services[..] {
        if config.ignore_smells.endpoint_based_interaction.contains(invoked_service) { continue }
        if let Some(dest_node) = microservices_hashmap.get(invoked_service) {
            // We need to assure that the only way to access
            // B is through k8s services, so we have to check that 
            // the node.has_service is true and we also have to 
            // check that the service named node_name has not in the manifest
            // any hostPort or hostNetwork
            if dest_node.has_direct_access {
                // possible smell
                println!(
                    "{}(*) Service named {} is an invoked service, \n\
                    but it is reachable directly by using a host port \n\
                    you declared. {} remove every host network and host port\n",
                    format!("! [Endpoint Based Interaction]\n").red().bold(),
                    format!("{}", invoked_service).cyan().bold(),
                    format!("\nHint:").yellow().italic()
                );

                if is_to_refactor {
                    if let Some(mut invoked_service_manifest) = deployments_manifests.clone()
                    .into_iter()
                    .find(|man| man.metadata.name == *invoked_service) {
                        // * Removing every host network or host port

                        invoked_service_manifest.spec.hostNetwork = None;

                        // pod case
                        if let Some(containers) = &invoked_service_manifest.spec.containers {
                            let mut pod_refactored_containers: Vec<Container> = containers.clone();
                            for container in containers { 
                                let mut c = container.clone();
                                let has_host_ports = !&container.ports.is_none() && container.ports.as_ref()
                                    .unwrap()
                                    .into_iter()
                                    .any(|port| !port.hostPort.is_none());

                                if has_host_ports { c.ports = None }
                                pod_refactored_containers.push(c);
                            }
                            invoked_service_manifest.spec.containers = Some(pod_refactored_containers);
                        }

                        // deployment case
                        if let Some(template) = &invoked_service_manifest.spec.template {
                            if let Some(containers) = &template.spec.containers {
                                let mut depl_refactored_containers: Vec<Container> = Vec::new();
                                for container in containers {
                                    let mut c = container.clone();
                                    let has_host_ports = !&container.ports.is_none() && container.ports.as_ref()
                                        .unwrap()
                                        .into_iter()
                                        .any(|port| !port.hostPort.is_none());

                                    if has_host_ports { c.ports = None }
                                    // println!("Pushing container: {:#?}", c);
                                    depl_refactored_containers.push(c);
                                }
                                let mut temp = template.clone();
                                temp.spec.containers = Some(depl_refactored_containers);
                                invoked_service_manifest.spec.template = Some(temp);
                            }
                        }
                    
                        let filename = format!("{}{}", invoked_service_manifest.metadata.name, ".yaml");
                        
                        yaml_handler::update_manifest(&invoked_service_manifest, filename);
                        
                    }
                }
                
            }

            if !dest_node.has_service {
                // possible smell
                println!(
                    "{}(*) Service named {} is reached by another microservice,\n\
                    but there's no k8s service associated with it.\n\
                    {} remove every host network and host port and use a k8s \n\
                    service instead.\n",
                    format!("! [Endpoint Based Interaction]\n").red().bold(),
                    format!("{}", invoked_service).cyan().bold(),
                    format!("\nHint:").yellow().italic(),
                );

                if is_to_refactor {
                    yaml_handler::create_service_from(invoked_service.to_string());
                }
            }
        }
    }
}

pub fn check_no_apigateway(manifests: &Vec<K8SManifest>, is_to_refactor: bool) {
    let deployment_manifest = yaml_handler::get_deployments_pods(manifests);

    for mut manifest in deployment_manifest {
        /* 
        if hostNetwork is set as true or inside a container there's ports.-hostPort,
        and there's no image that represent an official Docker image that implements
        message routing components then a horizontal scalability violation can occur
        */
        let containers = &manifest.spec.containers;
        
        let host_network: bool = if let Some(hn) = &manifest.spec.hostNetwork { *hn } else { false };

        if is_to_refactor && host_network {
            manifest.spec.hostNetwork = None;
            let filename = format!("{}{}", manifest.metadata.name, ".yaml");
            // println!("{} has been modified with\n{:#?}", filename, man);
            yaml_handler::update_manifest(&manifest, filename); 
        }

        // if manifest represents a pod
        if let Some(conts) = containers {
            let result = analyze_containers_nag(&manifest, &conts, host_network);

            if result.1 && is_to_refactor {
                let filename = format!("{}{}", manifest.metadata.name, ".yaml");
                let mut manifest_cpy = manifest.clone();
                manifest_cpy.spec.containers = Some(result.0);
                
                if result.2 {
                    manifest_cpy.spec.hostNetwork = None;
                }
                
                // println!("{} has been modified with\n{:#?}", filename, man);
                yaml_handler::update_manifest(&manifest_cpy, filename); 
            }

            return
        }
        
        // if manifest represents a deployment
        if let Some(template) = &manifest.spec.template {
            if let Some(nested_containers) = &template.spec.containers {
                let result = analyze_containers_nag(&manifest, &nested_containers, host_network);

                if result.1 && is_to_refactor {
                    let filename = format!("{}-Deployment{}", manifest.metadata.name, ".yaml");
                    let mut manifest_cpy = manifest.clone();

                    if result.2 {
                        manifest_cpy.spec.hostNetwork = Some(false);
                    }
                    
                    if let Some(template) = manifest.spec.template {
                        let _templae_spec = TemplateSpec {
                            initContainers: template.spec.initContainers,
                            containers: Some(result.0),
                            volumes: template.spec.volumes
                        };

                        let _template = Template { spec: _templae_spec, metadata: template.metadata };
                        manifest_cpy.spec.template = Some(_template);
                        manifest_cpy.spec.containers = None;
                    }
                    
                    println!("UPDATING MANIFEST IN {} WITH:\n{:#?}", filename, manifest_cpy);

                    // println!("{} has been modified with\n{:#?}", filename, man);
                    yaml_handler::update_manifest(&manifest_cpy, filename); 
                }
            }
        }
    }
}

pub fn check_independent_depl(manifests: &Vec<K8SManifest>, is_to_refactor: bool) {

    let deployment_manifests = yaml_handler::get_deployments_pods(manifests);

    for manifest in deployment_manifests {

        let mut manifest_cpy = manifest.clone();
        let filename = format!("{}{}", manifest.metadata.name, ".yaml");
        let containers = &manifest.spec.containers;

        // checking independent deployability
        if manifest.kind == "Pod" {
            if let Some(mut containers) = containers.clone() {
                let refactored_containers = analyze_multiple_containers(&mut containers, manifest.metadata.name.clone(), is_to_refactor);
                manifest_cpy.spec.containers = Some(refactored_containers);
             }
        } else if manifest.kind == "Deployment" {
            if let Some(template) = manifest.spec.template {
                if let Some(mut nested_containers) = template.spec.containers {
                    let refactored_containers = analyze_multiple_containers(&mut nested_containers, manifest.metadata.name.clone(), is_to_refactor);
                    
                    let _spec = TemplateSpec {
                        initContainers: template.spec.initContainers,
                        containers: Some(refactored_containers),
                        volumes: template.spec.volumes
                    };
                    let _template = Template {
                        spec: _spec,
                        metadata: template.metadata
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
            yaml_handler::update_manifest(&manifest_cpy, filename);
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
        let node_config_element = config.ignore_smells
            .multiple_container
            .iter()
            .find(
                |c| c.name == metadata_name
            );

        let has_pattern = get_patterns().iter()
            .any(|pattern| -> bool {
                container.name.contains(pattern) 
                || container.image.contains(pattern)
            });
    
        let mut has_known_sidecar: bool = false;
        
        if let Some(node_element) = node_config_element {
            if node_element.containers.is_none() { 
                has_known_sidecar = false 
            } else {
                has_known_sidecar = node_element.containers
                .as_ref()
                .unwrap()
                .iter()
                .any(|c| *c == container.name);
            }
        }
                
        if !(has_pattern || has_known_sidecar) {
            if !main_container_name.is_empty() {
                println!(
                    "{}{}\n(*) Container named {} may not be a sidecar, \n\
                    we cannot assure {} is a proper sidecar.\n",
                    format!("! [Multiple containers per Deployment] => ").red().bold(),
                    format!("in {}", metadata_name).yellow().bold(),
                    format!("{}", container.name).cyan().bold(),
                    format!("{}", container.image).bright_purple().bold(),
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

fn analyze_containers_nag(manifest: &K8SManifest, containers: &Vec<Container>, host_network: bool) -> (Vec<Container>, bool, bool) {
    let mut result_containers: Vec<Container> = Vec::new();
    let mut has_to_update = false;
    let mut remove_host_network = false;

    for container in containers {
        let mut c = container.clone();
        if host_network && !implements_message_routing(
            manifest.metadata.name.clone(),
            container.image.clone()
        ) {
            println!(
                "{}{}\n(*) HostNetwork is set to true and container's (named '{}'), \n\
                image '{}' may not implement message routing.\n",
                format!("! [No API Gateway] => ").red().bold(),
                format!("in {}", &manifest.metadata.name).yellow().bold(),
                format!("{}", container.name).cyan().bold(), 
                format!("{}", container.image).bright_purple().bold(),
            );

            has_to_update = true;
            remove_host_network = true;
        }

        if let Some(ports) = &container.ports {
            // check if the current container has at least one host port
            let has_host_port = ports
                .into_iter()
                .any(|port| !port.hostPort.is_none());

            // if it's true, then we have to verify that the current container is running
            // an official Docker image that implements message routing
            if has_host_port && !implements_message_routing(
                manifest.metadata.name.clone(),
                container.image.clone()
            ) {
                println!(
                    "{}{}\n(*) Container named '{}' has an hostPort associated, \n\
                    and its image '{}' may not implement message routing.\n",
                    format!("! [No API Gateway] => ").red().bold(),
                    format!("in {}", &manifest.metadata.name).yellow().bold(),
                    format!("{}", container.name).cyan().bold(),
                    format!("{}", container.image).bright_purple().bold(),
                );

                c.ports = None;

                has_to_update = true;
            }
        }

        result_containers.push(c);
    }

    (result_containers, has_to_update, remove_host_network)

}

fn implements_message_routing(
    pod_name: String,
    image_name: String
) -> bool {
    if let Some(node_config_element) = yaml_handler
        ::get_config()
        .ignore_smells
        .noapigateway
        .iter()
        .find(|c| c.name == pod_name) {
            //ritorna vero nei casi in cui nel 
            // config si ha - name: catalogue
            // senza che vengano specificati 
            // container; questo significa che 
            // dobbiamo ignorare tutti i container
            // dentro catalogue
            if node_config_element.containers.is_none() {
                return true 
            }
            return node_config_element
                .containers
                .as_ref()
                .unwrap()
                .iter()
                .any(|c| image_name.contains(&*c)) 
    }

    false
}