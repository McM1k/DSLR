use plotlib::histogram::{Histogram, Bins};
use plotlib::histogram;
use plotlib::scatter::Scatter;
use plotlib::scatter;
use crate::student::{Student, House, Features};
use crate::student::House::*;
use crate::select::*;
use crate::describe::*;
use plotlib::view::ContinuousView;
use plotlib::page::Page;
use plotlib::view::View;
use plotlib::style::*;
use strum::IntoEnumIterator;
use crate::student::Features::*;

pub fn histogram(students: Vec<Student>) {
    let h = get_histos_feature(
        students.clone(),
        Creatures //arithmancy and creatures  are good candidates
    );

    let v = ContinuousView::new()
       .add(&h[3])
        .add(&h[2])
        .add(&h[0])
        .add(&h[1])
        .x_label("grade repartition in astronomy");

    Page::single(&v).save("histogram.svg").expect("Unable to generate histogram.svg");
    println!("Wrote histogram.svg!");
}

fn get_histos_feature(students: Vec<Student>, feature: Features) -> Vec<Histogram> {
    let mut vec = Vec::new();

    for house in House::iter() {
        let grades = with_sorted_grades(
            with_house(students.clone(), house.clone()),
            feature.clone()
        );

        let h = Histogram::from_slice(&grades, Bins::Count(100))
            .style(&histogram::Style::new()
                .fill(house.colour()));
        vec.push(h);
    }
    vec
}

pub fn scatter(students: Vec<Student>) {
    let feat1 = Defense;
    let feat2 = Astronomy;

    let s = get_scatters_feature(students.clone(), feat1.clone(), feat2.clone());

    let v = ContinuousView::new()
        .add(&s[0])
        .add(&s[1])
        .add(&s[2])
        .add(&s[3])
        .x_label(feat1.str())
        .y_label(feat2.str());

    Page::single(&v).save("scatter.svg").expect("Unable to generate scatter.svg");
    println!("Wrote scatter.svg!");
}

fn get_scatters_feature(students: Vec<Student>, feature1: Features, feature2: Features) -> Vec<Scatter> {
    let mut vec = Vec::new();

    for house in House::iter() {
        let g = features_to_grade_tuples(house.clone(), students.clone(), feature1.clone(), feature2.clone());

        let s = Scatter::from_slice(&g)
            .style(&scatter::Style::new()
                .colour(house.clone().colour())
                .marker(house.clone().marker()));
        vec.push(s);
    }

    vec
}

pub fn pair(students: Vec<Student>) {
    //TODO
}
