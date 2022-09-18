use serde::{Deserialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Container {
    pub name: String,
    pub image: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Volume {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateSpec {
    pub initContainers: Option<Vec<Container>>,
    pub containers: Option<Vec<Container>>,
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Template {
    pub spec: Option<TemplateSpec>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Spec {
    pub initContainers: Option<Vec<Container>>,
    pub containers: Option<Vec<Container>>,
    pub volumes: Option<Vec<Volume>>,
    pub template: Option<Template>
}

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata { pub name: String }

#[derive(Debug, Clone, Deserialize)]
pub struct K8SManifest {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: Option<Metadata>,
    pub spec: Spec,
}