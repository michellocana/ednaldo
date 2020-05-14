mod grid;
mod img;

use grid::{get_pixel_grid, PixelGrid};
use image::DynamicImage;
use img::{get_random_image, print_image, resize_image};
use std::process::exit;
use term_size::dimensions as get_terminal_dimensions;

fn main() {
    let random_image: DynamicImage = match get_random_image() {
        Ok((image, _image_path)) => image,
        Err(err) => {
            println!("{}", err);
            exit(0)
        }
    };

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
}
