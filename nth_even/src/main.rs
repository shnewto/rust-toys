fn main() {
    println!("555: {}", nth_even(555));
    println!("0: {}", nth_even(0));
    println!("2: {}", nth_even(2));
}


fn nth_even(n: u32) -> u32 {
    if n > 1 {
        n * 2 - 2
    } else {
        0
    }
}
    