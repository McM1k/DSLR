use crate::student::{Student, Hand, House, Features};
use crate::predict::*;
use crate::strum::IntoEnumIterator;
use crate::select::feature_to_grades;
use crate::describe::get_minmax;
use crate::plot::plot_loss;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::intrinsics::write_bytes;

pub fn train(students: Vec<Student>) {
    let mut thetas = vec![0.0; 14];
    let mut weights = vec![thetas.clone(); 4];
    let epochs = 10;
    let normed = feature_scaling(students);
    let mut loss_data = vec![Vec::new(); 4];

    for epoch in 0..epochs {
        let mut k = 0;
        for house in House::iter() {
            loss_data[k].push((loss(&thetas, &normed, &house), k as f64));
            weights[k] = thetas_by_epoch(&normed, &house, &weights[k]);
            k += 1;
        }
    }

    plot_loss(&loss_data);
    to_csv(weights);
}

fn to_csv(weights: Vec<Vec<f64>>) {
    let mut content = format!("Index,th0,th1,th2,th3,th4,th5,th6,th7,th8,th9,th10,th11,th12,th13\n");
    for i in 0..weights.len() {
        content = format!("{}{}", content, i);
        for th in weights[i].clone() {
            content = format!("{},{}", content, th);
        }
        content = format!("{}\n", content);
    }
    let filename = "resources/weights.csv";
    let mut file = File::create(Path::new(filename)).expect("Cannot create weights.csv");
    match file.write_all(content.as_bytes()) {
        Ok(_x) => println!("Wrote {}", filename),
        Err(_e) => panic!("Cannot write weights.csv"),
    }
}

fn wash_data(students: Vec<Student>) -> Vec<Student> {
    let mut washed = Vec::new();
    for student in students {
        let mut contains_zero = false;
        for ft in Features::iter() {
            if ft.func()(&student) == 0.0 {
                contains_zero = true;
            }
        }
        if !contains_zero {
            washed.push(student.clone())
        }
    }
    washed
}

fn feature_scaling(students: Vec<Student>) -> Vec<Student> {
    //let mut washed = wash_data(students);
    let mut normed = students.clone();

    for ft in Features::iter() {
        let grades = feature_to_grades(students.clone(), ft.clone());
        let xrange = get_minmax(&grades);
        for i in 0..students.len() {
            normed[i].set_feature((ft.func()(&students[i]) - xrange.0) / (xrange.1 - xrange.0), &ft)
        }
    }

    normed
}

fn thetas_by_epoch(students: &Vec<Student>, house: &House, thetas: &Vec<f64>) -> Vec<f64> {
    let mut tmp = thetas.clone();
    tmp[0] += deriv(&thetas, &students, &house, |_s| 1.0);

    let mut i = 1;
    for ft in Features::iter() {
        tmp[i] += deriv(&thetas, &students, &house, ft.func());
        i += 1;
    }
    tmp
}

pub fn loss(thetas: &Vec<f64>, students: &Vec<Student>, house: &House) -> f64 {
    let m = students.len();
    let mut sum= 0.0;
    for x in students {
        let y = match &x.house {
            house => 1.0,
            _ => 0.0,
        };
        sum += y * (h(&thetas, &x)).log10() + (1.0 - y) * (1.0 - h(&thetas, &x)).log10()
    }
    (-1.0 / m as f64) * sum
}

pub fn deriv(thetas: &Vec<f64>, students: &Vec<Student>, house: &House, func: fn(&Student)->f64) -> f64 {
    let m = students.len();
    let mut sum = 0.0;
    for x in students {
        let y = match &x.house {
            house => 1.0,
            _ => 0.0,
        };
        sum = sum + (h(&thetas, &x) - y) * func(&x);
    }
    (-1.0 / m as f64) * sum
}