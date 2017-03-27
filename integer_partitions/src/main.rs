use std::env;

// Description:

// From wikipedia https://en.wikipedia.org/wiki/Partition_(number_theory)

// In number theory and combinatorics, a partition of a positive integer n, also called an integer partition, is a way of writing n as a sum of positive integers. Two sums that differ only in the order of their summands are considered the same partition.

// For example, 4 can be partitioned in five distinct ways:

// 4, 3 + 1, 2 + 2, 2 + 1 + 1, 1 + 1 + 1 + 1.

// We can write:

// enum(4) -> [[4],[3,1],[2,2],[2,1,1],[1,1,1,1]] and

// enum(5) -> [[5],[4,1],[3,2],[3,1,1],[2,2,1],[2,1,1,1],[1,1,1,1,1]].

// The number of parts in a partition grows very fast. For n = 50 number of parts is 204226, for 80 it is 15,796,476 It would be too long to tests answers with arrays of such size. So our task is the following:

// 1 - n being given (n integer, 1 <= n <= 50) calculate enum(n) ie the partition of n. We will obtain something like that:
// enum(n) -> [[n],[n-1,1],[n-2,2],...,[1,1,...,1]] (order of array and sub-arrays doesn't matter). This part is not tested.

// 2 - For each sub-array of enum(n) calculate its product. If n = 5 we'll obtain after removing duplicates and sorting:

// prod(5) -> [1,2,3,4,5,6]

// prod(8) -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15, 16, 18]

// If n = 40 prod(n) has a length of 2699 hence the tests will not verify such arrays. Instead our task number 3 is:

// 3 - return the range, the average and the median of prod(n) in the following form (example for n = 5):

// "Range: 5 Average: 3.50 Median: 3.50"

// Range is an integer, Average and Median are float numbers rounded to two decimal places (".2f" in some languages).

// Notes:

// Range : difference between the highest and lowest values.

// Mean or Average : To calculate mean, add together all of the numbers in a set and then divide the sum by the total count of numbers.

// Median : The median is the number separating the higher half of a data sample from the lower half. (https://en.wikipedia.org/wiki/Median)

// Hints:

// Try to optimize your program to avoid timing out.

// Memoization can be helpful but it is not mandatory for being successful.


pub fn range(vec: Vec<isize>) -> isize {
    vec.iter().max().unwrap() - vec.iter().min().unwrap()
}

pub fn average(vec: Vec<isize>) -> f64 {
    let sum: isize = vec.iter().sum();
    sum as f64 / vec.len() as f64
}

pub fn median(vec: Vec<isize>) -> f64 {
    let len = vec.len();
    if len % 2 == 0 {
      let pos1: usize = len / 2;
      let pos2: usize = len / 2 - 1;
      (vec[pos1] as f64 + vec[pos2] as f64) / 2.0
    } else {
      let pos: usize = len / 2;
      vec[pos] as f64
    }
}

pub fn part( n: isize ) -> String {
  // let mut partition_count = 0;
  let mut p = &mut vec![0; n as usize]; // An array to store a partition
  let mut k: isize = 0;  // Index of last element in a partition
  p[k as usize] = n;  // Initialize first partition as number itself

  let mut sub_arrays = Vec::new();
  // This loop first prints current partition, then generates next
  // partition. The loop stops when the current partition has all 1s
  loop {
    // increment count
    // partition_count += 1;
    // push partition onto return vector
    sub_arrays.push(p.clone());

    // Generate next partition

    // Find the rightmost non-one value in p[]. Also, update the
    // rem_val so that we know how much value can be accommodated
    let mut rem_val: isize = 0;
    while k >= 0 && p[k as usize] == 1 {
      rem_val += p[k as usize];
      k -= 1;
    }

    // if k < 0, all the values are 1 so there are no more partitions
    if k < 0 {
        return calculate_kata_values(sub_arrays)
      }

    // Decrease the p[k] found above and adjust the rem_val
    p[k as usize] -= 1;
    rem_val += 1;


    // If rem_val is more, then the sorted order is violeted.  Divide
    // rem_val in differnt values of size p[k] and copy these values at
    // different positions after p[k]
    while rem_val > p[k as usize] {
      p[k as usize + 1] = p[k as usize];
      rem_val = rem_val - p[k as usize];
      k += 1;
    }

    // Copy rem_val to next position and increment position
    k += 1;
    p[k as usize] = rem_val;
  }
}


pub fn calculate_kata_values(partitions: Vec<Vec<isize>>) -> String {

  let mut prod = Vec::new();
  for mut p in partitions {
    p.sort();
    p.retain(|&i| i != 0);
    prod.push(p.iter().fold(1, |a,&b| a*b));
  }
  prod.sort();
  prod.dedup();
  // println!("{:?}", prod);
  format!("Range: {} Average: {:.2} Median: {:.2}", range(prod.clone()), average(prod.clone()), median(prod.clone()))
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let len = args.len();

  if len < 2 {
    println!("No integer argument provided. Exiting");
    return
  } else if len > 2 {
    println!("More than 1 integer argument provided! Ignoring all but the first.");
  }

  let input: isize = args[1].parse().unwrap();

  println!("\n\n{}\n\n", part(input) );
}



#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(part(1), "Range: 0 Average: 1.00 Median: 1.00");
        assert_eq!(part(2), "Range: 1 Average: 1.50 Median: 1.50");
        assert_eq!(part(3), "Range: 2 Average: 2.00 Median: 2.00");
        assert_eq!(part(4), "Range: 3 Average: 2.50 Median: 2.50");
        assert_eq!(part(5), "Range: 5 Average: 3.50 Median: 3.50");
    }
}