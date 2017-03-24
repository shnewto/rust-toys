fn main() {
    println!("Hello, world!");
}

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