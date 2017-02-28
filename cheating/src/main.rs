fn main() {
    // let m = 1000004;
    // remove_nb(m);
    // remove_nb(100);
    // remove_nb(101);
    // remove_nb(102);
}


    // println!("m {} : sum {}", m, seq_sum);
    // return ret;

fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let mut ret:Vec<(i32, i32)> = vec![];

    let seq_sum: i64 = (1..m+1).fold(0i64, |a, b| a + b as i64);

    let lower = m/2;
    let upper = m+1;

    for i in lower..upper {
        let m = seq_sum % i as i64;

        if m != 0 {
            
            let h = i as i64 * m as i64 + i as i64 + m as i64;
            
            if h == seq_sum {
                ret.push((m as i32,i));
                ret.push((i,m as i32));
            }            
        }

    }

    ret.sort_by(|a, b| a.cmp(b));

    ret    
}

#[test]
fn basics_remove_nb() {
    assert_eq!(remove_nb(26), vec![(15, 21), (21, 15)]);
    assert_eq!(remove_nb(100), vec![]);
    assert_eq!(remove_nb(101), vec![(55, 91), (91, 55)]);
    assert_eq!(remove_nb(102), vec![(70, 73), (73, 70)]);
    assert_eq!(remove_nb(110), vec![(70, 85), (85, 70)]);
}