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
    let v = get_histos_feature(
        students.clone(),
        Creatures //arithmancy and creatures  are good candidates
    );

    Page::single(&v).save("histogram.svg").expect("Unable to generate histogram.svg");
    println!("Wrote histogram.svg!");
}

fn get_histos_feature(students: Vec<Student>, feature: Features) -> ContinuousView {
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

    let label = format!("grade repartition in {}", feature.str());
    view_houses(vec)
        .x_label(label.as_str())
}

pub fn scatter(students: Vec<Student>) {
    let ft1 = Defense;
    let ft2 = Astronomy;

    let v = get_scatters_feature(
        students.clone(),
        ft1.clone(),
        ft2.clone()
    );

    Page::single(&v).save("scatter.svg").expect("Unable to generate scatter.svg");
    println!("Wrote scatter.svg!");
}

fn get_scatters_feature(students: Vec<Student>, ft1: Features, ft2: Features) -> ContinuousView{
    let mut vec = Vec::new();

    for house in House::iter() {
        let g = features_to_grade_tuples(house.clone(), students.clone(), ft1.clone(), ft2.clone());

        let s = Scatter::from_slice(&g)
            .style(&scatter::Style::new()
                .colour(house.clone().colour())
                .marker(house.clone().marker()));
        vec.push(s);
    }

    view_houses(vec)
        .x_label(ft1.str())
        .y_label(ft2.str())
}

pub fn pair(students: Vec<Student>) {
    let vec = get_pair_line(students);

    let p = Page::empty();
    for v in vec {
        p.add_plot(&v);
    }
    p.save("pair.svg").expect("Unable to generate pair.svg");
    println!("Wrote scatter.svg");
}

fn get_pair_line(students: Vec<Student>) -> Vec<ContinuousView>{
    let mut vec = Vec::new();

    for ft1 in Features::iter() {
        for ft2 in Features::iter() {
            let v;
            if ft1 == ft2 {
                let label = format!("grade repartition in {}", ft1.str());
                v = get_histos_feature(students.clone(), ft1)
                    .x_label(label.as_str());
            }
            else {
               v = get_scatters_feature(students.clone(), ft1, ft2)
                   .x_label(ft1.str())
                   .y_label(ft2.str());
            }
            vec.push(v);
        }
    }

    vec
}

fn view_houses(vec: Vec<_P>) -> ContinuousView {
    ContinuousView::new()
        .add(&vec[3])
        .add(&vec[2])
        .add(&vec[0])
        .add(&vec[1])
}