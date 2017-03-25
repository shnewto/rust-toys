fn main() {
    println!("Hello, world!");
}

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
