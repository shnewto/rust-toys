fn main() {
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
}

// Description:

// Consider a sequence u where u is defined as follows:

// The number u(0) = 1 is the first one in u.
// For each x in u, then y = 2 * x + 1 and z = 3 * x + 1 must be in u too.
// There are no other numbers in u.
// Ex: u = [1, 3, 4, 7, 9, 10, 13, 15, 19, 21, 22, 27, ...]

// 1 gives 3 and 4, then 3 gives 7 and 10, 4 gives 9 and 13, then 7 gives 15 and 22 and so on...

// Task:

// Given parameter n the function dbl_linear (or dblLinear...) returns the element u(n) of the ordered (with <) sequence u.

// Example:

// dbl_linear(10) should return 22

// Note:

// Focus attention on efficiency


/// arbitrary big number
static mut A: [u32; 1000000] = [1;1000000];
static mut I: u32 = 0;

fn dbl_linear(n: u32) -> u32{

    let mut v:Vec<u32> = vec![];

    unsafe {
        if I >= n {
            return A[n as usize];
        }
        else {
            v.push(1);
        }
    }

    let mut b = n;

    while b >= 10 {   
        for i in 0..n {
            let x = v[i as usize];
            v.push( 2 * x + 1 );
            v.push( 3 * x + 1 );
        }

        v.sort();
        v.dedup(); 

        v.truncate((n + 1) as usize);

        b = b/10;
    }

    unsafe {
        for i in 0..v.len() {
            A[i as usize] = v[i as usize];
        }

        I = v.len() as u32;
    }

    v[n as usize]
}
#[test]
fn basics_remove_nb() {
    assert_eq!(dbl_linear(10), 22);
    assert_eq!(dbl_linear(20), 57);
    assert_eq!(dbl_linear(30), 91);
    assert_eq!(dbl_linear(50), 175);
    assert_eq!(dbl_linear(100), 447);
    assert_eq!(dbl_linear(1000), 8488);
    assert_eq!(dbl_linear(2000), 19773);
    assert_eq!(dbl_linear(6000), 80914);
    assert_eq!(dbl_linear(60000), 1511311);
    assert_eq!(dbl_linear(600000), 28864270);
}