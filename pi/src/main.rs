fn main() {
    println!("Hello, world!");
}

// Description:

// The aim of the kata is to try to show how difficult it can be to calculate decimals of an irrational number with a certain precision. We have chosen to get a few decimals of the number "pi" using

// the following infinite series (Leibniz 1646–1716):

// PI / 4 = 1 - 1/3 + 1/5 - 1/7 + ... which gives an approximation of PI / 4.

// http://en.wikipedia.org/wiki/Leibniz_formula_for_%CF%80

// To have a measure of the difficulty we will count how many iterations are needed to calculate PI with a given precision.

// There are several ways to determine the precision of the calculus but to keep things easy we will calculate to within epsilon of your language Math::PI constant. In other words we will stop the iterative process when the absolute value of the difference between our calculation and the Math::PI constant is less than epsilon.

// Your function

// iter_pi(epsilon) must return a tuple :
// (numberOfIterations, approximationOfPi)
// where approximation of PI has 10 decimals

// In Haskell you can use the function "trunc10Dble" (see "Your solution"); in Clojure you can use the function "round" (see "Your solution");in OCaml or Rust the function "rnd10" (see "Your solution") in order to avoid discusssions about the result.

// Examples :

// your function calculates 1000 iterations and 3.140592653839794 but returns:
// iter_pi 0.001 --> (1000, 3.1405926538)
// Unfortunately, this series converges too slowly to be useful, as it takes over 300 terms to obtain a 2 decimal place precision. To obtain 100 decimal places of PI, it was calculated that one would need to use at least 10^50 terms of this expansion!

// About PI : http://www.geom.uiuc.edu/~huberty/math5337/groupe/expresspi.html

fn rnd10(f: f64) -> f64 { (f * 1e10).round() / 1e10 }

fn iter_pi(epsilon: f64) -> (i32, f64) {
    let mut approx: f64 = 0.0;
    let mut k: u32 = 0;
    let numerator: i32 = -1;
    let mut count = 0;

    while (std::f64::consts::PI - (approx * 4.0)).abs() > rnd10(epsilon) {
        approx += numerator.pow(k) as f64/((2.0 * k as f64) + 1.0);
        k += 1;
        count += 1;
    }

    (count, rnd10(approx * 4.0))
}