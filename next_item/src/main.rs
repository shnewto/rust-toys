#![feature(test)]

extern crate test;

use std::fmt::Debug;
use std::option::Option;

fn main() {
    println!("{:?}", next_item( &[1, 2, 3], 2 ));
    println!("{:?}", next_item( &["Joe", "Bob", "Sally"], "Bob" ));
    println!("{:?}", next_item((1..10).collect::<Vec<_>>().as_slice(), 7));
    println!("{:?}", next_item(&[0, 1], 1) );
}

fn next_item<T: PartialEq + Clone + Debug>(input: &[T], val: T) -> Option<T> {
    let index = input.iter().position(|r| *r == val).unwrap();

    if index >= input.len() - 1 {
        None
    } else {
        Some(input[index + 1].clone())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn returns_expected() {
        assert_eq!(next_item(&["Joe", "Bob", "Sally"], "Bob"), Some("Sally"));
        assert_eq!(next_item(&[0, 1], 0), Some(1));
        assert_eq!(next_item(&[0, 1], 1), None);
        assert_eq!(next_item((1..10).collect::<Vec<_>>().as_slice(), 7),
                   Some(8));
    }

    #[bench]
    fn bench_nums(b: &mut Bencher) {
        b.iter(|| assert_eq!(next_item(&[1,2,3,4], 3), Some(4)));
    }

    #[bench]
    fn bench_strings(b: &mut Bencher) {
        b.iter( || assert_eq!( next_item(&["Joe", "Bob", "Sally", "Zen"], "Sally"), Some("Zen") ) );
    }
}