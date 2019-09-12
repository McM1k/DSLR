mod parser;
mod plot;
mod student;

use std::env;

fn main() {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("Cannot read fileName"));

    let data = get_file_content(filename);
}
