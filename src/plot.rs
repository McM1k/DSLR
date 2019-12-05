use crate::select::*;
use crate::student::Features::*;
use crate::student::House::*;
use crate::student::{Features, House, Student};
use strum::IntoEnumIterator;
//extern crate cpython;
//use cpython::{PyDict, Python};
use plotlib::histogram::{Bins, Histogram};
use plotlib::page::Page;
use plotlib::scatter::Scatter;
use plotlib::style::*;
use plotlib::view::ContinuousView;
use plotlib::{histogram, scatter};

pub fn histogram(students: Vec<Student>) {
    let ft = Creatures;

    let h = get_histos_feature(students, ft.clone());

    let v = ContinuousView::new()
        .add(&h[3])
        .add(&h[2])
        .add(&h[0])
        .add(&h[1])
        .x_label(ft.str());
    Page::single(&v)
        .save("histogram.svg")
        .expect("Unable to generate histogram.svg");
    println!("Wrote histogram.svg!");
}

fn get_histos_feature(students: Vec<Student>, feature: Features) -> Vec<Histogram> {
    let mut vec = Vec::new();

    for house in House::iter() {
        let g = with_sorted_grades(&with_house(&students, &house), &feature);

        let h = Histogram::from_slice(&g, Bins::Count(40))
            .style(&histogram::Style::new().fill(house.clone().colour()));

        vec.push(h);
    }

    vec
}

pub fn plot_loss(data: &Vec<Vec<(f64, f64)>>) {
    let gryf = Scatter::from_slice(&data[0])
        .style(&scatter::Style::new().size(2.0).colour(Gryffindor.colour()));
    let slyt = Scatter::from_slice(&data[1])
        .style(&scatter::Style::new().size(2.0).colour(Slytherin.colour()));
    let rave = Scatter::from_slice(&data[2])
        .style(&scatter::Style::new().size(2.0).colour(Ravenclaw.colour()));
    let huff = Scatter::from_slice(&data[3])
        .style(&scatter::Style::new().size(2.0).colour(Hufflepuff.colour()));

    let v = ContinuousView::new()
        .add(&gryf)
        .add(&slyt)
        .add(&rave)
        .add(&huff)
        .x_label("epoch")
        .y_label("loss");

    Page::single(&v)
        .save("loss.svg")
        .expect("Unable to generate loss.svg");
    println!("Wrote loss.svg!");
}

pub fn scatter(students: Vec<Student>) {
    let ft1 = Arithmancy;
    let ft2 = Creatures;

    let s = get_scatters_feature(students, ft1.clone(), ft2.clone());

    let v = ContinuousView::new()
        .add(&s[3])
        .add(&s[2])
        .add(&s[0])
        .add(&s[1])
        .x_label(ft1.str())
        .y_label(ft2.str());

    Page::single(&v)
        .save("scatter.svg")
        .expect("Unable to generate scatter.svg");
    println!("Wrote scatter.svg!");
}

fn get_scatters_feature(students: Vec<Student>, ft1: Features, ft2: Features) -> Vec<Scatter> {
    let mut vec = Vec::new();

    for house in House::iter() {
        let g = features_to_grade_tuples(&house, &students, &ft1, &ft2);

        let s = Scatter::from_slice(&g).style(
            &scatter::Style::new()
                .colour(house.clone().colour())
                .marker(house.clone().marker()),
        );
        vec.push(s);
    }

    vec
}

pub fn pair(filename: &str) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let locals = PyDict::new(py);
    locals
        .set_item(
            py,
            "plt",
            py.import("matplotlib.pyplot")
                .expect("Cannot import pyplot"),
        )
        .expect("Cannot assign pyplot");
    locals
        .set_item(py, "pd", py.import("pandas").expect("Cannot import pandas"))
        .expect("Cannot assign pandas");
    locals
        .set_item(
            py,
            "sns",
            py.import("seaborn").expect("Cannot import seaborn"),
        )
        .expect("Cannot assign seaborn");
    locals
        .set_item(py, "file", filename)
        .expect("Cannot give the file to python");
    let code = "data = pd.read_csv(file)\n\
    lines = [\"Arithmancy\", \"Astronomy\", \"Herbology\", \"Defense Against the Dark Arts\", \"Divination\", \"Muggle Studies\", \"Ancient Runes\", \"History of Magic\", \"Transfiguration\", \"Potions\", \"Care of Magical Creatures\", \"Charms\", \"Flying\"]\n\
    pal = dict(Gryffindor=\"red\", Hufflepuff=\"yellow\", Slytherin=\"green\", Ravenclaw=\"blue\")\n\
    g = sns.PairGrid(data, vars=lines, hue=\"Hogwarts House\", palette=pal, height=2, dropna=True)\n\
    g.map_lower(plt.scatter, alpha=0.2)\n\
    g.map_diag(plt.hist, alpha=0.6)\n\
    g.add_legend()\n\
    plt.savefig(\"pair.svg\", format=\"svg\")\n\
    ";
    py.run(code, None, Some(&locals))
        .expect("Cannot run python code");
    println!("Wrote pair.svg!");
}
