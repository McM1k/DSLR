use crate::student::{Student, Hand, House, Features};
use crate::predict::*;

pub fn train(students: Vec<Student>) {
    let mut thetas = vec![0.0; 14];
    let mut weights = vec![thetas.clone(); 4];
    let epochs = 10_000;

    for epoch in 0..epochs {
        //TODO : plot loss

        let mut k = 0;
        for house in House {
            weights[k] = thetas_by_epoch(students.clone(), house.clone(), weights[k]);
            k += 1;
        }
    }

    //TODO : write weights in weights.csv
}

fn thetas_by_epoch(students: Vec<Student>, house: House, thetas: Vec<f64>) -> Vec<f64> {
    let mut tmp = thetas.clone();
    tmp[0] += deriv(thetas, students.clone(), house, |_s| 1.0);

    let mut i = 1;
    for ft in Features {
        tmp[i] += deriv(thetas, students.clone(), house, ft.func());
        i += 1;
    }
    tmp
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

pub fn deriv(&thetas: Vec<f64>, students: Vec<Student>, &house: House, func: fn(&Student)->f64) -> f64 {
    let m = students.len();
    let mut sum = 0.0;
    for x in students {
        let y = match student.house {
            house => 1.0,
            _ => 0.0,
        };
        sum = sum + (h(thetas, student) - y) * func()(&x);
    }
    (-1.0 / m as f64) * sum
}