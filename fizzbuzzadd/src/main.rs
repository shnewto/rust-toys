#![feature(test)]
extern crate test;

fn main() {
    println!("Hello, world!");
}

fn multiply_for(num: i32) -> i32 {
    let mut sum = 0;

     for n in 0..num {
        if n % 5 == 0 {
            sum += n
        } else if n % 3 == 0 {
            sum += n
        }
    }

    sum
}

// use test::Bencher;

// #[bench]
// fn picnic_bench_for(picnic: &mut Bencher) {
//     picnic.iter(|| multiply_for(test::black_box(100000)) )
// }

// #[bench]
// fn picnic_bench_while(picnic: &mut Bencher) {
//     picnic.iter(|| multiply_while(test::black_box(100000)) )
// }

#[test]
fn returns_expected() {
  assert_eq!(multiply_while(10), 23);
  assert_eq!(multiply_while(11), 33);
  assert_eq!(multiply_while(6), 8);
}