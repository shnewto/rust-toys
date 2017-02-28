use std::collections::hash_set::HashSet;

fn main() {
    println!("Hello, world!");
}


pub fn destroy(input_sets: Vec<HashSet<char>>) -> String {

    let mut initial = String::from("a b c d e f g h i j k l m n o p q r s t u v w x y z");
    for idx in &input_sets {
        for c in idx {
            if *c != ' ' {
                let s = &c.to_string();
                initial = initial.replace( s, "_" );
            }
        }
    }

    initial
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn basic_test1() {
    let mut input_set: Vec<HashSet<char>> = Vec::new();
    input_set.push(['A', 'b'].iter().cloned().collect());
    input_set.push(['C', 'd'].iter().cloned().collect());
    assert_eq!(destroy(input_set), "a _ c _ e f g h i j k l m n o p q r s t u v w x y z");
}

#[test]
fn basic_test2() {
    let mut input_set: Vec<HashSet<char>> = Vec::new();
    input_set.push(['B', 'b'].iter().cloned().collect());
    input_set.push(['C', 'm', 'f'].iter().cloned().collect());
    assert_eq!(destroy(input_set), "a _ c d e _ g h i j k l _ n o p q r s t u v w x y z");
}