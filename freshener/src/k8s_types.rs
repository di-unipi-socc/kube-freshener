use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Container {
    pub name: String,
    pub image: String,
}

#[derive(Debug, Deserialize)]
pub struct Volume {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub initContainers: Option<Vec<Container>>,
    pub containers: Option<Vec<Container>>,
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Deserialize)]
pub struct Metadata { pub name: String }

#[derive(Debug, Deserialize)]
pub struct K8SManifest {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: Option<Metadata>,
    pub spec: Spec,
}