use crate::student::{Student, Hand, House};

pub fn train(students: Vec<Student>) {
    //TODO
}

pub fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + exp(z * -1.0))
}

pub fn loss(theta: f64, students: Vec<Student>) -> f64 {
    let m = students.len();
    let mut sum;
    for student in students {
        sum +=
    }
    (-1.0 / m as f64) * sum
}