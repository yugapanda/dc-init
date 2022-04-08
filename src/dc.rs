use std::fmt;

pub struct DockerCompose {
    pub version: String,
    pub services: Vec<DockerComposeService>,
}

pub struct DockerComposeService {
    pub image: String,
    pub name: Option<String>,
    pub restart: Option<String>,
    pub privileged: bool,
    pub command: Option<String>,
    pub ports: Vec<Ports>,
    pub volumes: Vec<Volume>,
    pub environment: Vec<String>,
}

impl DockerComposeService {
    fn restart_list() -> Vec<&'static str> {
        vec!["always", "no", "no-failure", "unless-stopped"]
    }
    fn restart_to_string(value: String) -> String {
        format!("restart: {}", value)
    }
}

pub struct Env {
    pub key: String,
    pub value: String,
}

impl Env {
    pub fn to_string(self) -> String {
        format!("{}:{}", self.key, self.value)
    }
}

pub struct Ports {
    pub local: usize,
    pub target: usize,
}

impl Ports {
    pub fn to_string(self) -> String {
        format!("{}={}", self.local, self.target)
    }
}

pub struct Volume {
    pub local: String,
    pub target: String,
}

impl Volume {
    pub fn to_string(self) -> String {
        format!("- {}:{}", self.local, self.target)
    }
}
