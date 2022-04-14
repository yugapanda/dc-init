use crate::{domain::{prompt::{HavePrompt, IDockerComposePrompt, HaveDockerComposePrompt}}, infra::prompt::PromptDialoguer};

pub struct Composer {}

impl PromptDialoguer for Composer {}
impl IDockerComposePrompt for Composer {}

impl HavePrompt for Composer {
    type Prompt = Self;

    fn get_prompt(&self) -> &Self::Prompt {
        &self
    }
}

impl HaveDockerComposePrompt for Composer {
    type DockerComposeService = Self;

    fn get_docker_compose_service(&self) -> &Self::DockerComposeService {
        &self
    }
}