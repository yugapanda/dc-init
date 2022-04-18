use regex::Regex;
use std::collections::HashMap;
use std::process::Command;

use super::dc::DockerCompose;
use super::dc::DockerComposeService;
use super::prompt::HavePrompt;
use super::prompt::Prompt;

pub trait DockerComposePrompt: HavePrompt {
    fn make_dc() -> DockerCompose {
        let version = Self::confirmation_str(Self::input_dc_version);
        let service_number = Self::input_container_number();
        let services_list: Vec<DockerComposeService> = (0..service_number).map(|_x| Self::make_service()).collect();

        let mut services = HashMap::new();
        
        for x in services_list {
            services.insert(x.name.clone(), x);
        }

        DockerCompose {
            version: version,
            services: services,
        }
    }

    fn make_service() -> DockerComposeService {
        let image = Self::search_image();
        let name = Self::confirmation_str(Self::container_name);
        let restart = Self::confirmation_str(Self::select_restart);
        let privileged = Self::confirmation_opt_bool(Self::select_privileged);
        let command = Self::confirmation_opt_str(Self::input_command);
        let tty = Self::confirmation_opt_bool(Self::select_tty);

        DockerComposeService {
            image,
            name,
            restart: Some(restart),
            privileged: Some(privileged),
            tty: Some(tty),
            command,
            ports: None,
            volumes:  None,
            environment:  None,
            network:  None,
        }
    }

    fn search_image() -> String {
        let image_search_word = Self::image_search_word();
        let images = Self::get_docker_image_names(image_search_word);
        let select_image = Self::select_image_name(images);

        let ok = Self::Prompt::select_one(
            format!("selected image is {select_image} ?").as_str(),
            vec!["Ok".to_string(), "Retry".to_string()],
            "select one",
        );

        if ok == "Ok" {
            select_image
        } else {
            Self::search_image()
        }
    }

    fn confirmation_str(f: impl Fn() -> String) -> String {
        let input = f();
        let ok = Self::Prompt::select_one(
            format!("input is {:?} OK ?", input).as_str(),
            vec!["Ok".to_string(), "Retry".to_string()],
            "select one",
        );

        if ok == "Ok" {
            input
        } else {
            Self::confirmation_str(f)
        }
    }

    fn confirmation_opt_str(f: impl Fn() -> Option<String>) -> Option<String> {
        let input = f();

        let conf = match &input {
            Some(x) => x.clone(),
            None => "nothing".to_string(),
        };

        let ok = Self::Prompt::select_one(
            format!("input is {} OK ?", conf).as_str(),
            vec!["Ok".to_string(), "Retry".to_string()],
            "select one",
        );

        if ok == "Ok" {
            input
        } else {
            Self::confirmation_opt_str(f)
        }
    }

    fn confirmation_opt_bool(f: impl Fn() -> bool) -> bool {
        let input = f();
        let ok = Self::Prompt::select_one(
            format!("input is {:?} OK ?", input).as_str(),
            vec!["Ok".to_string(), "Retry".to_string()],
            "select one",
        );

        if ok == "Ok" {
            input
        } else {
            Self::confirmation_opt_bool(f)
        }
    }


    fn container_name() -> String {
        Self::Prompt::input_with_retry("Input Container Name", "Please Input Container Name")
    }

    fn select_image_name(images: Vec<String>) -> String {
        Self::Prompt::select_one("Select Base Image", images, "Please Select Base Image")
    }

    fn select_privileged() -> bool {
        let is = Self::Prompt::select_one(
            "Need privilege ?",
            vec!["true".to_string(), "false".to_string()],
            "Please Select Base Image",
        );
        is == "true"
    }

    fn input_command() -> Option<String> {
        let command =
            Self::Prompt::input_with_retry_and_default("Input command", "need input version", "");

        if command == "".to_string() {
            None
        } else {
            Some(command)
        }
    }

    fn select_tty() -> bool {
        let is = Self::Prompt::select_one(
            "Need tty ?",
            vec!["true".to_string(), "false".to_string()],
            "Please Select tty",
        );
        is == "true"
    }

    fn select_restart() -> String {
        Self::Prompt::select_one(
            "Select Restart Way",
            vec![
                "no".to_string(),
                "on-failure".to_string(),
                "always".to_string(),
                "unless-stopped".to_string(),
            ],
            "Please Select Base Image",
        )
    }

    fn input_dc_version() -> String {
        Self::Prompt::input_with_retry_and_default(
            "docker-compose file version?",
            "need input version",
            "3.2",
        )
    }

    fn image_search_word() -> String {
        Self::Prompt::input_with_retry("What word(s) do you search for?", "please input some word")
    }

    fn input_container_number() -> usize {
        let container_number = Self::Prompt::input_with_retry_and_default(
            "How many containers ?",
            "Please input number",
            "1",
        );

        if let Ok(num) = container_number.parse::<usize>() {
            num
        } else {
            println!();
            Self::input_container_number()
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
    type DockerComposePrompt: DockerComposePrompt;
    fn get_docker_compose_service(&self) -> &Self::DockerComposePrompt;
}
