fn main() {
    println!("yolo world");
}



// the function ez_vec takes a static string and a terminating byte and returns an owned Vec<u8> for convenience
// Without it, character-based tests are a pain

fn ez_vec(s: &str, term_byte: u8) -> Vec<u8> {
    let mut v: Vec<u8> = vec![0; s.len() + 1];

    let mut idx = 0;
    for b in s.bytes() {
        v[idx] = b;
        idx += 1;
    }
    v[s.len()] = term_byte;

    v
}

#[test]
fn t1() {
// Echo until byte 255 encountered
  assert_eq!(String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Codewars", 255))).unwrap(), "Codewars");
}

#[test]
fn t2() {
// Echo until byte 0 encountered
  assert_eq!(String::from_utf8(brain_luck(",[.[-],]", ez_vec("Codewars", 0))).unwrap(), "Codewars");
}

#[test]
fn t3() {
// Multiply two numbers
  assert_eq!(brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]), vec![72]);
}

#[test]
fn t4() {
// Divide two numbers
  assert_eq!(brain_luck(",>,<[>[->+>+<<]>[-<<-[>]>>>[<[-<->]<[>]>>[[-]>>+<]>-<]<<]>>>+<<[-<<+>>]<<<]>>>>>[-<<<<<+>>>>>]<<<<<.", vec![12, 2]), vec![6]);
}


// fn t5() {

// }


fn tape_forward(data:&mut Vec<u8>, dptr: &mut usize) {
    if *dptr == (data.len() - 1) {
        data.push(0);
    }

    *dptr += 1;
}

fn tape_back(data:&mut Vec<u8>, dptr: &mut usize) {
    if *dptr == 0 {
        data.insert(0,0);
    } else {
        *dptr -= 1;
    } 
}

fn tape_val_inc(data:&mut Vec<u8>, dptr: usize) {
    if data[dptr] == 255 {
        data[dptr] = 0;
    } else {
        data[dptr] += 1;
    } 
}

fn tape_val_dec(data:&mut Vec<u8>, dptr: usize) {
    if data[dptr] == 0 {
        data[dptr] = 255;
    } else {
        data[dptr] -= 1;
    }
}

fn tape_write(data:&mut Vec<u8>, dptr: usize, input: &Vec<u8>, input_ptr: &mut usize) {
     data[dptr] = input[*input_ptr];
    *input_ptr += 1;
}

fn tape_seek_forward(instructions: &Vec<char>, data: u8, instr_ptr: &mut usize) {
    if data == 0 {
        let mut found = false;
        let mut mine = true;

        *instr_ptr += 1;

        while found == false {
            match instructions[*instr_ptr] {
                '[' => {
                    mine = false;
                }
                ']' => {
                    if mine == true {
                        found = true;
                    } else {
                        mine = true;
                    }
                }                            
                _ => {}
            }
            *instr_ptr += 1;                    
        }
    } else {
        *instr_ptr += 1;
    }
}

fn tape_seek_back(instructions: &Vec<char>, data: u8, instr_ptr: &mut usize) {
    if data != 0 {
        let mut found = false;
        let mut mine = true;

        *instr_ptr -= 1;

        while found == false {
            match instructions[*instr_ptr] {
                ']' => {
                    mine = false;
                    *instr_ptr -= 1;
                }
                '[' => {
                    if mine == true {
                        found = true;
                        *instr_ptr += 1;
                    } else {
                        mine = true;
                        *instr_ptr -= 1;
                    }
                }                            
                _ => {
                    *instr_ptr -= 1;
                }
            }
        }
    } else {
        *instr_ptr += 1;
    }
}

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    // println!("code: {:?} -- input: {:?}", code, input);
    let instructions: Vec<char> = code.chars().collect();
    let mut dptr = 0;
    let mut instr_ptr = 0;
    let mut input_ptr = 0;
    let mut output = vec![];
    let mut data: Vec<u8> = vec![0;1];
    let instr_len = instructions.len();
    // println!("1");    
    while instr_ptr < instr_len {
    // loop {
        // println!("{}", instructions[instr_ptr]);
        match instructions[instr_ptr] {
            '>' => {
                tape_forward(&mut data, &mut dptr);
                instr_ptr +=1;
            }
            '<' => {
                tape_back(&mut data, &mut dptr);
                instr_ptr +=1;   
            }
            '+' => {
                tape_val_inc(&mut data, dptr);
                instr_ptr +=1;
            }
            '-' => {
                tape_val_dec(&mut data, dptr);
                instr_ptr +=1;
            }
            '.' => {
                output.push(data[dptr]);
                instr_ptr +=1;
            }
            ',' => {
                tape_write(&mut data, dptr, &input, &mut input_ptr);
                instr_ptr +=1;
            }
            '[' => {
                tape_seek_forward(&instructions, data[dptr], &mut instr_ptr);
            }
            ']' => {
                tape_seek_back(&instructions, data[dptr], &mut instr_ptr);
            }
            _ => { panic!("BAD INSTRUCTION!"); }
        }
    }

    println!("\n\n{:?}\n\n", data);
    output
}
