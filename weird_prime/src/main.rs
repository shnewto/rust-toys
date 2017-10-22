

fn main() {
    // println!("{:?}", pn(7));
}

// fn testing3(n: i64, exp: i64) -> () {
//     assert_eq!(an_over_average(n), exp)
// }

#[test]
fn returns_expected() {
    assert_eq!(count_ones(1), 1);
    assert_eq!(count_ones(10), 8);
    assert_eq!(count_ones(100), 90);

    // assert_eq!(max_pn(1), 5);
    // assert_eq!(max_pn(5), 47);
    // assert_eq!(max_pn(7), 101);

    // testing3(5, 3);
    
}

fn count_ones(n: i64) -> i64 {
    if n == 0 { return 0 }
    let g_s = gn(n);
    let mut count: i64 = 0;
    for elem in g_s { if elem == 1 { count += 1; } }
    
    count
}

// fn pn(n: i64) -> Vec<i64> {
// }

// fn max_pn(n: i64) -> i64 {
//     *pn(n).last().unwrap()
// }

// fn an_over_average(n: i64) -> i64 {
//     n
// }

// fn an(n: i64) -> Vec<i64> {
//     vec![n]
// }

fn an(n: i64) -> i64 {
    match n {
        0 => return 0,
        1 => return 7,
        _ => { let an_val = an(n-1); return an_val + gcd(n, an_val) }
    }
}

fn an_seq(n: i64) -> Vec<i64> {
    let mut v: Vec<i64> = vec![];
    match n {
        0 => return v,
        _ => for i in 1..(n+1) { v.push(an(i)); }
    }

    v
}

fn gn(n: i64) -> Vec<i64> {
    let a_s = an_seq(n);
    let mut g_s: Vec<i64> = vec![];
    let mut last = 6; // hack, 7-6 == 1 at head of list...
    for elem in a_s {
        g_s.push((elem - last).abs());
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
