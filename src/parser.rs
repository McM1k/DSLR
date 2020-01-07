use crate::student::Student;

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

pub fn get_weights_file_content() -> (Vec<Vec<f64>>, Vec<(f64, f64)>) {
    let mut csv = csv::Reader::from_path("resources/weights.csv").expect("cannot read csv");
    let mut data: Vec<Vec<f64>> = Vec::new();
    let mut weights = Vec::new();
    let mut denorm_params = Vec::new();

    for line in csv.records() {
        let sr = line.expect("Cannot parse one of the lines");
        data.push(
            sr.iter()
                .map(|tk| Student::parse_f64(tk.to_string()))
                .collect(),
        );
    }
    for i in 0..4 {
        weights.push(data[i].clone());
        weights[i].remove(0);
    }
    println!("{:?}\n", weights);
    for j in 0..13 {
        denorm_params.push((data[4][j + 2], data[5][j + 2]));
    }
    println!("{:?}\n", denorm_params);
    (weights, denorm_params)
}

pub fn get_test_file_content(filename: String) -> Vec<Student> {
    let mut csv = csv::Reader::from_path(filename).expect("cannot read csv");
    let mut data = Vec::new();

    for line in csv.records() {
        let sr = line.expect("Cannot parse one of the lines");
        let tokens = sr.iter().map(|tk| tk.to_string()).collect();
        data.push(Student::deser_new(tokens));
    }

    data
}
