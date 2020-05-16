mod grid;
mod img;

use grid::{get_pixel_grid, PixelGrid};
use image::DynamicImage;
use img::{
    get_image_list, get_random_image_url, get_temp_image, print_image, resize_image, ImageList,
};
use std::process::exit;
use term_size::dimensions as get_terminal_dimensions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_list: ImageList = get_image_list().await?;
    let random_image_url = get_random_image_url(image_list);
    let random_image = get_temp_image(random_image_url).await?;
    let terminal_width: u32 = match get_terminal_dimensions() {
        Some((w, _h)) => w as u32,
        None => {
            println!("erro ao buscar largura do terminal Ednaldo Pereira");
            exit(0)
        }
    };

    let resized_image: DynamicImage = resize_image(random_image, terminal_width, Some(2));
    let pixel_grid: PixelGrid = get_pixel_grid(resized_image);

    print_image(pixel_grid, Some(2));

    Ok(())
}
