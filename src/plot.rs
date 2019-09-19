use plotlib::histogram::{Histogram, Bins};
use plotlib::histogram;
use crate::student::{Student, House};
use crate::select::*;
use crate::describe::*;
use plotlib::view::ContinuousView;
use plotlib::page::Page;
use strum::IntoEnumIterator;

pub fn histogram(students: Vec<Student>) {
    let data = vec![
    feature_homogeneity(students.clone(), |s|s.arithmancy),
    feature_homogeneity(students.clone(), |s|s.astronomy),
    feature_homogeneity(students.clone(), |s|s.charms),
    feature_homogeneity(students.clone(), |s|s.creatures),
    feature_homogeneity(students.clone(), |s|s.defense),
    feature_homogeneity(students.clone(), |s|s.divination),
    feature_homogeneity(students.clone(), |s|s.flying),
    feature_homogeneity(students.clone(), |s|s.herbology),
    feature_homogeneity(students.clone(), |s|s.history),
    feature_homogeneity(students.clone(), |s|s.muggle),
    feature_homogeneity(students.clone(), |s|s.potions),
    feature_homogeneity(students, |s|s.runes),
    ];

    let h = Histogram::from_slice(&data.clone(), Bins::Bounds(data))
        .style(&histogram::Style::new());

    let v = ContinuousView::new()
        .add(&h)
        .y_label("variance of each feature");

    Page::single(&v).save("histogram.svg");
}

fn feature_homogeneity(students: Vec<Student>, feature_fn: fn(&Student)->f64) -> f64 {
    let mut vec = Vec::new();

    for house in House::iter() {
        let grades = with_sorted_grades(
            with_house(students.clone(), house), feature_fn);
        vec.push(variance(grades.clone()) + mean(grades));
    }

    variance(vec)
}

pub fn scatter(students: Vec<Student>) {
    //TODO
}

pub fn pair(students: Vec<Student>) {
    //TODO
}
