fn main() {
    println!("Hello, world!");
}

// Description:

// Write a function called validBraces that takes a string of braces, and determines if the order of the braces is valid. validBraces should return true if the string is valid, and false if it's invalid.

// This Kata is similar to the Valid Parentheses Kata, but introduces four new characters. Open and closed brackets, and open and closed curly braces. Thanks to @arnedag for the idea!

// All input strings will be nonempty, and will only consist of open parentheses '(' , closed parentheses ')', open brackets '[', closed brackets ']', open curly braces '{' and closed curly braces '}'.

// What is considered Valid? A string of braces is considered valid if all braces are matched with the correct brace. 
// For example:
// '(){}[]' and '([{}])' would be considered valid, while '(}', '[(])', and '[({})](]' would be considered invalid.

// Examples: 
// validBraces( "(){}[]" ) => returns true 
// validBraces( "(}" ) => returns false 
// validBraces( "[(])" ) => returns false 
// validBraces( "([{}])" ) => returns true 

fn valid_braces(s: &str) -> bool {

    let mut v = Vec::new();

    for c in s.chars() {

        match c {
            ')' => {
                    if v.last() == Some(&'(') {
                        v.pop();
                    } else {
                        v.push(c);
                    }
                },
            '}' => {
                    if v.last() == Some(&'{') {
                        v.pop();
                    } else {
                        v.push(c);
                    }
                },
            ']' => {
                    if v.last() == Some(&'[') {
                        v.pop();
                    } else {
                        v.push(c);
                    }
                },
            _ => { v.push(c) }
        }
    }

    if v.len() == 0 {
        true
    } else {
        false
    }
}

#[test]
fn basic_tests() {
  assert_eq!(valid_braces("(){}[]"), true);
  assert_eq!(valid_braces("(}"), false);
  assert_eq!(valid_braces("[(])"), false);
  assert_eq!(valid_braces("([{}])"), true);
}
