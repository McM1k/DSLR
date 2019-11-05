use crate::strum::IntoEnumIterator;
use crate::student::{Features, House, Student};
use crate::parser::get_weights_file_content;

pub fn predict(students: Vec<Student>) {
    let weights = get_weights_file_content();
    for student in students {
        let mut scores = vec![0.0; 4];
        //TODO calc scores, cmp scores, output good house
    }
}

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (z * -1.0).exp())
}

pub fn h(thetas: &Vec<f64>, student: &Student) -> f64 {
    let mut result = thetas[0];
    let mut i = 1;
    for ft in Features::iter() {
        result += ft.func()(&student) * thetas[i];
        i += 1;
    }
    sigmoid(result)
}
