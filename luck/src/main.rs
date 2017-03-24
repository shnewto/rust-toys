fn main() {
    println!("Hello, world!");
}

// the function ez_vec takes a static string and a terminating byte and returns an owned Vec<u8> for convenience
// Without it, character-based tests are a pain

#[test]
fn example_tests() {
  // Echo until byte 255 encountered
//   assert_eq!(String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Codewars", 255))).unwrap(), "Codewars");
//   // Echo until byte 0 encountered
//   assert_eq!(String::from_utf8(brain_luck(",[.[-],]", ez_vec("Codewars", 0))).unwrap(), "Codewars");
  // Multiply two numbers
  assert_eq!(brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]), vec![72]);
}

// '>' increment the data pointer (to point to the next cell to the right).
//
// '<' decrement the data pointer (to point to the next cell to the left).
//
// '+' increment (increase by one, truncate overflow: 255 + 1 = 0) the byte at the data pointer.
//
// '-' decrement (decrease by one, treat as unsigned byte: 0 - 1 = 255 ) the byte at the data pointer.
//
// '.' output the byte at the data pointer.
//
// ',' accept one byte of input, storing its value in the byte at the data pointer.
//
// '[' if the byte at the data pointer is zero, then instead of moving the instruction pointer
//     forward to the next command, jump it forward to the command after the matching ] command.
//
// ']' if the byte at the data pointer is nonzero, then instead of moving the instruction pointer
//     forward to the next command, jump it back to the command after the matching [ command.

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {

    for c in code.chars() {
        match c {
            '>' => {

            }
            '<' => {

            }
            '+' => {

            }
            '-' => {

            }
            '.' => {

            }
            ',' => {

            }
            '[' => {

            }
            ']' => {

            }
            _ => {}
        }
        //
    }

    vec![]
  // your solution here
}
