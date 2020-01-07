use crate::select;
use crate::student::Features::*;
use crate::student::{Features, Student};
use std::fmt;

pub struct FeatureData {
    pub count: usize,
    pub mean: f64,
    pub std: f64,
    pub min: f64,
    pub q1: f64,
    pub q2: f64,
    pub q3: f64,
    pub max: f64,
}

impl fmt::Display for FeatureData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:>6}  {:>10.4}  {:>10.4}  {:>10.4}  {:>10.4}  {:>10.4}  {:>10.4}  {:>10.4}",
            self.count, self.mean, self.std, self.min, self.q1, self.q2, self.q3, self.max
        )
    }
}

pub fn describe(students: &[Student]) {
    let arithmancy = get_feature_data(students, &Arithmancy);
    let astronomy = get_feature_data(students, &Astronomy);
    let herbology = get_feature_data(students, &Herbology);
    let defense = get_feature_data(students, &Defense);
    let divination = get_feature_data(students, &Divination);
    let muggle = get_feature_data(students, &Muggle);
    let runes = get_feature_data(students, &Runes);
    let history = get_feature_data(students, &History);
    let transfig = get_feature_data(students, &Transfiguration);
    let potions = get_feature_data(students, &Potions);
    let creatures = get_feature_data(students, &Creatures);
    let charms = get_feature_data(students, &Charms);
    let flying = get_feature_data(students, &Flying);

    println!(
        "{:11} : {:>6}  {:>10}  {:>10}  {:>10}  {:>10}  {:>10}  {:>10}  {:>10}",
        "discipline",
        "count",
        "mean",
        "std dev",
        "minimum",
        "1st quart",
        "2nd quart",
        "3rd quart",
        "maximum"
    );
    println!("{:11} : {}", "arithmancy", arithmancy);
    println!("{:11} : {}", "astronomy", astronomy);
    println!("{:11} : {}", "herbology", herbology);
    println!("{:11} : {}", "defense", defense);
    println!("{:11} : {}", "divination", divination);
    println!("{:11} : {}", "muggle", muggle);
    println!("{:11} : {}", "runes", runes);
    println!("{:11} : {}", "history", history);
    println!("{:11} : {}", "transfig", transfig);
    println!("{:11} : {}", "potions", potions);
    println!("{:11} : {}", "creatures", creatures);
    println!("{:11} : {}", "charms", charms);
    println!("{:11} : {}", "flying", flying);
}

pub fn get_feature_data(students: &[Student], feature: &Features) -> FeatureData {
    let values = select::with_sorted_grades(students, feature);
    let quartiles = quartiles(&values);

    FeatureData {
        count: count(&values),
        mean: mean(&values),
        std: std(&values),
        min: quartiles.0,
        q1: quartiles.1,
        q2: quartiles.2,
        q3: quartiles.3,
        max: quartiles.4,
    }
}

pub fn count(values: &[f64]) -> usize {
    values.len()
}

pub fn mean(values: &[f64]) -> f64 {
    //moyenne
    let mut sum = 0.0;
    for value in values {
        sum += *value;
    }
    sum / count(values) as f64
}

pub fn variance(values: &[f64]) -> f64 {
    let mean = mean(values);
    let sq_mean = mean * mean;
    let mut sum = 0.0;

    for value in values {
        sum += (*value * *value) - sq_mean;
    }

    sum / count(values) as f64
}

pub fn std(values: &[f64]) -> f64 {
    //ecart-type == standard variation/deviation
    variance(values).sqrt()
}

pub fn quartiles(sorted: &[f64]) -> (f64, f64, f64, f64, f64) {
    let count = count(sorted);
    (
        sorted[0],
        sorted[count / 4],
        sorted[count / 2],
        sorted[count * 3 / 4],
        sorted[count - 1],
    )
}
