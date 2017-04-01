fn main() {
    // println!("{:?}", amicable(10000));
    // println!("{:?}", abundant(28123));
    println!("{:?}", non_abundant_sum(28123));
}



fn sum_factors(n: usize) -> usize {

    let bound = n / 2 + 1;    
    let mut sum = 0;

    for i in 1..bound {
        if n % i == 0 {
            sum += i;
        }
    }

    sum
}

fn abundant(limit: usize) -> Vec<usize> {

    let mut ab: Vec<usize> = vec![];

    for n in 1..limit+20 {

        let a = sum_factors(n);

        if a > n {
            ab.push(n);
        }
    }

    ab
}

fn get_all_nums(n: usize) -> Vec<usize> {

    let mut all_nums = vec![];
    
    for i in 1..n+20 {
        all_nums.push(i);
    }

    all_nums
}

fn non_abundant_sum(n: usize) -> usize {
    let abundant_nums = abundant(n);
    let mut all_nums = get_all_nums(n);
    
    for i in 0..abundant_nums.len() {
        for j in i..abundant_nums.len() {
            let s = abundant_nums[i] + abundant_nums[j];
            let index = all_nums.iter().position(|x| *x == s);
            
            match index {
                Some(idx) => { 
                    // println!("{} + {} = {}", abundant_nums[i], abundant_nums[j], s);            
                    all_nums.remove(idx); 
                    }
                None => {}
            }
        }
    }

    let sum: usize = all_nums.iter().sum();
    // println!("{:?}", all_nums );
    sum
}


fn amicable(n: usize) -> usize {
    
    let mut an: Vec<usize> = vec![];

    for a in 1..n {

        let b = sum_factors(a);
        let v = sum_factors(b);

        if (a == v) && (a != b) {
            an.push(a);
            an.push(b);
        }

    }

    an.sort();
    an.dedup();
    let sum: usize = an.iter().sum();

    sum
}
