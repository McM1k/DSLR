extern crate clap;

use clap::{App, Arg};

pub enum Visu {
    Describe,
    Histogram,
    Scatter,
    Pair,
}

pub enum Args {
    Visualize(String, Visu),
    Train(String),
    Predict(String),
}

pub fn get_opt() -> Args {
    let matches = App::new("Choipeaux v2.0")
        .author("gboudrie@student.42.fr")
        .about("Train on data, provide plots, then choose a house for your student.")
        .arg(
            Arg::with_name("predict")
                .short("p")
                .long("predict")
                .help("Predict houses on a given file")
                .conflicts_with_all(&["visualize", "train"]),
        )
        .arg(
            Arg::with_name("train")
                .short("t")
                .long("train")
                .help("Train the program with a given file")
                .conflicts_with_all(&["predict", "visualize"]),
        )
        .arg(
            Arg::with_name("visualize")
                .short("v")
                .long("visualize")
                .value_name("type")
                .help("Visualize training file in various ways")
                .takes_value(true)
                .conflicts_with_all(&["predict", "train"])
                .possible_values(&["describe", "histogram", "scatter", "pair"]),
        )
        .arg(
            Arg::with_name("FILE")
                .value_name("FILE")
                .help("The input file")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .get_matches();

    let mut args = Args::Predict("".to_string());
    let filename = matches.value_of("FILE").unwrap().to_string();
    if matches.is_present("predict") {
        args = Args::Predict(filename);
    } else if matches.is_present("train") {
        args = Args::Train(filename);
    } else if matches.is_present("visualize") {
        let mut visu = Visu::Describe;
        if matches.is_present("visualize") {
            visu = match matches.value_of("visualize").unwrap() {
                "describe" => Visu::Describe,
                "histogram" => Visu::Histogram,
                "scatter" => Visu::Scatter,
                "pair" => Visu::Pair,
                _ => Visu::Describe,
            };
        }
        args = Args::Visualize(filename, visu);
    }

    args
}
