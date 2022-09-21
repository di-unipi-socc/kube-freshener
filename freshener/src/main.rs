mod k8s_types;
mod yaml_handler;
mod freshener;

use std::env;

use crate::{yaml_handler::IgnoreItem};
use crate::{k8s_types::*};

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        let mut manifests: Vec<K8SManifest> = Vec::new();
            
        yaml_handler::parse_manifests(&mut manifests);
        freshener::check_independent_depl(&manifests);
        freshener::check_no_apigateway(&manifests);
        return;
    }

    match args[1].as_str() {
        "list-ignore" => yaml_handler::read_ignore_list(),
        "add-ignore" => {
            if args.len() < 5 {
                println!("[X] You're missing parameters for 'add-ignore' command [cargo run add-ignore <name> <image> <kind>");
                return;
            }

            let ignore_item = IgnoreItem {
                name: args[2].clone(),
                image: args[3].clone(),
                kind: args[4].clone(),
            };

            yaml_handler::add_ignore(ignore_item);
        },
        "delete-ignore" => {
            if args.len() < 3 {
                println!("[X] You're missing <name> parameter: [cargo run delete-ignore <name>]");
                return;
            }

            yaml_handler::delete_ignore(args[2].to_owned());
        },
        _ => { return; },
    }
}