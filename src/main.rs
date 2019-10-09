mod parser;
mod plot;
mod student;
mod describe;
mod options;
mod train;
mod predict;
mod new_student;
mod select;

extern crate strum;
extern crate strum_macros;

use std::env;
use options::{Args, Visu, get_opt};

fn main() {
    let args = get_opt();

    match args {
        Args::Train(filename, visu) => {
            let data = parser::get_train_file_content(filename.clone());
            match visu {
                Visu::Describe => describe::describe(data.clone()),
                Visu::Histogram => plot::histogram(&filename),
                Visu::Scatter => ()/*plot::scatter(data.clone())*/,
                Visu::Pair => ()/*plot::pair(data.clone())*/,
                Visu::None => (),
            }

            train::train(data);
        },
        Args::Predict(filename) => {
            let data = parser::get_test_file_content(filename);

            predict::predict(data);
        },
    }
}

