#![feature(test)]
extern crate test;

fn main() {
    println!("{}", order("is2 Thi1s T4est 3a"));
}

// Description:

// Your task is to sort a given string. Each word in the String will contain a single number. This number is the position the word should have in the result.

// Note: Numbers can be from 1 to 9. So 1 will be the first word (not 0).

// If the input String is empty, return an empty String. The words in the input String will only contain valid consecutive numbers.

// For an input: "is2 Thi1s T4est 3a" the function should return "Thi1s is2 3a T4est"

pub fn order( input: &str ) -> String {
    let split = input.split(" ");
    let mut tokens = Vec::new();

    for s in split {
        tokens.push(s);
    }

    let nums = input.chars().filter_map(|a| a.to_digit(10)).collect::<Vec<_>>();

    if tokens.len() < 2 {
        return input.to_string()
    }

    if nums.len() != tokens.len() {
        println!("Rules of the game weren't followed!");
        return input.to_string()
    }

    // const val: usize = nums.len();
    let mut ordered = vec![String::new(); nums.len()];

    for idx in 0..nums.len() {
        let n: u32 = nums[idx] - 1;
        ordered[n as usize] = tokens[idx as usize].to_string();
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