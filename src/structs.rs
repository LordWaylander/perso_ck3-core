use std::fmt;

#[derive(Clone, Default)]
pub struct Parameters {
    pub education: Option<String>,
    pub level: Option<i8>,
    /// @TODO !
    pub age: Option<i8>
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Default)]
pub struct Education {
    pub  name: String,
    pub level: u8,
    pub points : u16,
    pub bonus: Vec<Bonus>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Personality {
    pub name: String,
    pub points : i16,
    pub bonus: Vec<Bonus>,
    pub incompatible: Vec<String>
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq)]
pub struct Bonus {
    pub name: String,
    pub apttitudes: i8
}

#[derive(Debug, Default)]
pub struct Personnage {
    pub age: Age,
    pub education: Education,
    pub personnality: Vec<Personality>,
    pub statistiques: Statistiques,
    pub points_totaux: u16
}

#[derive(Debug)]
pub struct Age(pub i8);

impl Default for Age {
    fn default() -> Self {
        Age(25)
    }
}

impl Age {
    pub fn get_score_age(&self) -> i32 {
        match self.0 {
            0 => 0,
            1 => 2,
            2 => 4,
            3 => 6,
            25 => 67,
            _ => 67
        }
    }
}

impl fmt::Display for Age {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq)]
pub enum Signe {
    Increment,
    Decrement
}

#[derive(Debug, Default)]
pub struct Statistiques {
    pub diplomatie: Statistique,
    pub martialite: Statistique,
    pub intendance: Statistique,
    pub intrigue: Statistique,
    pub erudition: Statistique,
    pub prouesse: Statistique
}

#[derive(Debug, Clone, Default)]
pub struct Statistique {
    pub base: i8,
    pub bonus: i8
}

impl Statistique {
    fn new() -> Statistique {
        Statistique {
            // valeur de départ de tout personnage créé de base ds le jeu
            base: 5,
            bonus: 0
        }
    }
}

impl Statistiques {
    pub fn new() -> Statistiques {
        Statistiques {
            diplomatie : Statistique::new(),
            martialite: Statistique::new(),
            intendance: Statistique::new(),
            intrigue: Statistique::new(),
            erudition: Statistique::new(),
            prouesse: Statistique::new()
        }
    }

    pub fn incremente_or_decremente_stats(&mut self, stat_name: &str, signe: Signe) -> i32 {
        let modifier = if signe == Signe::Decrement {-1} else {1};
        let val = match stat_name {
            "intrigue" => {
                self.intrigue.base = (self.intrigue.base + modifier).max(0);
                self.intrigue.base
            },
            "diplomatie" => {
                self.diplomatie.base = (self.diplomatie.base + modifier).max(0);
                self.diplomatie.base
            },
            "martialite" => {
                self.martialite.base = (self.martialite.base + modifier).max(0);
                self.martialite.base
            },
            "intendance" => {
                self.intendance.base = (self.intendance.base + modifier).max(0);
                self.intendance.base
            },
            "erudition" => {
                self.erudition.base = (self.erudition.base + modifier).max(0);
                self.erudition.base
            },
            "prouesse" => {
                self.prouesse.base = (self.prouesse.base + modifier).max(0);
                self.prouesse.base
            },
            _ => panic!("erreur incremente_statst, bonus_name = {}",stat_name)
        };

        if stat_name == "prouesse" {
            Statistiques::val_prouesse(val).into()
        } else {
            Statistiques::val_stats(val).into()
        }

    }

    fn val_stats(val : i8) -> i8 {
        match val {
            0..=4 => 2,
            5..=8 => 4,
            9..=12 => 7,
            13..=16 => 11,
            17..=100 => 17, // a vérifier sur l'ensemble des valeurs mais flemme (regardé juqu'a 30)
            _ => 0
       }
    } 

    fn val_prouesse(val : i8) -> i8 {
        match val {
            0..=4 => 1,
            5..=8 => 2,
            9..=12 => 4,
            13..=16 => 7,
            17..=100 => 11, // a vérifier sur l'ensemble des valeurs mais flemme (regardé juqu'a 30)
            _ => 0
       }
    } 

    pub fn calcule_cout_increment(&self, stat_name: &str) -> i32 {
        let val = match stat_name {
            "intrigue" => {
                self.intrigue.base
            },
            "diplomatie" => {
                self.diplomatie.base
            },
            "martialite" => {
                self.martialite.base
            },
            "intendance" => {
                self.intendance.base
            },
            "erudition" => {
                self.erudition.base
            },
            "prouesse" => {
                self.prouesse.base
            },
            _ => panic!("erreur calcule_cout_increment, bonus_name = {}",stat_name)
        };

        if stat_name == "prouesse" {
            Statistiques::val_prouesse(val+1).into()
        } else {
            Statistiques::val_stats(val+1).into()
        }
    }

    pub fn add_bonus_to_stats(&mut self, bonus: Bonus) {
        match bonus.name.as_str() {
            "intrigue" => {
                self.intrigue.bonus += bonus.apttitudes
            },
            "diplomatie" => {
                self.diplomatie.bonus += bonus.apttitudes
            },
            "martialite" => {
                self.martialite.bonus += bonus.apttitudes
            },
            "intendance" => {
                self.intendance.bonus += bonus.apttitudes
            },
            "erudition" => {
                self.erudition.bonus += bonus.apttitudes
            },
            "prouesse" => {
                self.prouesse.bonus += bonus.apttitudes
            },
            _ => panic!("erreur personnalité, bonus_name = {}",bonus.name)
        }
    }
}