
fn rnd10(f: f64) -> f64 { (f * 1e10).round() / 1e10 }

fn iter_pi(epsilon: f64) -> (i32, f64) {
    let mut approx: f64 = 0.0;
    let mut k: u32 = 0;
    let numerator: i32 = -1;
    let mut count = 0;

    while (std::f64::consts::PI - (approx * 4.0)).abs() > rnd10(epsilon) {
        approx += numerator.pow(k) as f64/((2.0 * k as f64) + 1.0);
        k += 1;
        count += 1;
    }

    (count, rnd10(approx * 4.0))
}

fn main()
{
     println!("{:?}", iter_pi(0.01));
}

fn testing(epsilon: f64, exp: (i32, f64)) -> () {
    assert_eq!(iter_pi(epsilon), exp)
}

#[test]
fn tests_iter_pi() {
    testing(0.1, (10, 3.0418396189));
    testing(0.01,  (100, 3.1315929036));
    testing(0.001,  (1000, 3.1405926538));
    testing(0.0001,  (10000, 3.1414926536));
    testing(0.00001, (100001, 3.1416026535));
    testing(0.000001,  (1000001, 3.1415936536));
    testing(0.05,  (20, 3.0916238067));
}
// fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
//     if input.is_empty() { 
//         vec![]       
//     } else {
//         let pos: i32 = input.clone().into_iter().filter(|&i|i > 0).collect::<Vec<_>>().len() as i32;
//         let neg: i32 = input.clone().into_iter().filter(|&i|i < 0).collect::<Vec<_>>().iter().sum();
        
//         vec![pos, neg]        
//     }
// }

// fn main() {
//     let test_data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
//     println!("{:?}", count_positives_sum_negatives(test_data1));
// }

// #[test]
// fn returns_expected() {
//     let test_data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
//     let expected1 = vec![10, -65];
//     assert_eq!(count_positives_sum_negatives(test_data1), expected1);    
// }

// #[test]
// fn empty() {
//     let expected1 = vec![10, -65];
//     assert_eq!(count_positives_sum_negatives([]), []);    
// }