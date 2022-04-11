use crate::domain::prompt::Prompt;
use console::Term;
use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};

pub trait PromptDialoguer {}

impl<T: PromptDialoguer> Prompt for T {
    fn select_one(message: &str, images: Vec<String>, error_message: &str) -> String {
        println!("{}", message);
        if let Ok(Some(selection)) = Select::with_theme(&ColorfulTheme::default())
            .items(&images)
            .default(0)
            .interact_on_opt(&Term::stderr())
        {
            images[selection].clone()
        } else {
            println!("{}", &error_message);
            Self::select_one(message, images, error_message)
        }
    }

    fn input_with_retry(message: &str, error_message: &str) -> String {
        if let Ok(version) = Input::new().with_prompt(message).interact_text() {
            version
        } else {
            println!("{}", &error_message);
            Self::input_with_retry(message, error_message)
        }
    }

    fn input_with_retry_and_default(message: &str, error_message: &str, default: &str) -> String {
        if let Ok(version) = Input::new()
            .with_prompt(message)
            .default(default.clone().to_string())
            .interact_text()
        {
            version
        } else {
            println!("{}", &error_message);
            Self::input_with_retry_and_default(message, error_message, default)
        }
    }
}
