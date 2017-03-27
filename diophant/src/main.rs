// #![feature(test)]
// extern crate test;

// use test::Bencher;

// #[bench]
// fn actual(b: &mut Bencher) {
// 	b.iter(|| solequa(9000005))
// }

fn main() {
    println!("{:?}", solequa(9000005));
}

// Description:

// In mathematics, a Diophantine equation is a polynomial equation, usually in two or more unknowns, such that only the integer solutions are sought or studied.

// In this kata we want to find all integers x, y (x >= 0, y >= 0) solutions of a diophantine equation of the form

//  x ^ 2 - 4 * y ^ 2 = n
// where the unknowns are x and y and n is a given positive number. Solutions x, y will be given as an array of arrays (Ruby, Python, Clojure, JS, CS, TS)

//  [[x1, y1], [x2, y2] ....]
// as an array of tuples (Haskell, C++, Elixir)

//  [(x1, y1), (x2, y2) ....] or { {x1, y1}, {x2, y2} ....} or [{x1, y1}, {x2, y2} ....]
// as an array of pairs (C, see example tests)

// {{x1, y1}{x2, y2} ....}
// and as a string (Java, C#)

//  "[[x1, y1], [x2, y2] ....]"
// in decreasing order of the positive xi. If there is no solution returns [] or "[]".

// Examples:

// sol_equa(90005) -->  [[45003, 22501], [9003, 4499], [981, 467], [309, 37]]

// sol_equa(90002) --> []

// (Java, C#)

// solEquaStr(90005) --> "[[45003, 22501], [9003, 4499], [981, 467], [309, 37]]"

// solEquaStr(90002) --> "[]"
// Hint: x ^ 2 - 4 y ^ 2 = (x - 2y) (x + 2y).

fn solequa(n: u64) -> Vec<(u64, u64)> {
    let mut ret = Vec::new();
    let mut x: u64;
    let mut y: u64;

    let bound = ((n as f64).sqrt()) as u64 + 1;

    for i in 1..bound {
        if n % i != 0 {
            continue;
        }

        let q = n / i;

        y = (q - i) / 4;
        x = i + 2 * y;

        if (i == x - 2 * y) && (q == x + 2 * y) {
            ret.push((x, y));
        }
    }

    ret
}

// fn solequa(n: u64) -> Vec<(u64, u64)> {
//     let mut ret = Vec::new();
//     let mut x: f64;
//     let mut y: f64 = 1.0;
//     let mut t: f64 = 1.0;

//     let ylimit = ((n as f64 + 6.0) / 4.0).floor();

//     while y <= ylimit {

//         let s = n as f64 + 4.0 * (y * y);
        
//         x = s.sqrt().floor();

//         if ((x - 2.0 * y) * (x + 2.0 * y)) as u64 == n {
//             ret.insert(0, (x as u64, y as u64));
//             t = 2.0;
//         } 
        
//         y += t;
//     }

//     ret
// }


fn testing(n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(solequa(n), exp)
}

#[test]
fn basics_solequa() {
    testing(20, vec![(6, 2)]);
    testing(900000, vec![(112502, 56249), (56254, 28123), (37506, 18747), (22510, 11245), (18762, 9369),
    (12518, 6241), (11270, 5615), (7530, 3735), (6286, 3107), (4550, 2225), (3810, 1845), (2590, 1205),
    (2350, 1075), (1650, 675), (1430, 535), (1150, 325), (1050, 225), (950, 25)]);

    testing(9001, vec![(4501, 2250)]);
    testing(9000001, vec![(4500001, 2250000), (73801, 36870)]);
    testing(90000001, vec![(45000001, 22500000), (6428575, 3214284), (3461545, 1730766), (494551, 247230)]);

    testing(90002, vec![]);

    testing(9004, vec![(2252, 1125)]);
    testing(9000004, vec![(2250002, 1125000), (173090, 86532), (132370, 66168), (10402, 4980)]);
    testing(90000004, vec![(22500002, 11250000), (252898, 126360), (93602, 46560), (22498, 10200)]);

    testing(9005, vec![(4503, 2251), (903, 449)]);
    testing(90005, vec![(45003, 22501), (9003, 4499), (981, 467), (309, 37)]);
    testing(5, vec![(3, 1)]);
    testing(9000005, vec![(4500003, 2250001), (900003, 449999), (642861, 321427), (155187, 77579),
    (128589, 64277), (31107, 15481), (22269, 11033), (4941, 1963)]);

    testing(9009, vec![(4505, 2252), (1503, 750), (647, 320), (505, 248), (415, 202), (353, 170), (225, 102),
    (153, 60), (135, 48), (103, 20), (97, 10), (95, 2)]);

    testing(900000009, vec![(450000005, 225000002), (150000003, 75000000), (50000005, 24999998), (26470597, 13235290), (8823555, 4411752), (2941253, 1470550)]);
}
