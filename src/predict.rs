use crate::new_student::{NewStudent};
use crate::student::{House, Hand, Student};

pub fn predict(students: Vec<NewStudent>) {
    //TODO
}

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + exp(z * -1.0))
}

pub fn h(thetas: Vec<f64>, student: Student) -> f64 {
    let mut result = thetas[0];
    let mut i = 1;
    for ft in Features {
        result = result + (ft.func()(&student) * thetas[i]);
        i = i + 1;
    }
    sigmoid(result)
}
