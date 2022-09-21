use crate::{k8s_types::*, yaml_handler};

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
            analyze_containres_nag(&conts);
        }

        check_deployment_specs(manifest, |c| analyze_containres_nag(c)); 
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

        check_deployment_specs(manifest, |c| analyze_containers_mspc(c));
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

fn analyze_containres_nag(containers: &Vec<Container>) {
    for container in containers {
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

fn check_deployment_specs(manifest: K8SManifest, func: fn(&Vec<Container>)) {
    if let Some(template) = manifest.spec.template {
        if let Some(spec) = template.spec {
            if let Some(nested_containers) = spec.containers {
                func(&nested_containers);
            }
        }
    }
}

fn implements_message_routing(image_name: String) -> bool {
    yaml_handler::get_known_imgaes().into_iter().any(|i| i.kind == "mr" && i.image == image_name)
}