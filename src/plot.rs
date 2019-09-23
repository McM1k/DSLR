use plotlib::histogram::{Histogram, Bins};
use plotlib::histogram;
use crate::student::{Student, House};
use crate::select::*;
use crate::describe::*;
use plotlib::view::ContinuousView;
use plotlib::page::Page;
use strum::IntoEnumIterator;
use plotlib::view::View;
use plotlib::style::Bar;

pub fn histogram(students: Vec<Student>) {
    let hs = get_histos_feature(
        students.clone(),
        |s|s.creatures //arithmancy and creatures  are good candidates
    );

    let v = ContinuousView::new()
       .add(&hs[3])
        .add(&hs[2])
        .add(&hs[0])
        .add(&hs[1])
        .x_label("grade repartition in astronomy");

    Page::single(&v).save("histogram.svg");
    println!("Wrote histogram.svg!");
}

fn get_histos_feature(students: Vec<Student>, feature_fn: fn(&Student)->f64) -> Vec<Histogram> {
    let gryp_grades = with_sorted_grades(
        with_house(students.clone(), House::Gryffindor), feature_fn);
    let gryp_h = Histogram::from_slice(&gryp_grades, Bins::Count(100))
        .style(&histogram::Style::new()
            .fill("DD3355"));
    let slyt_grades = with_sorted_grades(
        with_house(students.clone(), House::Slytherin), feature_fn);
    let slyt_h = Histogram::from_slice(&slyt_grades, Bins::Count(100))
        .style(&histogram::Style::new()
            .fill("33DD55"));
    let rave_grades = with_sorted_grades(
        with_house(students.clone(), House::Ravenclaw), feature_fn);
    let rave_h = Histogram::from_slice(&rave_grades, Bins::Count(100))
        .style(&histogram::Style::new()
            .fill("3355DD"));
    let huff_grades = with_sorted_grades(
        with_house(students.clone(), House::Hufflepuff), feature_fn);
    let huff_h = Histogram::from_slice(&huff_grades, Bins::Count(100))
        .style(&histogram::Style::new()
            .fill("DDBB33"));

    vec![gryp_h, slyt_h, rave_h, huff_h]
}

pub fn scatter(students: Vec<Student>) {
    //TODO
}

pub fn pair(students: Vec<Student>) {
    //TODO
}
