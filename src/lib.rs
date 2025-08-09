pub mod structs;
use structs::*;
use rand::prelude::*;

const EDUCATION_FILE: &str = include_str!("../ressources/educations.json");
const PERSONNALITIES_FILE: &str = include_str!("../ressources/personnalities.json");
const LIMIT_POINTS: i32 = 400;



pub fn load_data() -> (Vec<Education>, Vec<Personality>) {
    let educations: Vec<Education> = serde_json::from_str(EDUCATION_FILE)
    .expect("error while parsing education data");

    let personnalities: Vec<Personality> = serde_json::from_str(PERSONNALITIES_FILE)
    .expect("error while parsing personality data");

(educations, personnalities)
}

pub fn remove_personnality(
    traits_incompatibles: Vec<String>, 
    personality_bonus: &mut Vec<Personality>, 
    personality_neutral: &mut Vec<Personality>
) -> () {

    traits_incompatibles.into_iter().for_each(
        |value| {
            if let Some(index) = personality_bonus.iter().position(|pers| pers.name == value) {
                personality_bonus.remove(index);
            };

            if let Some(index) = personality_neutral.iter().position(|pers| pers.name == value) {
                personality_neutral.remove(index);
            };
        }
    );
}

fn define_personality(
    personality: &mut Vec<Personality>,
    personnality_personnage: &mut Vec<Personality>, 
    points_personnage: &mut i32
) -> Vec<String> {

    let mut rng = rand::rng();
    let pers_index= rng.random_range(0..personality.len());

    personnality_personnage.push(personality[pers_index].clone());
    *points_personnage += personality[pers_index].points as i32;

    let traits_incompatibles: Vec<String> = personality[pers_index].incompatible.clone();
    personality.remove(pers_index);

    return traits_incompatibles;
}

