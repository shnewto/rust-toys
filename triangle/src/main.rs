use std::fs::File;
use std::io::{BufRead, BufReader};
mod node;
use node::*;

type Pyramid = Vec<Vec<usize>>;

fn main() {
    // let pyramid = get_pyramid("pyramid.txt");
    let pyramid = get_pyramid("mini.txt");
    let mut root = Node { val: pyramid[0][0], l: None, r: None };

    // populate treeee 


    populate_binary_tree(&mut root, 1, 0, 1, &pyramid, pyramid.len());  

    print_node(root, 0);
    root.print();
    // println!("{:?}", root);
}


fn populate_binary_tree(root: &mut Node, depth: usize, lidx: usize, ridx: usize, pyramid: &Pyramid, max_depth: usize) {

    if depth == max_depth {
        return
    }

    let mut l = Node { val: pyramid[depth][lidx], l: None, r: None };
    let mut r = Node { val: pyramid[depth][ridx], l: None, r: None };

    populate_binary_tree(&mut l, depth+1, lidx, ridx, pyramid, max_depth);
    populate_binary_tree(&mut r, depth+1, ridx, ridx + 1, pyramid, max_depth);

    root.l = Some(Box::new(l));
    root.r = Some(Box::new(r));
}

fn print_node( root: Node, depth: usize ) {
    
    match root.r {
        Some(node) => {
            print_node(*node, depth + 1);
        }
        None => return,
    }
    
    println!("{}:{}", std::iter::repeat(" ").take(depth*4).collect::<String>(), root.val );

    match root.l {
        Some(node) => {
            print_node(*node, depth + 1);
        }
        None => return,
    }    
}


pub fn get_pyramid(fname: &str) -> Pyramid {

    let file = BufReader::new(File::open(fname).unwrap());

    file.lines()
        .map( |l| l.unwrap().split(char::is_whitespace)
             .map(|g| g.chars().collect::<String>().parse::<usize>().unwrap())
             .collect() )
        .collect()
}