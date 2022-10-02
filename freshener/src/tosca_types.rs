use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Requirement {
    pub interaction: Option<Interaction>
}

#[derive(Debug, Deserialize)]
pub struct DetailedInteraction {
    pub node: Option<String>,
    pub relationship: Option<String>
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Interaction {
    String(String),
    DetailedInteraction(DetailedInteraction)
}

#[derive(Debug, Deserialize)]
pub struct NodeTemplate {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub requirements: Option<Vec<Requirement>>
}