

#![feature(test)]
extern crate test;

fn main() {
    println!("an_over_average(9) -- {:?}", an_over_average(9));
    println!("max_pn {:?}", max_pn(2));
    println!("count_ones {:?}", count_ones(2));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn count_ones_returns_expected() {
        assert_eq!(count_ones(1), 1);
        assert_eq!(count_ones(10), 8);
        assert_eq!(count_ones(100), 90);
    }

    #[test]
    fn max_pn_returns_expected() {
        assert_eq!(max_pn(1), 5);
        assert_eq!(max_pn(5), 47);
        assert_eq!(max_pn(7), 101);
    }

    #[test]
    fn an_over_average_returns_expected() {
        assert_eq!(an_over_average(1), 3);
        assert_eq!(an_over_average(5), 3);
        assert_eq!(an_over_average(9), 3);
    }

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
    gn(n).iter().filter(|&n| *n == 1).count() as i64
}

fn pn(n: i64) -> Vec<i64> {
    let mut count: i64 = 2;
    let mut primes: Vec<i64> = vec![];
    let mut n_minus_1: i64 = 7;
    
    loop {
        let curr: i64 = n_minus_1 + gcd(count as i64, n_minus_1);
        let val = (curr - n_minus_1).abs();
      
        if (val != 1) && (primes.iter().find(|&&x| x == val) == None) {
            primes.push(val);   
        }

        if primes.len() == n as usize {
            primes.sort();
            return primes
        }

        n_minus_1 = curr;      
        count = count + 1;
    }
}

fn max_pn(n: i64) -> i64 {
    if n == 0 { return 0 }
    *pn(n).last().unwrap()
}

fn an_over_average(n: i64) -> i64 {
    if n == 0 { return 0 }
    let mut avg: i64 = 0;
    let an_over_vec = an_over(n);
    println!("an_over_vec {:?}", an_over_vec);
    let denom = an_over_vec.len() as i64;
    for elem in an_over_vec {
        avg += elem;
    }
    
    avg / denom 
}

fn an_over(n: i64) -> Vec<i64> {
    let mut count: i64 = 2;
    let mut n_minus_1: i64 = 7;
    let mut ret: Vec<i64> = vec![];
    loop {
        let curr: i64 = n_minus_1 + gcd(count as i64, n_minus_1);
        let val = (curr - n_minus_1).abs();
      
        if val != 1 {
            ret.push(curr/count);   
        }

        if ret.len() == n as usize {
            return ret
        }

        n_minus_1 = curr;      
        count = count + 1;
    }
}

fn an(n: i64) -> Vec<i64> {
    let mut ret_vec: Vec<i64> = vec![7];
    let mut n_minus_1: i64 = 7;
    for i in 2..(n as usize) + 1 {
        let curr: i64 = n_minus_1 + gcd(i as i64, n_minus_1);
        ret_vec.push(curr);
        n_minus_1 = curr;
    }

    ret_vec
}

fn gn(n: i64) -> Vec<i64> {
    let ret_vec = an(n);
    let mut last = 6; // hack to get 1. abb(7-6)
    let mut gn_vec: Vec<i64> = vec![];

    for elem in ret_vec {
        let val = (elem - last).abs();
        gn_vec.push(val);
        last = elem
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
