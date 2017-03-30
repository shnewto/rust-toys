fn main() {
    println!("Hello, world!");
}

// Description:

// Given a Sudoku data structure with size NxN, N > 0 and √N == integer, write a method to validate if it has been filled out correctly.

// The data structure is a multi-dimensional Array(in Rust: Vec<Vec<u32>>) , ie:

// [
//   [7,8,4,  1,5,9,  3,2,6],
//   [5,3,9,  6,7,2,  8,4,1],
//   [6,1,2,  4,3,8,  7,5,9],

//   [9,2,8,  7,1,5,  4,6,3],
//   [3,5,7,  8,4,6,  1,9,2],
//   [4,6,1,  9,2,3,  5,8,7],

//   [8,7,6,  3,9,4,  2,1,5],
//   [2,4,3,  5,6,1,  9,7,8],
//   [1,9,5,  2,8,7,  6,3,4]
// ]
// Rules for validation

// Data structure dimension: NxN where N > 0 and √N == integer
// Rows may only contain integers: 1..N (N included)
// Columns may only contain integers: 1..N (N included)
// 'Little squares' (3x3 in example above) may also only contain integers: 1..N (N included)

#[derive(Debug)]
struct Sudoku{
    data: Vec<Vec<u32>>,
}


impl Sudoku{
    fn is_valid(&self) -> bool {

        // println!("{:?}", self);

        let len = self.data.len();

        if len < 1 {
            return false
        }

        // sanity check
        for v in &self.data {
            if v.len() != len {
                return false
            }
        }

        let fsqrt = (len as f64).sqrt();
        let usqrt = (len as f64).sqrt() as usize;

        if usqrt as f64 != fsqrt {
            return false
        }

        // first check, boxes
        let mut l_bound = 0;
        let mut u_bound = usqrt;

        while u_bound <= usqrt {

            let mut flags = vec![0; len];

            //rows
            for i in l_bound..u_bound {
                for v in &self.data[l_bound..u_bound] {
                    if v[i] == 0 {
                        return false
                    }
                    let idx = v[i] - 1;
                    if idx as usize >= len {
                        return false
                    }
                    if flags[idx as usize] == 0 {
                        flags[idx as usize] = 1;
                    } else {
                        return false
                    }
                }
            }

            //cols

            l_bound += usqrt;
            u_bound += usqrt;
        }

        // second check, columns
        for i in 0..len {
            let mut flags = vec![0; len];
            for v in &self.data {
                if v[i] == 0 {
                    return false
                }

                let idx = v[i] - 1;

                if idx as usize >= len {
                    return false
                }

                if flags[idx as usize] == 0 {
                    flags[idx as usize] = 1;
                } else {
                    return false
                }
            }

            if flags.contains(&0) {
                return false
            }
        }

        // third check, rows
        for v in &self.data {
            let mut flags = vec![0; len];
            for i in 0..len {

                if v[i] == 0 {
                    return false
                }

                let idx = v[i] - 1;

                if idx as usize >= len {
                    return false
                }

                if flags[idx as usize] == 0 {
                    flags[idx as usize] = 1;
                } else {
                    return false
                }
            }

            if flags.contains(&0) {
                return false
            }
        }

        true
    }
}

#[test]
fn good_sudoku() {
    let good_sudoku_1 = Sudoku{
        data: vec![
                vec![7,8,4, 1,5,9, 3,2,6],
                vec![5,3,9, 6,7,2, 8,4,1],
                vec![6,1,2, 4,3,8, 7,5,9],

                vec![9,2,8, 7,1,5, 4,6,3],
                vec![3,5,7, 8,4,6, 1,9,2],
                vec![4,6,1, 9,2,3, 5,8,7],

                vec![8,7,6, 3,9,4, 2,1,5],
                vec![2,4,3, 5,6,1, 9,7,8],
                vec![1,9,5, 2,8,7, 6,3,4]
            ]
    };

    let good_sudoku_2 = Sudoku{
        data: vec![
                vec![1,4,  2,3],
                vec![3,2,  4,1],

                vec![4,1,  3,2],
                vec![2,3,  1,4],
            ]
    };
    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
}

#[test]
fn bad_sudoku() {
    let bad_sudoku_1 = Sudoku{
        data: vec![
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],

                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],

                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
            ]
    };

    let bad_sudoku_2 = Sudoku{
        data: vec![
                vec![1,2,3,4,5],
                vec![1,2,3,4],
                vec![1,2,3,4],
                vec![1],
            ]
    };

    let bad_sudoku_3 = Sudoku{
        data: vec![
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![2,3,1, 5,6,4, 8,9,7],
                vec![3,1,2, 6,4,5, 9,7,8],

                vec![4,5,6, 7,8,9, 1,2,3],
                vec![5,6,4, 8,9,7, 2,3,1],
                vec![6,4,5, 9,7,8, 3,1,2],

                vec![7,8,9, 1,2,3, 4,5,6],
                vec![8,9,7, 2,3,1, 5,6,4],
                vec![9,7,8, 3,1,2, 6,4,5],
            ]
    };

    assert!(!bad_sudoku_1.is_valid());
    assert!(!bad_sudoku_2.is_valid());
    assert!(!bad_sudoku_3.is_valid());
}


// ---- bad_sudoku stdout ----
// 	[[1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 2, 3, 4, 5, 6, 7, 8, 9]]
// [[1, 2, 3, 4, 5], [1, 2, 3, 4], [1, 2, 3, 4], [1]]
// [[2]]
// [[]]
// [[0]]
// [[1, 2, 3, 4, 5, 6, 7, 8, 9], [2, 3, 1, 5, 6, 4, 8, 9, 7], [3, 1, 2, 6, 4, 5, 9, 7, 8], [4, 5, 6, 7, 8, 9, 1, 2, 3], [5, 6, 4, 8, 9, 7, 2, 3, 1], [6, 4, 5, 9, 7, 8, 3, 1, 2], [7, 8, 9, 1, 2, 3, 4, 5, 6], [8, 9, 7, 2, 3, 1, 5, 6, 4], [9, 7, 8, 3, 1, 2, 6, 4, 5]]