fn main() {

    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));
    println!("{}",dbl_linear(100000));

}

// static mut A: [u32; 1000000] = [1;1000000];
// static mut I: u32 = 0;

fn dbl_linear(n: u32) -> u32 {

    // unsafe {
    //     if I >= n {
    //         return A[n as usize]
    //     }
    // }

    let mut v:Vec<u32> = vec![0; (n+2) as usize];
    v[0] = 1;
    // let mut b = n;

    // while b >= 10 {
        for i in 0..n - 2 {
            let x: u32 = v[i as usize];
            println!("{}", 2 * x + 1);
            let y = 2 * x + 1;
            let mut j = i + 1;

            while j < v.len() as u32 {
                if (y < v[j as usize] as u32) || (v[j as usize] == 0) {
                    v[j as usize] = y;
                    break;
                }
                j += 1;
            }

            let z = 3 * x + 1;

            j = i + 1;

            while j < v.len() as u32 {
                if (z < v[j as usize] as u32) || (v[j as usize] == 0) {
                    v[j as usize] = z;
                    break;
                }
                j += 1;
            }
        }

        // v.sort();
        // v.dedup();

        // v.truncate((n + 1) as usize);

        // b = b/10;
    // }

    // unsafe {
    //     for i in 0..v.len() {
    //         A[i as usize] = v[i as usize];
    //     }

    //     I = v.len() as u32;
    // }

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