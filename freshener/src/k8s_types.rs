use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Port {
    pub containerPort: Option<i32>,
    pub hostPort: Option<i32>,
}

#[derive(Debug)]
pub struct Microservice {
    pub has_service: bool,
    pub has_direct_access: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limit {
    pub cpu: String,
    pub memory: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    pub limits: Option<Limit>,
    pub requests: Option<Limit>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Env {
    pub name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    pub image: String,
    pub ports: Option<Vec<Port>>,
    pub resources: Option<Resources>,
    #[serde(rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
    pub env: Option<Vec<Env>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TemplateSpec {
    pub initContainers: Option<Vec<Container>>,
    pub containers: Option<Vec<Container>>,
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub spec: TemplateSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selector {
    pub service: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlierDetection {
    #[serde(rename = "consecutive5xxErrors")]
    pub consecutive_errors: Option<i32>,
    pub interval: Option<String>

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficPolicy {
    #[serde(rename = "outlierDetection")]
    pub outlier_detection: Option<OutlierDetection>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Spec {
    pub initContainers: Option<Vec<Container>>,
    pub containers: Option<Vec<Container>>,
    pub volumes: Option<Vec<Volume>>,
    pub template: Option<Template>,
    pub hostNetwork: Option<bool>,
    pub selector: Option<Selector>,
    pub hosts: Option<Vec<String>>,
    pub host: Option<String>,
    pub trafficPolicy: Option<TrafficPolicy>,
    pub replicas: Option<i32>,
    pub restartPolicy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata { pub name: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct K8SManifest {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub kind: String,
    pub metadata: Metadata,
    pub spec: Spec,
}

pub fn get_patterns() -> Vec<String> {
    let owned_vec = vec![
        "sidecar",
        "ambassador",
        "adapter",
    ].into_iter()
        .map(|s| s.to_owned()).collect();

    owned_vec
}
