use crate::student::Hand;
use chrono::naive::NaiveDate;

#[derive(Clone, Debug)]
pub struct NewStudent {
    pub first_name: String,
    pub last_name: String,
    pub birthday: NaiveDate,
    pub best_hand: Hand,
    pub arithmancy: f64, //.4
    pub astronomy: f64,
    pub herbology: f64,
    pub defense: f64,
    pub divination: f64,
    pub muggle: f64,
    pub runes: f64,
    pub history: f64,
    pub transfiguration: f64,
    pub potions: f64,
    pub creatures: f64,
    pub charms: f64,
    pub flying: f64, //.16
}
