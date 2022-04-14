mod domain;
mod infra;
mod usecase;


use domain::dc_prompt::DockerComposePrompt;
use usecase::dc::Composer;

fn main() {
    prompt()
}

fn prompt() {

    let dc_version = Composer::get_dc_version();
    let number = Composer::get_container_number();

    let image_search_word = Composer::image_search_word();
    let images = Composer::get_docker_image_names(image_search_word);
    let select_image = Composer::select_image_name(images);
    let container_name = Composer::container_name();

}