use crate::{k8s_types::*, yaml_handler};

// TODO: Expand the known sidecars list in k8s_types.rs
pub fn check_independent_depl(manifests: Vec<K8SManifest>) {

    // filtering manifests, taking only the ones with kind as "Deployment" or "Pod"
    let deployment_manifests = yaml_handler::get_deployments_pods(manifests);

    for manifest in deployment_manifests {
        let containers = &manifest.spec.containers;
        let mut found_main_container = false;
        if let Some(containers) = containers {
            for container in containers {
                let has_pattern = get_patterns().iter()
                    .any(|pattern| -> bool {
                        container.name.contains(pattern) || container.image.contains(pattern)
                    });
                
                let has_known_sidecar = get_known_sidecars().iter()
                    .any(|known_sidecar| -> bool {
                        container.image.contains(known_sidecar)
                    });

                if !(has_pattern || has_known_sidecar) {
                    if found_main_container {
                        println!(
                            "\t[Smell occurred] container named {} may not be a sidecar,", 
                            container.name,
                        );
                        println!(
                            "\tbecause it has {} as an image, so we cannot ensure that this container is a proper sidecar.",
                            container.image,
                        );
                        println!("\tTherefore it can potentially violate the Independent Deployability rule.");
                        continue;
                    } 
                    found_main_container = true;
                }
            }
        }
    }

}
