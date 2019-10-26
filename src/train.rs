use crate::student::{Student, Hand, House, Features};

pub fn train(students: Vec<Student>) {
    //TODO
}

pub fn sigmoid(z: f64) -> f64 {
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

pub fn loss(thetas: Vec<f64>, students: Vec<Student>, house: House) -> f64 {
    let m = students.len();
    let mut sum= 0.0;
    for x in students {
        let y = match student.house {
            house => 1.0,
            _ => 0.0,
        };
        sum += y * log(h(thetas, x)) + (1.0 - y) * log(1.0 - h(thetas, x))
    }
    (-1.0 / m as f64) * sum
}

pub fn deriv(thetas: Vec<f64>, students: Vec<Student>, house: House, ft: Features) -> f64 {
    let m = students.len();
    let mut sum = 0.0;
    for x in students {
        let y = match student.house {
            house => 1.0,
            _ => 0.0,
        };
        sum = sum + (h(thetas, student) - y) * ft.func()(&x);
    }
    (-1.0 / m as f64) * sum
}