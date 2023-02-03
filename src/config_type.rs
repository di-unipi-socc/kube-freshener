use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NodeConfigElement {
    pub name: String,
    pub containers: Option<Vec<String>>
}

#[derive(Debug, Deserialize)]
pub struct Smells {
    pub multiple_container: Vec<NodeConfigElement>,
    pub noapigateway: Vec<NodeConfigElement>,
    pub endpoint_based_interaction: Vec<String>,
    pub wobbly: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ignore_smells: Smells,
    pub invoked_services: Vec<String>,
    pub ignored_manifests: Vec<String>
}