mod parser;
mod plot;
mod student;

use std::env;
use parser;

fn main() {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("Cannot read fileName"));

    let data = parser::get_file_content(filename);
}
