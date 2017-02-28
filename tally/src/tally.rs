use std::collections::HashMap;

// var scoreToTally = function(score){
// pub fn scoreToTally(score:i32) -> String {
    let tallys: HashMap<i32, &str> = 
    [(1, "a"),
     (2, "b"),
     (3, "c"),
     (4, "d"),
     (5, "e")].iter().cloned().collect();

     if tallys.contains_key(&score) {
         tallys[&score].to_string()
     } else {
        let mut retstr = String::new();
        let mut val: i32 = score;

        
        loop {
            val = val - 5;
            
            if val < 0 {
                retstr += tallys[&(5+val)];
                break;
            } else {
                retstr += "e <br>";
            }

            if val == 0 { break; }
        }

        retstr
     }
}

// fn main() {
//     println!("{:?}", score_to_tally(100) );
// }

#[test]
fn basic_test1() {
    assert_eq!(score_to_tally(3), "c");
}

#[test]
fn basic_test2() {
    assert_eq!(score_to_tally(4), "d");
}

#[test]
fn basic_test3() {
    assert_eq!(score_to_tally(5), "e");
}

#[test]
fn basic_test4() {
    assert_eq!(score_to_tally(10),"e <br>e <br>");
}

#[test]
fn basic_test5() {
    assert_eq!(score_to_tally(9),"e <br>d");
}
