fn main() {
    // let m = 1000004;
    // remove_nb(m);
    // remove_nb(100);
    // remove_nb(101);
    // remove_nb(102);
}


// Description:

// A friend of mine takes a sequence of numbers from 1 to n (where n > 0).
// Within that sequence, he chooses two numbers, a and b.
// He says that the product of a and b should be equal to the sum of all numbers in the sequence, excluding a and b.
// Given a number n, could you tell me the numbers he excluded from the sequence?
// The function takes the parameter: n (don't worry, n is always strictly greater than 0 and small enough so we shouldn't have overflow) and returns an array of the form:

// [(a, b), ...] or [[a, b], ...] or {{a, b}, ...} or or [{a, b}, ...]
// with all (a, b) which are the possible removed numbers in the sequence 1 to n.

// [(a, b), ...] or [[a, b], ...] or {{a, b}, ...} or ...will be sorted in increasing order of the "a".

// It happens that there are several possible (a, b). The function returns an empty array if no possible numbers are found which will prove that my friend has not told the truth! (Go: in this case return nil).

// (See examples for each language in "RUN EXAMPLES")

// Examples:

// removNb(26) should return [(15, 21), (21, 15)]
// or

// removNb(26) should return { {15, 21}, {21, 15} }
// or

// removeNb(26) should return [[15, 21], [21, 15]]
// or

// removNb(26) should return [ {15, 21}, {21, 15} ]
// or

// in C:
// removNb(26) should return  **an array of pairs {{15, 21}{21, 15}}**
// tested by way of strings.

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