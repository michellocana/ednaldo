use crate::grid::PixelGrid;
use colorful::{Colorful, RGB};
use image::{imageops, DynamicImage};

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
