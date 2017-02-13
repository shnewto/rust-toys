#![feature(test)]
extern crate test;
extern crate regex;

use regex::Regex;

fn main() {
    println!("{}", order("is2 Thi1s T4est 3a"));
    // order("is2 Thi1s T4est 3a");
}

pub fn order( input: &str ) -> String {
// pub fn order( input: &str ) {

    let split = input.split(" ");
    let mut tokens = Vec::new();
    let mut nums: Vec<usize> = Vec::new();

    for s in split {
        tokens.push(s);
    }

    if tokens.len() < 2 {
        return input.to_string()
    }

    let re = Regex::new(r"\d+").unwrap();

    for cap in re.captures_iter(input) {
        let num = &cap[0].to_string();
        nums.push(num.parse::<usize>().unwrap());
    }

    if nums.len() != tokens.len() {
        println!("Rules of the game weren't followed!");
        return input.to_string()
    }
    const val: usize = nums.len();
    let mut ordered: [String; val];

    for idx in 0..tokens.len() {
        // println!("ordered.insert({}, {})", nums[idx], t);
        ordered[nums[idx]] = tokens[idx].to_string();
    }

    let mut retval = String::new();

    for o in ordered {
        retval += &o;
        retval += " ";
    }

    let r = retval.trim();

    r.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }

    // #[bench]
    // fn bench_order(b: &mut Bencher) {
    //     let bench_string = "fir6st at12 g11ood 5the someth13ing som3ething Suck1ing i4s 2at becom9ing 8to s10orta ste7p";
    //     let answer = "Suck1ing 2at som3ething i4s 5the fir6st ste7p 8to becom9ing s10orta g11ood at12 someth13ing";
    //     b.iter(|| {
    //         assert_eq!(order(bench_string), answer);
    //     });
    // }
}