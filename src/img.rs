use crate::grid::PixelGrid;
use colorful::{Colorful, RGB};
use image::{imageops, DynamicImage};
use rand::Rng;
use reqwest;
use std::env::temp_dir;

const GITHUB_BASE_URL: &'static str =
    "https://raw.githubusercontent.com/michellocana/ednaldo/master/";

pub type ImageList = Vec<String>;

pub fn resize_image(
    image: DynamicImage,
    resize_width: u32,
    down_scaling_factor: Option<u32>,
) -> DynamicImage {
    let (image_width, image_height) = image.to_rgb().dimensions();
    let aspect_ratio = image_height / image_width;
    let characters_per_pixel_value = match down_scaling_factor {
        Some(n) => n,
        None => 1,
    };

    let resized_image = image.resize(
        resize_width / characters_per_pixel_value,
        resize_width / characters_per_pixel_value * aspect_ratio,
        imageops::FilterType::Triangle,
    );

    resized_image
}

#[test]
fn test_resize_image() {
    // given
    let image: DynamicImage = image::open("fixtures/ednaldinho.jpg").unwrap();
    let resized_image: DynamicImage = resize_image(image, 10, None);

    // when
    let (image_width, image_height) = resized_image.to_rgb().dimensions();

    // assert
    assert_eq!(image_width, 10);
    assert_eq!(image_height, 10);
}

pub fn print_image(pixel_grid: PixelGrid, characters_per_pixel: Option<u32>) -> String {
    let mut output: String = String::new();
    let characters_per_pixel_value = match characters_per_pixel {
        Some(n) => n,
        None => 1,
    };

    for row in pixel_grid {
        for column in row {
            let r = column[0];
            let g = column[1];
            let b = column[2];

            output = format!(
                "{}{}",
                output,
                std::iter::repeat("█")
                    .take(characters_per_pixel_value as usize)
                    .collect::<String>()
                    .color(RGB::new(r, g, b))
                    .bold()
            )
        }

        output.push('\n');
    }

    print!("{}", output);

    output
}

#[test]
fn test_print_image() {
    // given
    use crate::grid::get_pixel_grid;
    let image = image::open("fixtures/red-square.jpg").unwrap();
    let pixel_grid = get_pixel_grid(image);
    let pixel_row = &pixel_grid[0];
    let pixel_column = &pixel_row[0];

    // when
    let r = &pixel_column[0];
    let g = &pixel_column[1];
    let b = &pixel_column[2];

    // assert
    assert_eq!(r, &137);
    assert_eq!(g, &45);
    assert_eq!(b, &46);
}

pub fn get_random_image_url(image_list: ImageList) -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, image_list.len());
    let image_path = format!("{}{}{}", GITHUB_BASE_URL, "images/", image_list[index]);

    image_path
}

#[tokio::test]
async fn test_get_random_image_url() {
    // given
    use std::collections::HashSet;
    let image_list: ImageList = get_image_list().await.unwrap();
    let range_size = image_list.len() * 10;
    let iter = (0..range_size).into_iter();
    let mut image_names: Vec<String> = iter
        .map(|_n| get_random_image_url(image_list.clone()))
        .collect();

    // when
    let unique_image_names: HashSet<String> = image_names.drain(0..range_size).collect();
    let image_count = image_list.len();

    // assert
    assert_eq!(unique_image_names.len(), image_count);
}

pub async fn get_temp_image(url: String) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let image_bytes = reqwest::get(&url[..]).await?.bytes().await?;
    let mut temp_dir = temp_dir();
    temp_dir.push("ednaldo/pereira.jpg");
    let mut out = tokio::fs::File::create(&temp_dir).await?;

    tokio::io::copy(&mut &*image_bytes, &mut out).await?;

    match image::open(temp_dir.to_str().unwrap()) {
        Ok(image) => Ok(image),
        Err(error) => Err(Box::new(error)),
    }
}

#[tokio::test]
async fn test_get_temp_image() -> Result<(), Box<dyn std::error::Error>> {
    // given
    let image_list = get_image_list().await?;
    let url = get_random_image_url(image_list);
    let temp_image: DynamicImage = get_temp_image(url).await?;

    // when
    let (image_width, image_height) = temp_image.to_rgb().dimensions();

    // assert
    assert_eq!(image_width > 0, true);
    assert_eq!(image_height > 0, true);

    Ok(())
}

pub async fn get_image_list() -> Result<ImageList, Box<dyn std::error::Error>> {
    let url = &format!("{}{}", GITHUB_BASE_URL, "images.txt")[..];
    let images_string = reqwest::get(url).await?.text().await?;
    let images_vec: ImageList = images_string
        .split_whitespace()
        .map(|image_name| String::from(image_name))
        .collect();

    Ok(images_vec)
}

#[tokio::test]
async fn test_get_image_list() -> Result<(), Box<dyn std::error::Error>> {
    // given
    let image_list = get_image_list().await?;

    // when
    let image_list_size = image_list.len();

    // assert
    assert!(image_list_size > 0, true);

    Ok(())
}
