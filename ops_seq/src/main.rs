
fn main() {
    println!("{}", calc(vec![ -2, -1, 0, 1, 2]));
}


#[test]
fn tests() {
    assert_eq!(calc(vec![0, 2, 1, -6, -3, 3]), 31);
    assert_eq!(calc(vec![0]), 0);
    assert_eq!(calc(vec![1, 1, 1, 1, 1]), 5);
    assert_eq!(calc(vec![1, 1, -9, 9, 16, -15, -45, -73, 26]), 1665);
    assert_eq!(calc(vec![1, -1, 10, -9, 16, 15, 45, -73, -26]), 2584);
    assert_eq!(calc(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
    assert_eq!(calc(vec![-5, -5, -5, -5, -5, -5, -5]), -45);
}
// 1. { -2, -1, 0, 1 * 1, 2 * 2 }
// 2. { -2, -1, 0 * 3, 1, 4 }
// 3. { -2, -1, 0, 1, -1 * 4 }
// 4. -6

fn calc(array: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for i in 0..array.len() {
        let mut a = array[i];
        if a > 0 { a = a * a; }
        if (i + 1) % 3 == 0 { a = a * 3; }
        if (i + 1) % 5 == 0 { a = a * -1; }
        sum += a;
    }
    sum
}