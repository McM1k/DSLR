use csv::StringRecord;
use std::fs::File;

fn get_file_content(filename: String) -> Vec<Student> {
    let csv = csv::Reader::from_path(filename);
    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    let mut data = Vec::new();

    for line in reader.records() {
        let str = line.expect("Cannot parse one of the lines");


    }
}

