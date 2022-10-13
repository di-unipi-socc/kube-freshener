mod k8s_types;
mod yaml_handler;
mod freshener;
mod cmd_handler;
mod tosca_types;
mod config_type;

use std::env;

use tosca_types::NodeTemplate;

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
            let mut manifests: Vec<K8SManifest> = Vec::new();
            let mut tosca_nodes: Vec<NodeTemplate> = Vec::new();
                
            println!("{}\n", format!("*** K8S FRESHENER ***").blue().bold());

            println!("{}", format!("####### Parsing ########").bold());

            startup(&mut tosca_nodes, &mut manifests);
            println!("{}", format!("### Start Inspection ###").bold());

            let solve = args.len() >= 3 && args[2].clone() == "-s";

            freshener::check_independent_depl(&manifests, solve);
            freshener::check_no_apigateway(&manifests);
            freshener::check_endpoint_based_interaction(&manifests, &tosca_nodes);
            freshener::check_wobbly_interaction(&manifests, &tosca_nodes);
        },
        _ =>  println!("Unrecognized command")
    }

}

fn startup(tosca_nodes: &mut Vec<NodeTemplate>, manifests: &mut Vec<K8SManifest>) {
    yaml_handler::parse_tosca(tosca_nodes);
    yaml_handler::parse_manifests(manifests);
}