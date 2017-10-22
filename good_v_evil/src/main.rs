fn main() {
    println!("Hello, world!");
}

#[test]
fn returns_expected() {
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
    assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
}

// On the side of good we have:

// Hobbits: 1
// Men: 2
// Elves: 3
// Dwarves: 3
// Eagles: 4
// Wizards: 10

// On the side of evil we have:

// Orcs: 1
// Men: 2
// Wargs: 2
// Goblins: 2
// Uruk Hai: 3
// Trolls: 5
// Wizards: 10

const HOBBITS_G: usize = 0;
const MEN_G: usize = 1;
const ELVES_G: usize = 2;
const DWARVES_G: usize = 3;
const EAGLES_G: usize = 4;
const WIZARDS_G: usize = 5;

const ORCS_E: usize = 0;
const MEN_E: usize = 1;
const WARGS_E: usize = 2;
const GOBLINS_E: usize = 3;
const URUK_HAI_E: usize = 4;
const TROLLS_E: usize = 5;
const WIZARDS_E: usize = 6;

const HOBBITS_VAL: usize = 1;
const ORCS_VAL: usize = 1;
const MEN_VAL: usize = 2;
const WARGS_VAL: usize = 2;
const GOBLINS_VAL: usize = 2;
const ELVES_VAL: usize = 3;
const DWARVES_VAL: usize = 3;
const URUK_HAI_VAL: usize = 3;
const EAGLES_VAL: usize = 4;
const TROLLS_VAL: usize = 5;
const WIZARDS_VAL: usize = 10;


fn good_vs_evil(good: &str, evil: &str) -> String {

    let team_good: Vec<&str> = good.split(' ').collect();
    let team_evil: Vec<&str> = evil.split(' ').collect();

    let mut score_good: usize = 0;
    let mut score_evil: usize = 0;

    for i in 0..team_evil.len() {
        match i {
            ORCS_E => {
                score_evil += 
                team_evil[i].parse::<usize>().unwrap() * ORCS_VAL;
            }
            MEN_E => {
                score_evil += 
                team_evil[i].parse::<usize>().unwrap() * MEN_VAL;
            }
            WARGS_E => {
                score_evil += 
                team_evil[i].parse::<usize>().unwrap() * WARGS_VAL;
            }
            GOBLINS_E => {
                score_evil += 
                team_evil[i].parse::<usize>().unwrap() * GOBLINS_VAL;
            }
            URUK_HAI_E => {
                score_evil += 
                team_evil[i].parse::<usize>().unwrap() * URUK_HAI_VAL;
            }
            TROLLS_E => {
                score_evil += 
                team_evil[i].parse::<usize>().unwrap() * TROLLS_VAL;
            }
            WIZARDS_E => {
                score_evil += 
                team_evil[i].parse::<usize>().unwrap() * WIZARDS_VAL;
            }    
            _ => {}        
        }
    }

    for i in 0..team_good.len() {
        match i {
            HOBBITS_G => {
                score_good += 
                team_good[i].parse::<usize>().unwrap() * HOBBITS_VAL;
            }
            MEN_G => {
                score_good += 
                team_good[i].parse::<usize>().unwrap() * MEN_VAL;
            }
            ELVES_G => {
                score_good += 
                team_good[i].parse::<usize>().unwrap() * ELVES_VAL;
            }
            DWARVES_G => {
                score_good += 
                team_good[i].parse::<usize>().unwrap() * DWARVES_VAL;
            }
            EAGLES_G => {
                score_good += 
                team_good[i].parse::<usize>().unwrap() * EAGLES_VAL;
            }
            WIZARDS_G => {
                score_good += 
                team_good[i].parse::<usize>().unwrap() * WIZARDS_VAL;
            }    
            _ => {}        
        }
    }    

    if score_good > score_evil { 
        return "Battle Result: Good triumphs over Evil".to_string()
    }          

    if score_evil > score_good {
        return "Battle Result: Evil eradicates all trace of Good".to_string()
    }

    return "Battle Result: No victor on this battle field".to_string()
}