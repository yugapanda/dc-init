use serde::{Deserialize, Serialize};
use serde_yaml::to_string;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerCompose {
    pub version: String,
    pub services: Vec<DockerComposeService>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerComposeService {
    pub image: String,
    pub name: String,
    pub restart: Option<String>,
    pub privileged: Option<String>,
    pub command: Option<String>,
    pub tty: Option<String>,
    pub ports: Vec<String>,
    pub volumes: Vec<String>,
    pub environment: Vec<String>,
    pub network: Vec<String>
}

impl DockerCompose {
    pub fn to_yaml(&self) -> String {
        if let Ok(x) = to_string(self) {
            x
        } else {
            "".to_string()
        }
    }
}