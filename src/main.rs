mod domain;
mod infra;
mod usecase;

use domain::dc_prompt::DockerComposePrompt;
use usecase::dc::Composer;

fn main() {
    prompt()
}

fn prompt() {
    let dc = Composer::make_dc();
    println!("{}", dc.to_yaml());
}