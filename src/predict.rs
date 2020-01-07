use crate::parser::get_weights_file_content;
use crate::strum::IntoEnumIterator;
use crate::student::House::*;
use crate::student::{Features, House, Student};
use crate::train::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn predict(students: Vec<Student>) {
    let (weights, denorm_params) = get_weights_file_content();
    let mut answers = Vec::new();
    //let normed = feature_scaling(&students, &denorm_params);
    let normed = standard_score(&students, &denorm_params);
    for student in normed {
        let mut scores = vec![0.0; 4];
        for k in 0..House::iter().len() {
            scores[k] = h(&weights[k], &student);
        }
        println!("{:?}", scores);
        answers.push(compare_scores(&scores));
    }
    write_csv(&answers)
}

fn write_csv(answers: &[House]) {
    let mut content = "Index, Hogwarts House\n".to_string();
    for (i, answer) in answers.iter().enumerate() {
        content = format!("{}{},{:?}\n", content, i, *answer);
    }
    let filename = "resources/houses.csv";
    let mut file = File::create(Path::new(filename)).expect("Cannot create houses.csv");
    match file.write_all(content.as_bytes()) {
        Ok(_x) => println!("Wrote {}", filename),
        Err(_e) => panic!("Cannot write houses.csv"),
    }
}

fn compare_scores(scores: &[f64]) -> House {
    let mut tmp = 0.0;
    let mut index = 0;
    for (i, score) in scores.iter().enumerate() {
        if *score > tmp {
            tmp = *score;
            index = i;
        }
    }

    match index {
        0 => Gryffindor,
        1 => Slytherin,
        2 => Ravenclaw,
        3 => Hufflepuff,
        _ => panic!("should not be able to obtain that index"),
    }
}

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

pub fn h(thetas: &[f64], student: &Student) -> f64 {
    let mut result = thetas[0];
    for (i, ft) in Features::iter().enumerate() {
        result += ft.func()(&student) * thetas[i + 1];
    }
    sigmoid(result)
    //result
}
