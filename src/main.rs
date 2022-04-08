use std::process::Command;
mod prompt;
mod dc;

use prompt::*;
use regex::Regex;

fn main() {
    prompt()
}

fn prompt() {
    let dc_version = get_dc_version();
    let number = get_container_number();

    let image_search_word = image_search_word();
    let images = get_docker_image_names(image_search_word);
    let select_image = select_image_name(images);
    let container_name = container_name();
}

fn container_name() -> String {
    input_with_retry("Input Container Name", "Please Input Container Name")
}

fn select_image_name(images: Vec<String>) -> String {
    select_one("Select Base Image", images, "Please Select Base Image")
}

fn get_dc_version() -> String {
    input_with_retry_and_default("docker-compose file version?", "need input version", "3.2")
}

fn image_search_word() -> String {
    input_with_retry("What word(s) do you search for?", "please input message")
}

fn get_container_number() -> usize {
    let container_number =
        input_with_retry_and_default("How many containers ?", "Please input number", "1");

    if let Ok(num) = container_number.parse::<usize>() {
        num
    } else {
        println!();
        get_container_number()
    }
}

fn get_docker_image_names(image_search_word: String) -> Vec<String> {
    let out = Command::new("sh")
        .arg("-c")
        .arg(format!("{}{}", "docker search ", image_search_word))
        .output()
        .expect("failed to execute process");

    let search_result = String::from_utf8(out.stdout).unwrap();

    extract_image_names(search_result)
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
