fn main() {
    println!("Hello, world!");
}

// Description:

// Professor Chambouliard hast just discovered a new type of magnet material. He put particles of this material in a box made of small boxes arranged in K rows and N columns as a kind of 2D matrix K x N where K and N are postive integers. He thinks that his calculations show that the force exerted by the particle in the small box (k, n) is:

// v(k, n) = \frac{1}{k(n+1)^{2k}}

// The total force exerted by the first row with k = 1 is:

// u(1, N) = \sum_{n=1}^{n=N}v(1, n) = \frac{1}{1.2^2} + \frac{1}{1.3^2}+...+\frac{1}{1.(N+1)^2}

// We can go on with k = 2 and then k = 3 etc ... and consider:

// S(K, N) = \sum_{k=1}^{k=K}u(k, N) = \sum_{k=1}^{k=K}(\sum_{n=1}^{n=N}v(k, n)) \rightarrow (doubles(maxk, maxn))

// Task:

// To help Professor Chambouliard can we calculate the function doubles that will take as parameter maxk and maxn such that doubles(maxk, maxn) = S(maxk, maxn)? Experiences seems to show that this could be something around 0.7 when maxk and maxn are big enough.


fn vee( k: i32, n: i32) -> f64 {

    1.0 / (k as f64 * ((n + 1) as f64).powf((2*k) as f64))

}


fn you( k: i32, maxn: i32 ) -> f64 {

    let mut s: f64 = 0.0;
    for n in 1..maxn+1 {
        s += vee(k, n as i32);
    }

    s
}

fn doubles( maxk: i32, maxn: i32 ) -> f64 {

    let mut s: f64 = 0.0;
    for k in 1..maxk+1 {
        s += you(k as i32, maxn);
    }

    s
}

fn assert_fuzzy_equals(actual: f64, expected: f64) {
    let merr = 1.0e-10;
    let inrange =
        if expected == 0.0 {
            (actual.abs() <= merr)
        } else {
            ((actual - expected).abs() / expected <= merr)
        };
    if inrange == false {
        println!("Expected value must be near: {:e} but was:{:e}", expected, actual);
    }
    assert_eq!(true, inrange);
}

fn dotest(maxk: i32, maxn: i32, exp: f64) -> () {
    assert_fuzzy_equals(doubles(maxk, maxn), exp);
}

#[test]
fn basic_tests_doubles() {
    dotest(1, 10, 0.5580321939764581);
    dotest(10, 1000, 0.6921486500921933);
    dotest(10, 10000, 0.6930471674194457);
}