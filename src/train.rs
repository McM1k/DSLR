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
    let epochs = 500;
    let denorm_params = get_means_std(&students);
    let normed = standard_score(&students, &denorm_params);
    let mut loss_data = vec![Vec::new(); 4];
    let mut percent = 0;
    let part = epochs / 100;
    let rate = 1.0;

    for epoch in 0..epochs {
        for (k, house) in House::iter().enumerate() {
            if epoch % part == 0 || epoch < part * 3 {
                if house == House::Gryffindor && epoch % (part * 5) == 0 {
                    println!("training : {}%", percent);
                    percent += 5;
                }
                loss_data[k].push((f64::from(epoch), loss(&weights[k], &normed, &house)));
            }
            weights[k] = thetas_by_epoch(rate, &normed, &house, &weights[k]);
        }
    }
    plot_loss(&loss_data);
    to_csv(weights, &denorm_params);
}

fn to_csv(weights: Vec<Vec<f64>>, denorm_params: &[(f64, f64)]) {
    let mut content =
        "Index,th0,th1,th2,th3,th4,th5,th6,th7,th8,th9,th10,th11,th12,th13\n".to_string();

    for (i, thetas) in weights.iter().enumerate() {
        content = format!("{}{}", content, i);
        for th in thetas.clone() {
            content = format!("{},{}", content, th);
        }
        content = format!("{}\n", content);
    }
    content = format!("{}{},0.0", content, 4);
    for (x, _y) in denorm_params {
        content = format!("{},{}", content, x);
    }
    content = format!("{}\n{},0.0", content, 5);
    for (_x, y) in denorm_params {
        content = format!("{},{}", content, y);
    }
    content = format!("{}\n", content);

    let filename = "resources/weights.csv";
    let mut file = File::create(Path::new(filename)).expect("Cannot create weights.csv");
    match file.write_all(content.as_bytes()) {
        Ok(_x) => println!("Wrote {}", filename),
        Err(_e) => panic!("Cannot write weights.csv"),
    }
}

pub fn get_means_std(students: &[Student]) -> Vec<(f64, f64)> {
    let mut vec = Vec::new();

    for ft in Features::iter() {
        let grades = feature_to_grades(students, &ft);
        vec.push((mean(&grades), std(&grades)));
    }

    vec
}

pub fn standard_score(students: &[Student], means_std: &[(f64, f64)]) -> Vec<Student> {
    let mut normed = students.to_vec().clone();

    for (j, ft) in Features::iter().enumerate() {
        let (mean, std) = means_std[j];

        for (i, student) in students.iter().enumerate() {
            normed[i].set_feature((ft.func()(student) - mean) / std, &ft);
        }
    }

    normed
}

fn thetas_by_epoch(rate: f64, students: &[Student], house: &House, thetas: &[f64]) -> Vec<f64> {
    let mut tmp = thetas.to_vec().clone();

    tmp[0] -= rate * deriv(&thetas, &students, &house, |_s| 1.0);
    for (i, ft) in Features::iter().enumerate() {
        tmp[i + 1] -= rate * deriv(&thetas, &students, &house, ft.func());
    }

    tmp.to_vec()
}

pub fn loss(thetas: &[f64], students: &[Student], house: &House) -> f64 {
    let m = students.len();
    let mut sum = 0.0;
    for x in students {
        let y = is_good_house(&x, &house);
        sum += y * (h(&thetas, &x)).log10() + (1.0 - y) * (1.0 - h(&thetas, &x)).log10()
    }
    sum / m as f64
}

pub fn deriv(
    thetas: &[f64],
    students: &[Student],
    house: &House,
    func: fn(&Student) -> f64,
) -> f64 {
    let count = 0.0;
    let m = students.len();
    let mut sum = 0.0;

    for x in students {
        let y = is_good_house(&x, &house);
        sum += (h(&thetas, &x) - y) * func(&x);
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

#[cfg(test)]
mod train_tests {
    mod is_good_house {
        use crate::student::*;
        use crate::train::is_good_house;
        use chrono::NaiveDate;

        #[test]
        fn good_house() {
            let student = Student {
                house: House::Gryffindor,
                first_name: "".to_string(),
                last_name: "".to_string(),
                birthday: NaiveDate::from_num_days_from_ce(0),
                best_hand: Hand::Left,
                arithmancy: 0.0,
                astronomy: 0.0,
                herbology: 0.0,
                defense: 0.0,
                divination: 0.0,
                muggle: 0.0,
                runes: 0.0,
                history: 0.0,
                transfiguration: 0.0,
                potions: 0.0,
                creatures: 0.0,
                charms: 0.0,
                flying: 0.0,
            };

            assert_eq!(is_good_house(&student, &House::Gryffindor), 1.0);
        }

        #[test]
        fn wrong_house() {
            let student = Student {
                house: House::Hufflepuff,
                first_name: "".to_string(),
                last_name: "".to_string(),
                birthday: NaiveDate::from_num_days_from_ce(0),
                best_hand: Hand::Left,
                arithmancy: 0.0,
                astronomy: 0.0,
                herbology: 0.0,
                defense: 0.0,
                divination: 0.0,
                muggle: 0.0,
                runes: 0.0,
                history: 0.0,
                transfiguration: 0.0,
                potions: 0.0,
                creatures: 0.0,
                charms: 0.0,
                flying: 0.0,
            };

            assert_eq!(is_good_house(&student, &House::Gryffindor), 0.0);
        }
    }
}
