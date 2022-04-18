use std::{collections::HashMap};

use serde_with::skip_serializing_none;
use serde::{Deserialize, Serialize};
use serde_yaml::to_string;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerCompose {
    pub version: String,
    pub services: HashMap<String, DockerComposeService>,
}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerComposeService {
    pub image: String,
    pub name: String,
    pub restart: Option<String>,
    pub privileged: Option<bool>,
    pub command: Option<String>,
    pub tty: Option<bool>,
    pub ports: Option<Vec<String>>,
    pub volumes: Option<Vec<String>>,
    pub environment: Option<Vec<String>>,
    pub network: Option<Vec<String>>,
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
