use chrono::naive::NaiveDate;
use chrono::Local;
use std::fmt::Error;

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
    pub birthday: NaiveDate,
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

//impl Deserialize for Student {
//    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> {
//        let a = deserializer.into();
//    }
//}

impl Student {
    pub fn my_deserialize(tokens: Vec<String>) -> Student {
        Student {
            house: Student::which_house(tokens[0].clone()),
            first_name: tokens[1].clone(),
            last_name: tokens[2].clone(),
            birthday: Student::which_birthday(tokens[3].clone()),
            best_hand: Student::which_hand(tokens[4].clone()),
            arithmancy: Student::parse_f64(tokens[5].clone()),
            astronomy: Student::parse_f64(tokens[6].clone()),
            herbology: Student::parse_f64(tokens[7].clone()),
            defense: Student::parse_f64(tokens[8].clone()),
            divination: Student::parse_f64(tokens[9].clone()),
            muggle: Student::parse_f64(tokens[10].clone()),
            runes: Student::parse_f64(tokens[11].clone()),
            history: Student::parse_f64(tokens[12].clone()),
            transfiguration: Student::parse_f64(tokens[13].clone()),
            potions: Student::parse_f64(tokens[14].clone()),
            creatures: Student::parse_f64(tokens[15].clone()),
            charms: Student::parse_f64(tokens[16].clone()),
            flying: Student::parse_f64(tokens[17].clone())
        }
    }

    pub fn which_house(house: String) -> House {
        match house.as_str() {
            "Ravenclaw" => House::Ravenclaw,
            "Slytherin" => House::Slytherin,
            "Gryffindor" => House::Gryffindor,
            "Hufflepuff" => House::Hufflepuff,
            _ => panic!("Unrecognized house"),
        }
    }

    pub fn which_hand(hand: String) -> Hand {
        match hand.as_str() {
            "Left" => Hand::Left,
            "Right" => Hand::Right,
            _ => panic!("Unrecognized hand"),
        }
    }

    fn parse_f64(str: String) -> f64 {
        str.parse::<f64>().expect("Cannot parse one value")
    }

    fn which_birthday(date: String) -> NaiveDate {
        NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d").expect("Cannot parse date")
    }
}