use chrono::DateTime;
use chrono::Local;

pub enum House {
    Ravenclaw,
    Slytherin,
    Gryffindor,
    Hufflepuff,
}

pub enum Hand {
    Left,
    Right,
}

pub struct Student {
    pub house: House,
    pub first_name: String,
    pub last_name: String,
    pub birthday: chrono::DateTime<Local>,
    pub best_hand: Hand,
    pub arithmancy: f64,
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
    pub flying: f64,
}

impl Deserialize for Student {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> {

    }
}