use serde::{Deserialize};

#[derive(Debug, Clone, Deserialize)]
#[allow(non_snake_case)]
pub struct Port {
    pub containerPort: Option<i32>,
    pub hostPort: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Container {
    pub name: String,
    pub image: String,
    pub ports: Option<Vec<Port>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Volume {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(non_snake_case)]
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
#[allow(non_snake_case)]
pub struct Spec {
    pub initContainers: Option<Vec<Container>>,
    pub containers: Option<Vec<Container>>,
    pub volumes: Option<Vec<Volume>>,
    pub template: Option<Template>,
    pub hostNetwork: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata { pub name: String }

#[derive(Debug, Clone, Deserialize)]
#[allow(non_snake_case)]
pub struct K8SManifest {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: Option<Metadata>,
    pub spec: Spec,
}

pub fn get_patterns() -> Vec<String> {
    let owned_vec = vec![
        "sidecar",
        "ambassador",
        "adapter",
    ].iter()
        .map(|&s| s.to_owned()).collect::<Vec<_>>();

    owned_vec
}

pub fn get_known_sidecars() -> Vec<String> {
    let x =vec![
        "busybox",
        "dynatrace/oneagent",
        "datadog/agent",
        "prom/prometheus",
        "elasticsearch",
        "kibana"
    ].iter().map(|&s| s.to_owned()).collect::<Vec<_>>();

    x
}
