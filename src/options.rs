extern crate clap;

use clap::{App, Arg, SubCommand, ArgMatches};

pub enum Visu {
    None,
    Describe,
    Histogram,
    Scatter,
    Pair,
}

pub enum Args {
    Train(String, Visu),
    Predict(String),
}

pub fn get_opt() -> Args {
    let matches = App::new("choipeaux v2.0")
        .about("Train on data, provide plots, then choose a house for your student")
        .arg(Arg::with_name("predict")
            .short("p")
            .long("predict")
            .value_name("TESTFILE")
            .help("Predict houses on a given file")
            .takes_value(true)
            .conflicts_with("train"))
        .arg(Arg::with_name("train")
            .short("t")
            .long("train")
            .value_name("TRAINFILE")
            .help("Train the program with a given file")
            .takes_value(true)
            .conflicts_with("predict"))
        .arg(Arg::with_name("visualize")
            .short("v")
            .long("visualize")
            .help("Visualize training file in various ways")
            .requires("train")
            .takes_value(true)
            .possible_values(&["describe", "histogram", "scatter", "pair"]))
        .get_matches();


    let mut args = Args::Predict("".to_string());
    if matches.is_present("predict") {
        args = Args::Predict(matches.value_of("TESTFILE").unwrap().to_string());
    }
    else if matches.is_present("train") {
        let mut visu = Visu::None;
        if matches.is_present("visualize") {
            visu = match matches.value_of("visualize").unwrap() {
                "describe" => Visu::Describe,
                "histogram" => Visu::Histogram,
                "scatter" => Visu::Scatter,
                "pair" => Visu::Pair,
                _ => Visu::None,
            };
        }
        args = Args::Train(matches.value_of("TRAINFILE").unwrap().to_string(), visu);
    }

    args
}