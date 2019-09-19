use crate::student::{House, Hand, Student};

pub fn with_house(students: Vec<Student>, house: House) -> Vec<Student>{
    let mut selected = Vec::new();

    for student in students {
        if student.house == house {
            selected.push(student);
        }
    }

    selected
}

pub fn with_sorted_grades(students: Vec<Student>, feature_fn: fn(&Student)->f64) -> Vec<f64> {
    let mut values = feature_to_grades(students, feature_fn);
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    values
}

pub fn feature_to_grades(students: Vec<Student>, feature_fn: fn(&Student)->f64) -> Vec<f64> {
    students
        .iter()
        .map(feature_fn)
        .collect::<Vec<f64>>()
}