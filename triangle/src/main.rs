use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let pyramid = get_pyramid("pyramid.txt");
    let mut path_cost = 0;

    let height = pyramid.len();

    let mut idx = 0;

    path_cost += pyramid[0][0];

    for i in 0..(height - 1) {
        if pyramid[i+1][idx+1] > pyramid[i+1][idx] {
            path_cost += pyramid[i+1][idx+1];
            idx = idx + 1;
        } else {
            path_cost += pyramid[i+1][idx];
        }
    }

    println!("{:?}", path_cost);
}

type Pyramid = Vec<Vec<u64>>;

pub fn get_pyramid(fname: &str) -> Pyramid {

    let mut file = BufReader::new(File::open(fname).unwrap());

    let mut file_string = String::new();
    file.read_line(&mut file_string).unwrap();

    let mut pyramid: Pyramid = vec![vec![]];

    pyramid = file.lines()
        .map( |l| l.unwrap().split(char::is_whitespace)
             .map(|g| g.chars().collect::<String>().parse::<u64>().unwrap())
             .collect() )
        .collect();

    pyramid
}