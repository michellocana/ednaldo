use image::DynamicImage;

type RgbColor = Vec<u8>;
type PixelGridRow = Vec<RgbColor>;
pub type PixelGrid = Vec<PixelGridRow>;

pub fn get_pixel_grid(result: DynamicImage) -> PixelGrid {
    let rgb_result = result.to_rgb();
    let image_width = rgb_result.dimensions().0;
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

#[test]
fn test_get_pixel_grid() {
    // given
    let pixel_grid: PixelGrid = get_pixel_grid(image::open("fixtures/red-square.jpg").unwrap());
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
