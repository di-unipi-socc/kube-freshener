use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Port {
    pub hostPort: Option<i32>,
    
    #[serde(rename = "containerPort")]
    pub container_port: Option<i32>
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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    pub limits: Option<Limit>,
    pub requests: Option<Limit>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Env {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec: Option<Command>
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    
    #[serde(rename = "securityContext")]
    pub security_context: Option<SecurityContext>,
    
    pub image: String,
    pub ports: Option<Vec<Port>>,
    
    #[serde(rename = "readinessProbe")]
    pub readiness_probe: Option<Exec>,
    
    #[serde(rename = "livenessProbe")]
    pub liveness_probe: Option<Exec>,
    
    pub resources: Option<Resources>,
    
    #[serde(rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
    
    pub env: Option<Vec<Env>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub name: String,
}

#[skip_serializing_none]
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
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataTemplate>
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Labels {
    pub app: Option<String>,
    pub service: Option<String>,
    pub name: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selector {
    pub service: Option<String>,
    
    #[serde(rename = "matchLabels")]
    pub match_labels: Option<Labels>,
    
    pub app: Option<String>
}

#[skip_serializing_none]
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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    #[serde(rename = "fsGroup")]
    pub fs_group: Option<i32>,
    
    #[serde(rename = "runAsGroup")]
    pub run_as_group: Option<i32>,
    
    #[serde(rename = "runAsNonRoot")]
    pub run_as_non_root: Option<bool>,
    
    #[serde(rename = "runAsUser")]
    pub run_as_user: Option<i32>,
    
    #[serde(rename = "allowPrivilegeEscalation")]
    pub allow_privilege_escalation: Option<bool>,
    
    pub priviliged: Option<bool>,
    
    #[serde(rename = "readOnlyRootFilesystem")]
    pub read_only_root_filesystem: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Destination {
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteSpec {
    pub destinations: Destination
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpSpec {
    pub route: Vec<RouteSpec>,
    pub timeout: String
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Spec {
    #[serde(rename = "serviceAccountName")]
    pub service_account_name: Option<String>,

    #[serde(rename = "terminationGracePeriodSeconds")]
    pub termination_grace_period_seconds: Option<i32>,
    
    #[serde(rename = "securityContext")]
    pub security_context: Option<SecurityContext>,
    
    pub initContainers: Option<Vec<Container>>,
    
    pub containers: Option<Vec<Container>>,
    
    pub volumes: Option<Vec<Volume>>,
    
    pub template: Option<Template>,
    
    pub hostNetwork: Option<bool>,
    
    pub selector: Option<HashMap<String, Value>>,
    
    pub hosts: Option<Vec<String>>,
    
    pub http: Option<Vec<HttpSpec>>,
    
    pub host: Option<String>,
    
    pub trafficPolicy: Option<TrafficPolicy>,
    
    pub replicas: Option<i32>,
    
    pub restartPolicy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata { 
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Labels>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataTemplate {
    pub labels: Labels
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
