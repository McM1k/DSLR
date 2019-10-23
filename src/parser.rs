use csv::StringRecord;
use std::fs::File;
use crate::student::Student;
use crate::new_student::NewStudent;

pub fn get_train_file_content(filename: String) -> Vec<Student> {
    let mut csv = csv::Reader::from_path(filename).expect("cannot read csv");

    let mut data = Vec::new();

    for line in csv.records() {
        let sr = line.expect("Cannot parse one of the lines");
        let tokens = sr.iter().map(|tk| tk.to_string()).collect();
        data.push(Student::my_deserialize(tokens));
    }

    data
}


//pub fn get_data_from_csv(filename: String) -> DataFrame {
  //  DataFrame::read_csv(&filename, ',')
//}

pub fn get_test_file_content(filename: String) -> Vec<NewStudent> {
    Vec::new() //TODO
}
