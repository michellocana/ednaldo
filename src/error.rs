use colored::*;
use std::{error::Error, process::exit};

const EDNALDOS_SHIRT_COLOR: (u8, u8, u8) = (253, 142, 96);

/// Prints an error with Ednaldo Pereira's name highlighted
pub fn print_error(error: Box<dyn Error>) -> ! {
    print_colored(format!("{}", error));
    exit(0);
}

/// Prints a warning and don't exit code
pub fn print_warning(warning: String) {
    print_colored(warning);
}

fn print_colored(message: String) {
    let (r, g, b) = EDNALDOS_SHIRT_COLOR;
    let colored_error = message.replace(
        "Ednaldo Pereira",
        &"Ednaldo Pereira".truecolor(r, g, b).bold().to_string()[..],
    );

    println!("{}", colored_error);
}
