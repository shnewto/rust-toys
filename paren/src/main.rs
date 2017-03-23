fn main() {
    println!("Hello, world!");
}

fn valid_braces(s: &str) -> bool {

    let mut paren = 0;
    let mut brace = 0;
    let mut bracket = 0;

    for c in s.chars() {
        match c {
            '(' => {
                    paren += 1
                },
            ')' => {
                    paren -= 1
                },
            '{' => {
                    brace += 1
                },
            '}' => {
                    brace -= 1
                },
            '[' => {
                    bracket += 1
                },
            ']' => {
                    bracket += 1
                },
            _ => {}
        }
    }

    if (paren == 0) && (brace == 0) && (bracket == 0) {
        true
    } else {
        false
    }
}

#[test]
fn basic_tests() {
  expect_true("()");
  expect_false("[(])");
}
