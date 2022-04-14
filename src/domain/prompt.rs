use std::process::Command;

use regex::Regex;


pub trait Prompt {
    fn select_one(message: &str, images: Vec<String>, error_message: &str) -> String;
    fn input_with_retry(message: &str, error_message: &str) -> String;
    fn input_with_retry_and_default(message: &str, error_message: &str, default: &str) -> String;
}


pub trait HavePrompt {
    type Prompt: Prompt;
    fn get_prompt(&self) -> &Self::Prompt;
}


pub trait DockerComposePrompt {
    fn container_name() -> String;
    fn select_image_name(images: Vec<String>) -> String;
    fn get_dc_version() -> String;
    fn image_search_word() -> String;
    fn get_container_number() -> usize;
    fn get_docker_image_names(image_search_word: String) -> Vec<String>;
    fn extract_image_names(docker_search_str: String) -> Vec<String>;
}

pub trait IDockerComposePrompt: HavePrompt {}

impl<T: IDockerComposePrompt> DockerComposePrompt for T {
    fn container_name() -> String {
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