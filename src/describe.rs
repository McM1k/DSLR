use crate::student::Student;
use std::fmt;
use crate::select;

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
        write!(f, "{:>6}  {:>10.4}  {:>10.4}  {:>10.4}  {:>10.4}  {:>10.4}  {:>10.4}  {:>10.4}", self.count, self.mean, self.std, self.min, self.q1, self.q2, self.q3, self.max)
    }
}


pub fn describe(students: Vec<Student>) {
    let arithmancy = get_feature_data(students.clone(), |s|s.arithmancy);
    let astronomy = get_feature_data(students.clone(), |s|s.astronomy);
    let herbology = get_feature_data(students.clone(), |s|s.herbology);
    let defense = get_feature_data(students.clone(), |s|s.defense);
    let divination = get_feature_data(students.clone(), |s|s.divination);
    let muggle = get_feature_data(students.clone(), |s|s.muggle);
    let runes = get_feature_data(students.clone(), |s|s.runes);
    let history = get_feature_data(students.clone(), |s|s.history);
    let transfig = get_feature_data(students.clone(), |s|s.transfiguration);
    let potions = get_feature_data(students.clone(), |s|s.potions);
    let creatures = get_feature_data(students.clone(), |s|s.creatures);
    let charms = get_feature_data(students.clone(), |s|s.charms);
    let flying = get_feature_data(students.clone(), |s|s.flying);

    println!("{:11} : {:>6}  {:>10}  {:>10}  {:>10}  {:>10}  {:>10}  {:>10}  {:>10}", "discipline", "count", "mean", "std dev", "minimum", "1st quart", "2nd quart", "3rd quart", "maximum");
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

pub fn get_feature_data(students: Vec<Student>, feature_fn: fn(&Student)->f64) -> FeatureData {
    let values = select::with_sorted_grades(students, feature_fn);
    let quartiles = quartiles(values.clone());

    FeatureData {
        count: count(values.clone()),
        mean: mean(values.clone()),
        std: std(values),
        min: quartiles.0,
        q1: quartiles.1,
        q2: quartiles.2,
        q3: quartiles.3,
        max: quartiles.4,
    }
}

pub fn count(values: Vec<f64>) -> usize {
    values.len()
}

pub fn mean(values: Vec<f64>) -> f64 { //moyenne
    let mut sum = 0.0;
    for value in values.clone() {
        sum += value;
    }
    sum / count(values) as f64
}

pub fn variance(values: Vec<f64>) -> f64 {
    let mean = mean(values.clone());
    let sq_mean = mean * mean;
    let mut sum = 0.0;

    for value in values.clone() {
        sum += (value * value) - sq_mean;
    }

    sum / count(values) as f64
}

pub fn std(values: Vec<f64>) -> f64 { //ecart-type
    variance(values).sqrt()
}

pub fn quartiles(values: Vec<f64>) -> (f64, f64, f64, f64, f64) {
    let count = count(values.clone());
    (values[0],
     values[count / 4],
     values[count / 2],
     values[count * 3 / 4],
     values[count - 1])
}