use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Port {
    // pub containerPort: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<Port>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Resources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<Env>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TemplateSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initContainers: Option<Vec<Container>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<Container>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub spec: TemplateSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selector {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlierDetection {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "consecutive5xxErrors")]
    pub consecutive_errors: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initContainers: Option<Vec<Container>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<Container>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostNetwork: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<Selector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trafficPolicy: Option<TrafficPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restartPolicy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata { 
    pub name: String
 }

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
