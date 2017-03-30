use std::fs::File;
use std::io::{BufRead, BufReader};
mod node;

type Pyramid = Vec<Vec<u64>>;

fn main() {
    // let pyramid = get_pyramid("pyramid.txt");
    let pyramid = get_pyramid("mini.txt");
    let mut root = node::Node::new();

    // populate treeee 
    
    n.print();
    // println!("{:?}", path);
}



pub fn get_pyramid(fname: &str) -> Pyramid {

    let file = BufReader::new(File::open(fname).unwrap());

    file.lines()
        .map( |l| l.unwrap().split(char::is_whitespace)
             .map(|g| g.chars().collect::<String>().parse::<u64>().unwrap())
             .collect() )
        .collect()
}