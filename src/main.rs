mod describe;
mod options;
mod parser;
mod plot;
mod predict;
mod select;
mod student;
mod train;

extern crate strum;
extern crate strum_macros;

use options::{get_opt, Args, Visu};

fn main() {
    let args = get_opt();

    match args {
        Args::Visualize(filename, visu) => {
            let data = parser::get_train_file_content(filename.clone());
            match visu {
                Visu::Describe => describe::describe(data.clone()),
                Visu::Histogram => plot::histogram(data.clone()),
                Visu::Scatter => plot::scatter(data.clone()),
                Visu::Pair => plot::pair(&filename),
            }
        }
        Args::Train(filename) => {
            let data = parser::get_train_file_content(filename.clone());

            train::train(data);
        }
        Args::Predict(filename) => {
            let data = parser::get_test_file_content(filename);

            predict::predict(data);
        }
    }
}
