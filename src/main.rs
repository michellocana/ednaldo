mod grid;
mod img;

use grid::{get_pixel_grid, PixelGrid};
use image::DynamicImage;
use img::{print_image, resize_image};
use term_size;

fn main() {
    if let Some((terminal_width, _terminal_height)) = term_size::dimensions() {
        match image::open("images/ednaldo.jpg") {
            Ok(result) => {
                let resized_result: DynamicImage =
                    resize_image(result, terminal_width as u32, Some(2));
                let pixel_grid: PixelGrid = get_pixel_grid(resized_result);
                print_image(pixel_grid, Some(2));
            }
            Err(_) => println!("não foi possível ler a imagem Ednaldo Pereira"),
        }
    } else {
        println!("erro ao buscar largura do terminal Ednaldo Pereira");
    }
}
