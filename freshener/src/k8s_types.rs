use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Port {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostPort: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "securityContext")]
    pub security_context: Option<SecurityContext>,
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<Port>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "readinessProbe")]
    pub readiness_probe: Option<Exec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "livenessProbe")]
    pub liveness_probe: Option<Exec>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataTemplate>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Labels {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selector {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "matchLabels")]
    pub match_labels: Option<Labels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>
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
pub struct SecurityContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fsGroup")]
    pub fs_group: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "runAsGroup")]
    pub run_as_group: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "runAsNonRoot")]
    pub run_as_non_root: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "runAsUser")]
    pub run_as_user: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allowPrivilegeEscalation")]
    pub allow_privilege_escalation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priviliged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "readOnlyRootFilesystem")]
    pub read_only_root_filesystem: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Spec {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serviceAccountName")]
    pub service_account_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "terminationGracePeriodSeconds")]
    pub termination_grace_period_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "securityContext")]
    pub security_context: Option<SecurityContext>,
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
