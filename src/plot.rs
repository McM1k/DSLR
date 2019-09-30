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
use std::alloc::handle_alloc_error;

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
        .x_label("Creatures repartition");

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
//    let label = format!("grade repartition in {}", feature.str());
//    view_houses(vec)
//        .x_label(label.as_str())
}

pub fn scatter(students: Vec<Student>) {
    let ft1 = Defense;
    let ft2 = Astronomy;

    let s = get_scatters_feature(
        students.clone(),
        ft1.clone(),
        ft2.clone()
    );

    let v = ContinuousView::new()
        .add(&s[3])
        .add(&s[2])
        .add(&s[0])
        .add(&s[1])
        .x_label(ft1.str())
        .y_label(ft2.str());

    Page::single(&v).save("scatter.svg").expect("Unable to generate scatter.svg");
    println!("Wrote scatter.svg!");
}

fn get_scatters_feature(students: Vec<Student>, ft1: Features, ft2: Features) -> Vec<Scatter>{
    let mut vec = Vec::new();

    for house in House::iter() {
        let g = features_to_grade_tuples(house.clone(), students.clone(), ft1.clone(), ft2.clone());

        let s = Scatter::from_slice(&g)
            .style(&scatter::Style::new()
                .colour(house.clone().colour())
                .marker(house.clone().marker()));
        vec.push(s);
    }

    vec
//    view_houses(vec)
//        .x_label(ft1.str())
//        .y_label(ft2.str())
}

pub fn pair(students: Vec<Student>) {
    let s = get_all_scatter(students.clone());
    let h = get_all_histos(students);
    big_oof(s, h);
}

fn big_oof(s: Vec<Vec<Scatter>>, h: Vec<Vec<Histogram>>) {
    Page::empty()
        .dimensions(4000, 4000)

    //FT1 = ARITHMANCY
        .add_plot(&ContinuousView::new().add(&h[0][3]).add(&h[0][2]).add(&h[0][0]).add(&h[0][1]).x_label(Arithmancy.str()))
        .add_plot(&ContinuousView::new().add(&s[1][3]).add(&s[1][2]).add(&s[1][0]).add(&s[1][1]).x_label(Arithmancy.str()).y_label(Astronomy.str()))
        .add_plot(&ContinuousView::new().add(&s[2][3]).add(&s[2][2]).add(&s[2][0]).add(&s[2][1]).x_label(Arithmancy.str()).y_label(Charms.str()))
        .add_plot(&ContinuousView::new().add(&s[3][3]).add(&s[3][2]).add(&s[3][0]).add(&s[3][1]).x_label(Arithmancy.str()).y_label(Creatures.str()))

        .save("pair.svg").expect("Unable to generate pair.svg");
    println!("Wrote pair.svg");
}

fn get_all_histos(students: Vec<Student>) -> Vec<Vec<Histogram>> {
    let mut vec = Vec::new();

    for ft in Features::iter() {
        let h = get_histos_feature(students.clone(), ft);
        vec.push(h);
    }

    vec
}

fn get_all_scatter(students: Vec<Student>) -> Vec<Vec<Scatter>> {
    let mut vec = Vec::new();

    for ft1 in Features::iter() {
        for ft2 in Features::iter() {
            let s = get_scatters_feature(students.clone(), ft1.clone(), ft2);
            vec.push(s);
        }
    }

    vec
}

//fn get_pair_line(students: Vec<Student>) -> Vec<ContinuousView>{
//    let mut vec = Vec::new();
//
//    for ft1 in Features::iter() {
//        for ft2 in Features::iter() {
//            let v;
//            if ft1 == ft2 {
//                let label = format!("grade repartition in {}", ft1.str());
//                v = get_histos_feature(students.clone(), ft1)
//                    .x_label(label.as_str());
//            }
//            else {
//               v = get_scatters_feature(students.clone(), ft1, ft2)
//                   .x_label(ft1.str())
//                   .y_label(ft2.str());
//            }
//            vec.push(v);
//        }
//    }
//
//    vec
//}

//fn view_houses(vec: Vec<_P>) -> ContinuousView {
//    ContinuousView::new()
//        .add(&vec[3])
//        .add(&vec[2])
//        .add(&vec[0])
//        .add(&vec[1])
//}