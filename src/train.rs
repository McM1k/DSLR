use crate::describe::*;
use crate::plot::plot_loss;
use crate::predict::*;
use crate::select::feature_to_grades;
use crate::strum::IntoEnumIterator;
use crate::student::{Features, House, Student};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn train(students: Vec<Student>) {
    let thetas = vec![0.0; 14];
    let mut weights = vec![thetas.clone(); 4];
    let epochs = 2000;
    //let ranges = get_ranges(&students);
    //let normed = feature_scaling(&students, &ranges);
    let normed = standard_score(&students);
    let mut loss_data = vec![Vec::new(); 4];
    let mut percent = 0;
    let part = epochs / 100;

    for epoch in 0..epochs {
        for (k, house) in House::iter().enumerate() {
            if epoch % part == 0 || epoch < part * 3 {
                if house == House::Gryffindor && epoch % (part * 5) == 0 {
                    println!("training : {}%", percent);
                    percent += 5;
                }
                loss_data[k].push((f64::from(epoch), loss(&weights[k], &normed, &house)));
            }
            weights[k] = thetas_by_epoch(&normed, &house, &weights[k]);
        }
    }
    //plot_loss(&loss_data);
    //to_csv(unnormalize_weights(&weights, &ranges));
    to_csv(weights);
}

fn to_csv(weights: Vec<Vec<f64>>) {
    let mut content =
        "Index,th0,th1,th2,th3,th4,th5,th6,th7,th8,th9,th10,th11,th12,th13\n".to_string();
    for (i, thetas) in weights.iter().enumerate() {
        content = format!("{}{}", content, i);
        for th in thetas.clone() {
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

//fn wash_data(students: Vec<Student>) -> Vec<Student> {
//    let mut washed = Vec::new();
//    for student in students {
//        let mut contains_zero = false;
//        for ft in Features::iter() {
//            if ft.func()(&student) == 0.0 {
//                contains_zero = true;
//            }
//        }
//        if !contains_zero {
//            washed.push(student.clone())
//        }
//    }
//    washed
//}

fn get_ranges(students: &Vec<Student>) -> Vec<(f64, f64)> {
    let mut vec = Vec::new();
    for ft in Features::iter() {
        let grades = feature_to_grades(students, &ft);
        vec.push(get_minmax(&grades));
    }
    vec
}

fn unnormalize_weights(normed: &Vec<Vec<f64>>, ranges: &Vec<(f64, f64)>) -> Vec<Vec<f64>> {
    let mut weights = Vec::new();

    for normed_thetas in normed {
        let mut thetas = Vec::new();

        for (i, th) in normed_thetas.iter().enumerate() {
            if i == 0 {
                thetas.push(*th);
            }
            else {
                thetas[0] -= *th * ranges[i-1].0 / (ranges[i-1].1 - ranges[i-1].0);
                thetas.push(*th / (ranges[i-1].1 - ranges[i-1].0));
            }
        }
        weights.push(thetas);
    }

    weights
}

pub fn standard_score(students: &Vec<Student>) -> Vec<Student>{
    let mut normed = students.clone();

    for ft in Features::iter() {
        let grades = feature_to_grades(students,&ft);
        let mean = mean(&grades);
        let std = std(&grades);

        for (i, student) in students.iter().enumerate() {
            normed[i].set_feature(
                (ft.func()(student) - mean) / std,
                &ft,
            );
        }
    }

    normed
}

pub fn feature_scaling(students: &Vec<Student>, ranges: &Vec<(f64, f64)>) -> Vec<Student> {
    let mut normed = students.clone();

    for (j,ft) in Features::iter().enumerate() {
        let xrange = ranges[j];
        for (i, student) in students.iter().enumerate() {
            normed[i].set_feature(
                (ft.func()(student) - xrange.0) / (xrange.1 - xrange.0),
                &ft,
            );
        }
    }

    normed
}

fn thetas_by_epoch(students: &Vec<Student>, house: &House, thetas: &Vec<f64>) -> Vec<f64> {
    let mut tmp = thetas.clone();

    for (i, ft) in Features::iter().enumerate() {
        if i == 0 {
            tmp[i] += deriv(&thetas, &students, &house, |_s| 1.0);
        }
        else {
            tmp[i] += deriv(&thetas, &students, &house, ft.func());
        }
    }
    tmp
}

pub fn loss(thetas: &Vec<f64>, students: &Vec<Student>, house: &House) -> f64 {
    let m = students.len();
    let mut sum = 0.0;
    for x in students {
        let y = is_good_house(&x, &house);
        sum += y * (h(&thetas, &x)).log10() + (1.0 - y) * (1.0 - h(&thetas, &x)).log10()
    }
    sum / m as f64
}

pub fn deriv(
    thetas: &Vec<f64>,
    students: &Vec<Student>,
    house: &House,
    func: fn(&Student) -> f64,
) -> f64 {
    let count = 0.0;
    let m = students.len();
    let mut sum = 0.0;
    for x in students {
       // if func(&x) == 0.0 {
      //      count += 1.0;
       // }
      //  else {
        let y = is_good_house(&x, &house);
        sum += (h(&thetas, &x) - y) * func(&x);
      //  }
    }
    sum / (m as f64 - count)
}

fn is_good_house(student: &Student, house: &House) -> f64 {
    if student.house == *house {
        1.0
    } else {
        0.0
    }
}
