mod k8s_types;
mod yaml_handler;
mod freshener;
mod cmd_handler;

use std::env;

use crate::{yaml_handler::KnownImage};
use crate::{k8s_types::*};
use crate::{cmd_handler::CMD};


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        let mut manifests: Vec<K8SManifest> = Vec::new();
            
        yaml_handler::parse_manifests(&mut manifests);
        freshener::check_independent_depl(&manifests);
        freshener::check_no_apigateway(&manifests);
        return;
    }

    let command = CMD::from_str(args[1].as_str());

    match command {
        CMD::ListKnownImages => yaml_handler::read_known_imgaes(),
        CMD::ListManifestsIgnore => yaml_handler::read_manifest_ignore(),
        CMD::AddKnownImage => {
            if CMD::check_args(&command, &args) {
                let known_image = KnownImage {
                    name: args[2].clone(),
                    image: args[3].clone(),
                    kind: args[4].clone(),
                };
    
                yaml_handler::add_known_image(known_image);
            }
        },
        CMD::AddManifestIgnore => {
            if CMD::check_args(&command, &args) {
                let filename = args[2].clone().to_owned();
                yaml_handler::add_manifest_ignore(filename);
            }
        },
        CMD::DeleteKnownImage => {
            if CMD::check_args(&command, &args) {
                let name = args[2].clone().to_owned();
                yaml_handler::delete_known_image(name);
            }
        },
        CMD::DeleteManifestIgnore => {
            if CMD::check_args(&command, &args) {
                let name = args[2].clone().to_owned();
                yaml_handler::delete_manifest_ignore(name);
            }
        }
        _ =>  println!("Unrecognized command")
    }

}

