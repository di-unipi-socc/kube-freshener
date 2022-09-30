use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Requirement {
    interaction: Option<Interatction>
}

#[derive(Debug, Deserialize)]
pub struct DetailedInteraction {
    node: Option<String>,
    relationship: Option<String>
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Interatction {
    String(String),
    DetailedInteraction(DetailedInteraction)
}

#[derive(Debug, Deserialize)]
pub struct NodeTemplate {
    pub name: Option<String>,
    pub requirements: Option<Vec<Requirement>>
}