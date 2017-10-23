

#![feature(test)]
extern crate test;

fn main() {
    println!("{:?}", an_seq(10000, vec![]));
    // println!("{:?}", an_over_average(5));
    // println!("{:?}", pn(1));
    // println!("{:?}", max_pn(1));
//     println!("{:?}", gn(10, vec![]));
//     println!("{:?}", count_ones(10));
}

// println!("max_pn(n) : {}", n);

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // #[test]
    // fn returns_expected() {
    //     assert_eq!(count_ones(1), 1);
    //     assert_eq!(count_ones(10), 8);
    //     assert_eq!(count_ones(100), 90);

    //     assert_eq!(max_pn(1), 5);
    //     assert_eq!(max_pn(5), 47);
    //     assert_eq!(max_pn(7), 101);

    //     assert_eq!(an_over_average(5), 3);
    // }

    #[bench]
    fn bench_count_ones(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(count_ones(100), 90);
        });
    }

    #[bench]
    fn bench_max_pn(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(max_pn(7), 101);
        });
    }

    #[bench]
    fn bench_an_over_average(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(an_over_average(5), 3);
        });
    }        
}


fn count_ones(n: i64) -> i64 {
    if n == 0 { return 0 }
    gn(n, vec![]).iter().filter(|&n| *n == 1).count() as i64
}

fn pn(n: i64) -> Vec<i64> {

    let mut count: i64 = n;
    let mut gn_vec: Vec<i64> = vec![];
    loop {
        gn_vec = gn(count, gn_vec);
        gn_vec.sort();
        gn_vec.dedup();        
        gn_vec.remove(0);        
        if gn_vec.len() == n as usize { return gn_vec }
        count += 1;
    }
}

fn max_pn(n: i64) -> i64 {
    *pn(n).last().unwrap()
}

fn an_over_average(n: i64) -> i64 {
    let mut avg: i64 = 0;
    let ao_s = an_over(n);
    let denom = ao_s.len() as i64;
    for elem in ao_s {
        avg += elem;
    }
    
    avg / (denom + 1)
}

fn an_over(n: i64) -> Vec<i64> {
    let pn_vec = pn(n);
    let mut ret: Vec<i64> = vec![];

    for (i, elem) in pn_vec.iter().enumerate() {
        if *elem != 1 {
            ret.push(elem/(i as i64 + 1));
        }
    }

    ret
}

fn an(n: i64) -> i64 {
    match n {
        0 => return 0,
        1 => return 7,
        _ => { 
            let an_val = an(n-1); return an_val + gcd(n, an_val) 
        }
    }
}


fn an_seq(n: i64, mut state: Vec<i64>) -> Vec<i64> {
    match n {
        0 => { return state }
        _ => for i in (state.len() + 1)..(n as usize + 1) { state.push(an(i as i64)); }
    }

    state
}

fn gn(n: i64, state: Vec<i64>) -> Vec<i64> {
    let mut last;
    let len = state.len();

    if  len > 0 {
        last = *state.last().unwrap() - 1; // starting from a previous state
    } else {
        last = 6; // starting from beginning, 7-6 == 1 at head of list...
    }

    let ret_vec = an_seq(n, state);
    let mut gn_vec: Vec<i64> = vec![];

    for elem in ret_vec[len..(n as usize)].iter() {
        let val = (elem - last).abs();
        gn_vec.push(val);
        last = *elem
    }

    gn_vec
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