pub fn generate_personnage(parameters : Parameters) -> Personnage {
    let datas: (Vec<Education>, Vec<Personality>) = load_data();
    let educations: Vec<Education> = datas.0;
    let personalities: Vec<Personality> =  datas.1;

    let mut rng = rand::rng();
    let mut statistiques = Statistiques::new();


    // dbg!(&args);
    let age: Age;
    let mut educs_possible: Vec<Education> = educations;

    let education_level_is_some = parameters.level.is_some();
    let education_is_some = parameters.education.is_some();
    let age_is_some = parameters.age.is_some();
    
    if age_is_some {
        age = Age(parameters.age.unwrap());
    } else {
        age = Age::random();
    }

    //dbg!(&age);

    // education
    if education_is_some {
        match parameters.education.as_ref().unwrap().as_str() {
            "diplomatie" | "martialite" | "intrigue" | "intendance" | "erudition" => {
                if age < Age(16) && age > Age(2) {
                    //todo!();
                    let education_choosen = parameters.education.unwrap();
                    educs_possible = educs_possible.into_iter().filter(|educ| educ.level == 0 && educ.name == education_choosen).collect();
                } else if age >= Age(16) {
                    let education_choosen = parameters.education.unwrap();
                    educs_possible = educs_possible.into_iter().filter(|educ| educ.name == education_choosen).collect();
                } else {
                    todo!("age < 2");
                    // y'a pas d'éducations possibles
                    // car <2 tu y a pas le droit, c'est le jeu qui décidera
                    //educs_possible.clear();
                }
            },
            _ => {
                panic!("Education pas dans la liste")
            }
        }
    }

    if education_level_is_some && age >= Age(16) {
        match parameters.level.unwrap() {
            1 | 2 | 3 | 4 | 5 => {
                let education_level_choosen = parameters.level.clone().unwrap() as u8;
                educs_possible = educs_possible.into_iter().filter(|educ| educ.level == education_level_choosen).collect();
            },
            _ => {
                panic!("level pas dans la liste")
            }
        }
    }


    let score_age = age.get_score_age();
    let mut points_personnage: i32 = score_age + 65; // 65 car le perso a 5 stats partout = 65 pts

    /* Education -> ------------------------------------------------------------------------------ */
    let education_personnage: Option<Education>;

    if age < Age(16) && age > Age(2) {
        // pcq plus haut si défini on met dans un array mais si pas défini...
        if education_is_some {
            let idx = rng.random_range(0..educs_possible.len());
            education_personnage = Some(educs_possible[idx].clone());
        } else {
            educs_possible = educs_possible.into_iter().filter(|educ| educ.level == 0).collect();
            let idx = rng.random_range(0..educs_possible.len());
            education_personnage = Some(educs_possible[idx].clone());
        }
        
       
    } else if age >= Age(16) {
        if education_level_is_some || education_is_some {
            let idx = rng.random_range(0..educs_possible.len());
            education_personnage = Some(educs_possible[idx].clone());
        } else {
            // sinon rien est rentré, donc full random mais avec une certaine chance d'obtenir une bonne éducation
            let percentage = rng.random_range(0..100);

            if percentage < 10 {
                let very_good_education: Vec<Education> = educs_possible.into_iter().filter(|educ| educ.level == 5).collect();
                let educ_index= rng.random_range(0..very_good_education.len());
                education_personnage = Some(very_good_education[educ_index].clone());
            } else if percentage < 90 {
                let good_education: Vec<Education> = educs_possible.into_iter().filter(|educ| educ.level >= 3 && educ.level < 5).collect();
                let educ_index= rng.random_range(0..good_education.len());
                education_personnage = Some(good_education[educ_index].clone());
            } else {
                let education: Vec<Education> =educs_possible.into_iter().filter(|educ| educ.level < 3).collect();
                let educ_index= rng.random_range(0..education.len());
                education_personnage = Some(education[educ_index].clone());
            }
        }
    } else {
        education_personnage = None;
    }

    // dbg!(&education_personnage);

    if let Some(education_personnage) = education_personnage.clone() {
        points_personnage += education_personnage.points as i32;

        for bonus in education_personnage.bonus.clone() {
            statistiques.add_bonus_to_stats(bonus);
        }
    }

    
    
    // dbg!("pts APRES SELECT EDUCATION = {points_personnage}");

    /* Personnality -> ------------------------------------------------------------------------------ */

    let mut personality_bonus: Vec<Personality> = Vec::new();
    let mut personality_neutral: Vec<Personality> = Vec::new();

    if let Some(education_personnage) = education_personnage.clone() {

        for personnality  in personalities.into_iter() {

            let mut match_bonus_education = false;
            // let mut match_no_bonus_education = true;


                for bonus in personnality.bonus.iter() {
                    if education_personnage.name == "martialite" && (bonus.name == education_personnage.name || bonus.name == "prouesse") && bonus.apttitudes > 0 {
                        // car faut prendre la prouesse aussi un seigneur de guerre qui sait pas se battre il est inutile
                        match_bonus_education = true;
                    } else if bonus.name == education_personnage.name && bonus.apttitudes > 0 {
                        match_bonus_education = true;
                    } /*else {
                        match_no_bonus_education = true;
                    }*/
                }

            if match_bonus_education {
                personality_bonus.push(personnality);
            } else /*if match_no_bonus_education*/ { 
                personality_neutral.push(personnality);
            }
        }


    } else {
        personality_neutral = personalities
    }

    // dbg!(&personality_bonus);
    // dbg!("*************************");
    // dbg!(&personality_neutral);

    

    // dbg!("*****BEFORE*****");
    // dbg!("personality_bonus : ");
    // dbg!("{:?}", personality_bonus);
    // dbg!("personality_neutral : ");
    // dbg!("{:?}", personality_neutral);


    // < 3 SI age >= 16, 
    // dessous (RP) age <8 = 0; 8..=10 = 1 ; 10..=12 = 2; >14 = 3
    // a vérifier pour les valeurs en jeu
    //todo!()
    let mut personnality_personnage: Vec<Personality> = Vec::new();
    let limit_number_personnality = 3;
    while personnality_personnage.len() < limit_number_personnality {

        let traits_incompatibles = if let Some(_) = education_personnage && rng.random_range(0..100) < 60{
            define_personality(&mut personality_bonus, &mut personnality_personnage, &mut points_personnage)
        } else {
           define_personality(&mut personality_neutral, &mut personnality_personnage, &mut points_personnage)
        };

        remove_personnality(traits_incompatibles, &mut personality_bonus, &mut personality_neutral);
    }   

    for personality in personnality_personnage.clone() {
        for bonus in personality.bonus {
            statistiques.add_bonus_to_stats(bonus);
        }
    }

    // dbg!("*****AFTER*****");
    // dbg!("personality_bonus : ");
    // dbg!(&personality_bonus);
    // dbg!("personality_neutral : ");
    // dbg!(&personality_neutral);

    /* Statistiques -> ------------------------------------------------------------------------------ */

    // dbg!("INITALIZATION");
    // dbg!("{:?}", statistiques);

    let mut stats = vec![
        "intrigue",
        "diplomatie",
        "martialite",
        "intendance",
        "erudition",
        "prouesse"
    ];

    if let Some(education_personnage) = education_personnage.clone() {
        stats = stats.into_iter().filter(|name|*name != education_personnage.name).collect();
    }

    /*
        C'est pas parfait, exemple :
         *** statistiques ***
            diplomatie : 6
            martialite : 7
            intendance : 20
            intrigue : 9
            erudition : 11
            prouesse : 12
            points_totaux : 390
        on pourrait augmenter la diplomatie de +2 pour avoir 398 pts
        mais en dehors de ça, ça fait le taf
    */

    while points_personnage <  LIMIT_POINTS {
        
        //10% de base d'obtenir +1 dans l'éducation choisie
        let stat_name = if let Some(education_personnage) = education_personnage.clone() {

            let percentage = rng.random_range(0..100);

            let stat_name = if percentage < 10 {
                /*
                    Si martialité, 80% de chances augmenter martialité et 20% prouesse ?
                */
                if education_personnage.name == "martialite" {
                    if rng.random_range(0..100) < 80 {
                        education_personnage.name
                    } else {
                        "prouesse".to_string()
                    }
                } else {
                    education_personnage.name
                }

            } else {
                let index = rng.random_range(0..stats.len());
                stats[index].to_string()
            };

            stat_name


        } else {
            let index = rng.random_range(0..stats.len());
            stats[index].to_string()
        };

        let cout = statistiques.calcule_cout_increment(&stat_name);
       
        if points_personnage+cout <= LIMIT_POINTS {
            let num = statistiques.incremente_or_decremente_stats(&stat_name, Signe::Increment);
            points_personnage += num
        } else if points_personnage >= 390 && points_personnage < LIMIT_POINTS-1{
            // en gros si il reste entre 10 et 2 pts a attribuer autant essayer de rentabiliser un max
            // mais j'ai pas mieux que ce truc pour l'instant
            let mut bool_break = false;
            
            for stat_name in stats.iter() {
                let cout = statistiques.calcule_cout_increment(stat_name);
                if points_personnage+cout <= LIMIT_POINTS {
                    let num = statistiques.incremente_or_decremente_stats(stat_name, Signe::Increment);
                    points_personnage += num;
                    bool_break = false;
                } else {
                    bool_break = true;
                }
            }

            if bool_break {
                break;
            }

        } else {
            break;
        }
    }

    let perso: Personnage = Personnage {
        age : age,
        education: education_personnage.unwrap_or_default(),
        personnality: personnality_personnage,
        statistiques,
        points_totaux: points_personnage as u16
    };

    perso

}

