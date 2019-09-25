use plotlib::histogram::*;
use plotlib::histogram;
use crate::student::{Student, House};
use crate::select::*;
use crate::describe::*;
use plotlib::view::ContinuousView;
use plotlib::page::Page;
use plotlib::view::View;
use plotlib::style::*;
use strum::IntoEnumIterator;

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
    let mut vec = Vec::new();
    for house in House::iter() {
        let grades = with_sorted_grades(
            with_house(students.clone(), house.clone()),
            feature_fn
        );

        let h = Histogram::from_slice(&grades, Bins::Count(100))
            .style(&histogram::Style::new()
                .fill(match house {
                    House::Gryffindor => "#DD3355",
                    House::Slytherin => "#33DD55",
                    House::Ravenclaw => "#3355DD",
                    House::Hufflepuff => "#DDBB33",
                }));
        vec.push(h);
    }
    vec
}

pub fn scatter(students: Vec<Student>) {
    //TODO
}

pub fn pair(students: Vec<Student>) {
    //TODO
}
