use crate::student::House::Gryffindor;
use chrono::naive::NaiveDate;
use plotlib::style::Marker;
use strum_macros::EnumIter;

#[derive(Clone, Debug, EnumIter, PartialEq)]
pub enum House {
    Gryffindor,
    Slytherin,
    Ravenclaw,
    Hufflepuff,
}

impl House {
    pub fn colour(&self) -> &str {
        match self {
            House::Gryffindor => "#DD3355",
            House::Slytherin => "#33DD55",
            House::Ravenclaw => "#3355DD",
            House::Hufflepuff => "#DDBB33",
        }
    }

    pub fn marker(&self) -> Marker {
        match self {
            House::Gryffindor => Marker::Square,
            House::Slytherin => Marker::Cross,
            House::Ravenclaw => Marker::Circle,
            House::Hufflepuff => Marker::Circle,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Hand {
    Left,
    Right,
}

#[derive(Clone, Debug, EnumIter, PartialEq)]
pub enum Features {
    Arithmancy,
    Astronomy,
    Charms,
    Creatures,
    Defense,
    Divination,
    Flying,
    Herbology,
    History,
    Muggle,
    Potions,
    Runes,
    Transfiguration,
}

impl Features {
    pub fn func(&self) -> fn(&Student) -> f64 {
        match self {
            Features::Arithmancy => |s| s.arithmancy,
            Features::Astronomy => |s| s.astronomy,
            Features::Charms => |s| s.charms,
            Features::Creatures => |s| s.creatures,
            Features::Defense => |s| s.defense,
            Features::Divination => |s| s.divination,
            Features::Flying => |s| s.flying,
            Features::Herbology => |s| s.herbology,
            Features::History => |s| s.history,
            Features::Muggle => |s| s.muggle,
            Features::Potions => |s| s.potions,
            Features::Runes => |s| s.runes,
            Features::Transfiguration => |s| s.transfiguration,
        }
    }
    pub fn str(&self) -> &str {
        match self {
            Features::Arithmancy => "Arithmancy",
            Features::Astronomy => "Astronomy",
            Features::Charms => "Charms",
            Features::Creatures => "Care of magical creatures",
            Features::Defense => "Defense against the dark arts",
            Features::Divination => "Divination",
            Features::Flying => "Flying",
            Features::Herbology => "Herbology",
            Features::History => "History of magic",
            Features::Muggle => "Muggle studies",
            Features::Potions => "Potions",
            Features::Runes => "Ancient Runes",
            Features::Transfiguration => "Transfiguration",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Student {
    pub house: House,
    pub first_name: String,
    pub last_name: String,
    pub birthday: NaiveDate,
    pub best_hand: Hand,
    pub arithmancy: f64, //.5
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
    pub flying: f64, //.17
}

//impl Deserialize for Student {
//    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> {
//        let a = deserializer.into();
//    }
//}

impl Student {
    pub fn my_deserialize(tokens: Vec<String>) -> Student {
        Student {
            house: Student::which_house(tokens[1].clone()),
            first_name: tokens[2].clone(),
            last_name: tokens[3].clone(),
            birthday: Student::which_birthday(tokens[4].clone()),
            best_hand: Student::which_hand(tokens[5].clone()),
            arithmancy: Student::parse_f64(tokens[6].clone()),
            astronomy: Student::parse_f64(tokens[7].clone()),
            herbology: Student::parse_f64(tokens[8].clone()),
            defense: Student::parse_f64(tokens[9].clone()),
            divination: Student::parse_f64(tokens[10].clone()),
            muggle: Student::parse_f64(tokens[11].clone()),
            runes: Student::parse_f64(tokens[12].clone()),
            history: Student::parse_f64(tokens[13].clone()),
            transfiguration: Student::parse_f64(tokens[14].clone()),
            potions: Student::parse_f64(tokens[15].clone()),
            creatures: Student::parse_f64(tokens[16].clone()),
            charms: Student::parse_f64(tokens[17].clone()),
            flying: Student::parse_f64(tokens[18].clone()),
        }
    }

    pub fn deser_new(tokens: Vec<String>) -> Student {
        Student {
            house: Gryffindor,
            first_name: tokens[2].clone(),
            last_name: tokens[3].clone(),
            birthday: Student::which_birthday(tokens[4].clone()),
            best_hand: Student::which_hand(tokens[5].clone()),
            arithmancy: Student::parse_f64(tokens[6].clone()),
            astronomy: Student::parse_f64(tokens[7].clone()),
            herbology: Student::parse_f64(tokens[8].clone()),
            defense: Student::parse_f64(tokens[9].clone()),
            divination: Student::parse_f64(tokens[10].clone()),
            muggle: Student::parse_f64(tokens[11].clone()),
            runes: Student::parse_f64(tokens[12].clone()),
            history: Student::parse_f64(tokens[13].clone()),
            transfiguration: Student::parse_f64(tokens[14].clone()),
            potions: Student::parse_f64(tokens[15].clone()),
            creatures: Student::parse_f64(tokens[16].clone()),
            charms: Student::parse_f64(tokens[17].clone()),
            flying: Student::parse_f64(tokens[18].clone()),
        }
    }

    pub fn set_feature(&mut self, value: f64, ft: &Features) {
        match ft {
            Features::Arithmancy => self.arithmancy = value,
            Features::Astronomy => self.astronomy = value,
            Features::Charms => self.charms = value,
            Features::Creatures => self.creatures = value,
            Features::Defense => self.defense = value,
            Features::Divination => self.divination = value,
            Features::Flying => self.flying = value,
            Features::Herbology => self.herbology = value,
            Features::History => self.history = value,
            Features::Muggle => self.muggle = value,
            Features::Potions => self.potions = value,
            Features::Runes => self.runes = value,
            Features::Transfiguration => self.transfiguration = value,
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

    pub fn parse_f64(str: String) -> f64 {
        if str.eq(&"".to_string()) {
            return 0.0;
        }
        str.parse::<f64>().expect("Cannot parse one value")
    }

    pub fn which_birthday(date: String) -> NaiveDate {
        NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d").expect("Cannot parse date")
    }
}
