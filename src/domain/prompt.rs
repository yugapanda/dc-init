
pub trait Prompt {
    fn select_one(message: &str, selection: Vec<String>, error_message: &str) -> String;
    fn input_with_retry(message: &str, error_message: &str) -> String;
    fn input_with_retry_and_default(message: &str, error_message: &str, default: &str) -> String;
}

pub trait HavePrompt {
    type Prompt: Prompt;
    fn get_prompt(&self) -> &Self::Prompt;
}
