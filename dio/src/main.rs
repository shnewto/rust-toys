fn main() {
    println!("Hello, world!");
}


fn solequa(n: u64) -> Vec<(u64, u64)> {
    let mut ret = Vec::new();
    let mut x: u64 = 0;
    let mut y: u64;

    let xlimit = n / 2 + 1;

    while x <= xlimit {

        let ylimit: u64 = x / 2 + 1;

        y = 0;

        while y < ylimit {

            let res = (x - 2 * y) * (x + 2 * y);

            if res < n {
                break;
            }

            if res == n {
                ret.insert(0, (x, y));
                break;
            }

            y += 1;
        }

        x += 1;
    }

    ret
}


fn testing(n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(solequa(n), exp)
}


#[test]
fn basics_solequa() {
    testing(5, vec![(3, 1)]);
    testing(20, vec![(6, 2)]); 
    testing(9001, vec![(4501, 2250)]);
    testing(9004, vec![(2252, 1125)]);
    testing(9009, vec![(4505, 2252), (1503, 750), (647, 320), (505, 248), (415, 202), (353, 170), (225, 102), (153, 60), (135, 48), (103, 20), (97, 10), (95, 2)]);
    testing(90005, vec![(45003, 22501), (9003, 4499), (981, 467), (309, 37)]);
    testing(90002, vec![]); 
}