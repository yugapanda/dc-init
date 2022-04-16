use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerCompose {
    pub version: String,
    pub services: Vec<DockerComposeService>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerComposeService {
    pub image: String,
    pub name: Option<String>,
    pub restart: Option<String>,
    pub privileged: Option<String>,
    pub command: Option<String>,
    pub tty: Option<String>,
    pub ports: Vec<Ports>,
    pub volumes: Vec<Volume>,
    pub environment: Vec<String>,
    pub network: Vec<String>
}

impl DockerComposeService {
    fn restart_list() -> Vec<&'static str> {
        vec!["always", "no", "no-failure", "unless-stopped"]
    }
    fn restart_to_string(value: String) -> String {
        format!("restart: {}", value)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Env {
    pub key: String,
    pub value: String,
}

impl Env {
    pub fn to_string(self) -> String {
        format!("{}:{}", self.key, self.value)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Ports {
    pub local: usize,
    pub target: usize,
}

impl Ports {
    pub fn to_string(self) -> String {
        format!("{}={}", self.local, self.target)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub local: String,
    pub target: String,
}

impl Volume {
    pub fn to_string(self) -> String {
        format!("- {}:{}", self.local, self.target)
    }
}
