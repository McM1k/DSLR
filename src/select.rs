use crate::student::{Features, House, Student};

pub fn with_house(students: Vec<Student>, house: House) -> Vec<Student> {
    let mut selected = Vec::new();

    for student in students {
        if student.house == house {
            selected.push(student);
        }
    }

    selected
}

pub fn features_to_grade_tuples(
    house: House,
    students: Vec<Student>,
    feature1: Features,
    feature2: Features,
) -> Vec<(f64, f64)> {
    let mut grades = Vec::new();
    let g1: Vec<f64> = with_house(students.clone(), house.clone())
        .iter()
        .map(feature1.func())
        .collect();
    let g2: Vec<f64> = with_house(students, house)
        .iter()
        .map(feature2.func())
        .collect();
    if g1.len() != g2.len() {
        panic!("Not the same amount of grades on two features - features_to_grade_tuples");
    }
    for i in 0..g1.len() {
        grades.push((g1[i], g2[i]));
    }

    wash_tuple_zeros(grades)
}

pub fn with_sorted_grades(students: Vec<Student>, feature: Features) -> Vec<f64> {
    let mut values = feature_to_grades(students, feature);
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    wash_zeros(values)
}

pub fn feature_to_grades(students: Vec<Student>, feature: Features) -> Vec<f64> {
    students.iter().map(feature.func()).collect::<Vec<f64>>()
}

fn wash_tuple_zeros(data: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let mut vec = Vec::new();
    for (x, y) in data {
        if x != 0.0 && y != 0.0 {
            vec.push((x, y));
        }
    }
    vec
}

fn wash_zeros(data: Vec<f64>) -> Vec<f64> {
    let mut vec = Vec::new();
    for x in data {
        if x != 0.0 {
            vec.push(x);
        }
    }
    vec
}
