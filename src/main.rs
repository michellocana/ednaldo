use colorful::{Colorful, RGB};
use image::{imageops, DynamicImage};
use term_size;

type RgbColor = Vec<u8>;
type PixelGridRow = Vec<RgbColor>;
type PixelGrid = Vec<PixelGridRow>;

const CHARACTERS_PER_PIXEL: u32 = 2;

fn main() {
    if let Some((terminal_width, _terminal_height)) = term_size::dimensions() {
        match image::open("images/ednaldo.jpg") {
            Ok(result) => {
                let resized_result: DynamicImage = resize_image(result, terminal_width as u32);
                let pixel_grid: PixelGrid = get_pixel_grid(resized_result);
                print_image(pixel_grid);
            }
            Err(_) => println!("não foi possível ler a imagem Ednaldo Pereira"),
        }
    } else {
        println!("erro ao buscar largura do terminal Ednaldo Pereira");
    }
}

fn get_pixel_grid(result: DynamicImage) -> PixelGrid {
    let rgb_result = result.to_rgb();
    let (image_width, _image_height) = rgb_result.dimensions();
    let mut pixel_vec: PixelGrid = vec![];

    for (index, pixel) in rgb_result.iter().enumerate() {
        let is_r_value = index % 3 == 0;

        let row_index = (index as u32 / (image_width * 3)) as usize;
        let column_index = ((index as u32) % (image_width * 3)) as usize;

        // Pushing row
        if column_index == 0 {
            let row: PixelGridRow = vec![];
            pixel_vec.push(row);
        }

        if is_r_value {
            let rgb_vec: RgbColor = vec![*pixel];
            pixel_vec[row_index].push(rgb_vec);
        } else {
            let current_column_index = pixel_vec[row_index].len() - 1;
            pixel_vec[row_index][current_column_index].push(*pixel);
        }
    }

    pixel_vec
}

fn resize_image(image: DynamicImage, resize_width: u32) -> DynamicImage {
    let (image_width, image_height) = image.to_rgb().dimensions();
    let aspect_ratio = image_height / image_width;

    let resized_image = image.resize(
        resize_width / CHARACTERS_PER_PIXEL,
        resize_width / CHARACTERS_PER_PIXEL * aspect_ratio,
        imageops::FilterType::Triangle,
    );

    resized_image
}

fn print_image(pixel_grid: PixelGrid) {
    for row in pixel_grid {
        for column in row {
            let r = column[0];
            let g = column[1];
            let b = column[2];

            print!(
                "{}",
                std::iter::repeat("█")
                    .take(CHARACTERS_PER_PIXEL as usize)
                    .collect::<String>()
                    .color(RGB::new(r, g, b))
                    .bold()
            );
        }

        println!("");
    }
}
