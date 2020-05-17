mod error;
mod grid;
mod img;

use error::print_error;
use grid::{get_pixel_grid, PixelGrid};
use image::DynamicImage;
use img::{
    get_image_list, get_random_image_url, get_temp_image, print_image, resize_image, ImageList,
};
use term_size::dimensions as get_terminal_dimensions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO make "ednaldo" command only work with "pereira" arg
    let image_list: ImageList = match get_image_list().await {
        Ok(image_list) => image_list,
        Err(err) => print_error(err),
    };
    let random_image_url = get_random_image_url(image_list);
    let random_image = match get_temp_image(random_image_url).await {
        Ok(random_image) => random_image,
        Err(err) => print_error(err),
    };
    let terminal_width: u32 = match get_terminal_dimensions() {
        Some((w, _h)) => w as u32,
        None => print_error("erro ao buscar largura do terminal Ednaldo Pereira".into()),
    };

    let resized_image: DynamicImage = resize_image(random_image, terminal_width, Some(2));
    let pixel_grid: PixelGrid = get_pixel_grid(resized_image);

    // TODO test in more terminals
    print_image(pixel_grid, Some(2));

    Ok(())
}
