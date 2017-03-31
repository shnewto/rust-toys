use std::fs::File;
use std::io::{BufRead, BufReader};
mod node;
use node::*;
use std::cmp;

type Pyramid = Vec<Vec<usize>>;

fn main() {
    // let mut pyramid = get_pyramid("pyramid.txt");
    let mut pyramid = get_pyramid("mini.txt");
    // let mut root = Node { val: pyramid[0][0], l: None, r: None };

    // populate_binary_tree(&mut root, 1, 0, 1, &pyramid, pyramid.len());

    // print_node(root, 0);
    // let max_sum = max_path_tree(root);
    let max_sum = max_path_vec(pyramid[0][0], 1, 0, 1, &pyramid, pyramid.len());

    println!("max sum: {}", max_sum);
}


fn max_path_tree( root: Node ) -> usize {

    let r_max: usize;
    match root.r {
        Some(node) => {
            r_max = max_path_tree(*node);
        }
        None => { r_max = 0; }
    }

    let l_max: usize;
    match root.l {
        Some(node) => {
            l_max = max_path_tree(*node);
        }
        None => { l_max = 0; }
    }

    root.val + cmp::max(l_max, r_max)
}

fn max_path_vec(root: usize, depth: usize, lidx: usize, ridx: usize, pyramid: &Pyramid, max_depth: usize) -> usize {

    if depth == max_depth {
        return root
    }

    let l_max = max_path_vec(pyramid[depth][lidx], depth+1, lidx, ridx, pyramid, max_depth);
    let r_max = max_path_vec(pyramid[depth][ridx], depth+1, ridx, ridx + 1, pyramid, max_depth);

    root + cmp::max(l_max, r_max)
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
        None => {}
    }

    println!("{}:{}", std::iter::repeat(" ").take(depth*4).collect::<String>(), root.val );

    match root.l {
        Some(node) => {
            print_node(*node, depth + 1);
        }
        None => {}
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