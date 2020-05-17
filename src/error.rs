use colorful::{Colorful, RGB};
use std::{error::Error, process::exit};

const EDNALDOS_SHIRT_COLOR: (u8, u8, u8) = (253, 142, 96);

/// Prints an error with Ednaldo Pereira's name highlighted
pub fn print_error(error: Box<dyn Error>) -> ! {
    let (r, g, b) = EDNALDOS_SHIRT_COLOR;
    let color = RGB::new(r, g, b);
    let colored_error = format!("{}", error).replace(
        "Ednaldo Pereira",
        &"Ednaldo Pereira".color(color).bold().to_string()[..],
    );

    println!("{}", colored_error);
    exit(0);
}
