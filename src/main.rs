mod k8s_types;
mod yaml_handler;
mod freshener;
mod cmd_handler;
mod config_type;

use std::env;

use crate::{k8s_types::*};
use crate::{cmd_handler::CMD};
use colored::Colorize;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please type commands like analyze, list-known-images, ...");
        return;
    }

    let command = CMD::from_str(args[1].as_str());

    match command {
        CMD::Analyze => {
            let mut manifests: Vec<K8SManifest>;
                
            println!("{}\n", format!("*** K8S FRESHENER ***").blue().bold());

            println!("{}", format!("####### Parsing ########").bold());

            manifests = startup(true);
            println!("{}", format!("### Start Inspection ###").bold());

            let is_to_refactor = args.len() >= 3 && args[2].clone() == "-s";

            freshener::check_independent_depl(&manifests, is_to_refactor);

            manifests = startup(false);
            freshener::check_no_apigateway(&manifests, is_to_refactor);

            manifests = startup(false);
            freshener::check_endpoint_based_interaction(&manifests, is_to_refactor);

            manifests = startup(false);
            freshener::check_wobbly_interaction(&manifests, is_to_refactor);

            println!("{}", format!("### Inspection Ended ###").bold());
        },
        _ =>  println!("Unrecognized command")
    }

}

fn startup(log: bool) -> Vec<K8SManifest> {
    return yaml_handler::parse_manifests(log);
}
