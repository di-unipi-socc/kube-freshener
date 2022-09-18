mod k8s_types;
mod yaml_handler;

use crate::{k8s_types::*};

fn main() {
    let mut manifests: Vec<K8SManifest> = Vec::new();

    yaml_handler::parse_manifests(&mut manifests);

    println!("[*] Parsing done");
    // let containers = &manifest.spec.containers;
    // if let Some(containers) = containers {
    //     println!("CONTAINERS:\n{:?}", *containers);
    // }

    check_independent_depl(manifests);
}

// TODO: Multiple services per container
fn check_independent_depl(manifests: Vec<K8SManifest>) {

    // filtering manifests, taking only the ones with kind as "Deployment" or "Pod"
    let deployment_manifests = manifests.iter()
        .filter(|man| man.kind == "Deployment" || man.kind == "Pod")
        .collect::<Vec<_>>();

    for manifest in deployment_manifests {
        let containers = &manifest.spec.containers;
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
                    println!("\t=> [Smell occurred] container named {} may not be a sidecar.", container.name);
                    println!("\t.. Therefore it can potentially violate the Independent Deployability rule");
                }
            }
        }
    }

}
