fn main() {
    println!("{}", count_ones(42));
}

#[test]
fn testing1(n: i64, exp: i64) -> () {
    assert_eq!(count_ones(n), exp)
}

#[test]
fn testing2(n: i64, exp: i64) -> () {
    assert_eq!(max_pn(n), exp)
}

#[test]
fn testing3(n: i64, exp: i64) -> () {
    assert_eq!(an_over_average(n), exp)
}

#[test]
fn returns_expected() {
        
    testing1(1, 1);
    testing1(10, 8);
    testing1(100, 90);

    testing2(1, 5);
    testing2(5, 47);
    testing2(7, 101);

    testing3(5, 3);
    
}

fn count_ones(n: i64) -> i64 {
    let s = gn(n);

    println!("{:?}", s.into_iter().find(|&x| x == 1));

    3
}

// fn max_pn(n: i64) -> i64 {
//     n
// }

// fn an_over_average(n: i64) -> i64 {
//     n
// }

// fn an(n: i64) -> Vec<i64> {
//     vec![n]
// }

fn an(n: i64) -> i64 {
    match n {
        1 => return 7,
        _ => { return an(n-1) + gcd(n, an(n-1)) }
    }
}

fn an_seq(n: i64) -> Vec<i64> {
    let mut v: Vec<i64> = vec![];
    for i in 0..n { v.push(an(i)); }
    v
}

fn gn(n: i64) -> Vec<i64> {
    let a_s = an_seq(n);
    let mut g_s: Vec<i64> = vec![1];

    let mut last = 7;
    for elem in a_s {
        if elem != last { g_s.push((elem - last).abs()) }
        last = elem
    }

    g_s
}

fn gcd(v1: i64, v2: i64) -> i64 {
    let mut x = v1;
    let mut y = v2;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn gcd_vec(x: i64, vals: Vec<i64>) -> i64 {
    if vals.len() == 0 { return x }
    let mut greatest = x;
    for v in vals {
        greatest = gcd(v, greatest);
    }
    greatest
}