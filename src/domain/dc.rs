use regex::Regex;
use serde::{Deserialize, Serialize};
use std::process::Command;

use super::prompt::{HavePrompt, Prompt};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerCompose {
    pub version: String,
    pub services: Vec<DockerComposeService>,
}

pub trait DockerComposePrompt {
    fn container_name(&self) -> String;
    fn select_image_name(images: Vec<String>) -> String;
    fn get_dc_version() -> String;
    fn image_search_word() -> String;
    fn get_container_number() -> usize;
    fn get_docker_image_names(image_search_word: String) -> Vec<String>;
    fn extract_image_names(docker_search_str: String) -> Vec<String>;
}

pub trait IDockerComposePrompt: HavePrompt {}

impl<T: IDockerComposePrompt> DockerComposePrompt for T {
    fn container_name(&self) -> String {
        <T as HavePrompt>::Prompt::input_with_retry(
            "Input Container Name",
            "Please Input Container Name",
        )
    }

    fn select_image_name(images: Vec<String>) -> String {
        <T as HavePrompt>::Prompt::select_one(
            "Select Base Image",
            images,
            "Please Select Base Image",
        )
    }

    fn get_dc_version() -> String {
        <T as HavePrompt>::Prompt::input_with_retry_and_default(
            "docker-compose file version?",
            "need input version",
            "3.2",
        )
    }

    fn image_search_word() -> String {
        <T as HavePrompt>::Prompt::input_with_retry(
            "What word(s) do you search for?",
            "please input message",
        )
    }

    fn get_container_number() -> usize {
        let container_number = <T as HavePrompt>::Prompt::input_with_retry_and_default(
            "How many containers ?",
            "Please input number",
            "1",
        );

        if let Ok(num) = container_number.parse::<usize>() {
            num
        } else {
            println!();
            Self::get_container_number()
        }
    }

    fn get_docker_image_names(image_search_word: String) -> Vec<String> {
        let out = Command::new("sh")
            .arg("-c")
            .arg(format!("{}{}", "docker search ", image_search_word))
            .output()
            .expect("failed to execute process");

        let search_result = String::from_utf8(out.stdout).unwrap();

        Self::extract_image_names(search_result)
    }

    fn extract_image_names(docker_search_str: String) -> Vec<String> {
        let results = docker_search_str.split("\n").collect::<Vec<&str>>();
        let pattern = Regex::new(r"\s+").unwrap();
        let image_and_details = results
            .into_iter()
            .map(|x| {
                pattern
                    .replace_all(x, " ")
                    .into_owned()
                    .as_str()
                    .to_string()
                    .into()
            })
            .collect::<Vec<String>>();

        image_and_details[1..]
            .into_iter()
            .map(|x| x.split(" ").next().unwrap().to_string().into())
            .collect::<Vec<String>>()
    }
}

pub trait HaveDockerComposePrompt {
    type DockerComposeService: DockerComposePrompt;
    fn get_docker_compose_service(&self) -> &Self::DockerComposeService;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
