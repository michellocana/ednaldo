use colored::*;
use std::{error::Error, process::exit};

const EDNALDOS_SHIRT_COLOR: (u8, u8, u8) = (253, 142, 96);

/// Prints an error with Ednaldo Pereira's name highlighted
pub fn print_error(error: Box<dyn Error>) -> ! {
    let (r, g, b) = EDNALDOS_SHIRT_COLOR;
    let colored_error = format!("{}", error).replace(
        "Ednaldo Pereira",
        &"Ednaldo Pereira".truecolor(r, g, b).bold().to_string()[..],
    );

    println!("{}", colored_error);
    exit(0);
}
