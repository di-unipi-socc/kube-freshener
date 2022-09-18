mod k8s_types;
mod yaml_handler;
mod freshener;

use crate::{k8s_types::*};

fn main() {
    let mut manifests: Vec<K8SManifest> = Vec::new();

    yaml_handler::parse_manifests(&mut manifests);

    freshener::check_independent_depl(manifests);
}