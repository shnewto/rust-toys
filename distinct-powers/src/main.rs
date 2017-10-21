extern crate num;

use num::bigint::{ToBigUint,BigUint};

fn main() {

    let mut v: Vec<BigUint> = vec![];

    let a_max = 6;
    let b_max = 6;
    
    for a in 2..a_max {
        for b in 2..b_max {
            let mut n = a.to_biguint().unwrap();
            for _ in 2..b+1 {
                n = n.clone() * n.clone();
            }
            // let n = (a as u128).pow(b as u32);
            // println!("{}", n);
            v.push(n);
        }
    }

    v.sort();
    v.dedup();

    println!("N: {}", v.len());
    // println!("{:?}", v);
}