use crate::{
    domain::{
        dc_prompt::{DockerComposePrompt, HaveDockerComposePrompt},
        prompt::HavePrompt,
    },
    infra::prompt::PromptDialoguer,
};

pub struct Composer {}

impl PromptDialoguer for Composer {}
impl DockerComposePrompt for Composer {}

impl HavePrompt for Composer {
    type Prompt = Self;

    fn get_prompt(&self) -> &Self::Prompt {
        &self
    }
}

impl HaveDockerComposePrompt for Composer {
    type DockerComposePrompt = Self;

    fn get_docker_compose_service(&self) -> &Self::DockerComposePrompt {
        &self
    }
}
