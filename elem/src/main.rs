// Description:

// For a given chemical formula represented by a string, count the number of atoms of each element contained in the molecule and return an object.

// For example:

// parse_molecule("H2O");           // water
// // Ok([("H", 2), ("O", 1)])

// parse_molecule("Mg(OH)2");       // magnesium hydroxide
// // Ok([("Mg", 1), ("O", 2), ("H", 2)]

// parse_molecule("K4[ON(SO3)2]2"); // Fremy's salt
// // Ok([("K", 4), ("O", 14),("N", 2),("S", 4)])

// parse_molecule("pie")
// // Err(ParseError)
// As you can see, some formulas have brackets in them. The index outside the brackets tells you that you have to multiply count of each atom inside the bracket on this index. For example, in Fe(NO3)2 you have one iron atom, two nitrogen atoms and six oxygen atoms.

// Note that brackets may be round, square or curly and can also be nested. Index after the braces is optional.

use std::collections::HashSet;

type Atom = (String, usize);
type Molecule = Vec<Atom>;
type ParseError = &'static str;


static ELEMS: &'static [&'static str] =
&[ "Ac", "Al", "Am", "Sb", "Ar", "As", "At", "Ba", "Bk", "Be", "Bi", "Bh", "B", "Br",
"Cd", "Ca", "Cf", "C", "Ce", "Cs", "Cl", "Cr", "Co", "Cn", "Cu", "Cm", "Ds", "Db",
"Dy", "Es", "Er", "Eu", "Fm", "Fl", "F", "Fr", "Gd", "Ga", "Ge", "Au", "Hf", "Hs",
"He", "Ho", "H", "In", "I", "Ir", "Fe", "Kr", "La", "Lr", "Pb", "Li", "Lv", "Lu",
"Mg", "Mn", "Mt", "Md", "Hg", "Mo", "Nd", "Ne", "Np", "Ni", "Nb", "N", "No", "Os",
"O", "Pd", "P", "Pt", "Pu", "Po", "K", "Pr", "Pm", "Pa", "Ra", "Rn", "Re", "Rh",
"Rg", "Rb", "Ru", "Rf", "Sm", "Sc", "Sg", "Se", "Si", "Ag", "Na", "Sr", "S", "Ta",
"Tc", "Te", "Tb", "Tl", "Th", "Tm", "Sn", "Ti", "W", "Uuo", "Uup", "Uus", "Uut",
"U", "V", "Xe", "Yb", "Y", "Zn", "Zr",];


pub fn seek_num(char_vec: &Vec<char>, mol_ptr: &mut usize) -> usize {
    let mut n = String::new();

    while *mol_ptr < char_vec.len() && char_vec[*mol_ptr].is_numeric() {
        n += &char_vec[*mol_ptr].to_string();
        *mol_ptr += 1;
    }

    if n.len() > 0 {
        n.parse::<usize>().unwrap()
    } else {
        1
    }
}

pub fn seek_name(char_vec: &Vec<char>, mol_ptr: &mut usize) -> Result<Atom, ParseError> {
    let mut s: String = char_vec[*mol_ptr].to_string();

    *mol_ptr += 1;

    while *mol_ptr < char_vec.len() && char_vec[*mol_ptr].is_lowercase() {
        s += &char_vec[*mol_ptr].to_string();
        *mol_ptr += 1;
    }

    let n = seek_num(char_vec, mol_ptr);

    Ok((s,n))
}

pub fn seek_bracket(char_vec: &Vec<char>, mol_ptr: &mut usize) -> Result<Atom, ParseError> {
    let mut s = String::new();
    let mut found = false;
    let mut mine = 0;

    *mol_ptr += 1;    

    while found == false {
        if *mol_ptr >= char_vec.len() {
            return Err("Mismatched parenthesis")
        }        
        match char_vec[*mol_ptr] {
            '[' => {
                mine += 1;
                s += &char_vec[*mol_ptr].to_string();
            }
            ']' => {
                if mine == 0 {
                    found = true;
                } else {
                    mine -= 1;
                    s += &char_vec[*mol_ptr].to_string();
                }
            }                            
            _ => {
                s += &char_vec[*mol_ptr].to_string();
            }
        }

        *mol_ptr += 1;
    }

    let n = seek_num(char_vec, mol_ptr);

    Ok((s,n)) 
}

pub fn seek_paren(char_vec: &Vec<char>, mol_ptr: &mut usize) -> Result<Atom, ParseError> {
    let mut s = String::new();   
    let mut found = false;
    let mut mine = 0;

    *mol_ptr += 1;    

    while found == false {
        if *mol_ptr >= char_vec.len() {
            return Err("Mismatched parenthesis")
        }        
        match char_vec[*mol_ptr] {
            '(' => {
                mine += 1;
                s += &char_vec[*mol_ptr].to_string();
            }
            ')' => {
                if mine == 0 {
                    found = true;
                } else {
                    mine -= 1;
                    s += &char_vec[*mol_ptr].to_string();
                }
            }                            
            _ => {
                s += &char_vec[*mol_ptr].to_string();
            }
        }

        *mol_ptr += 1;
    }

    let n = seek_num(char_vec, mol_ptr);

    Ok((s,n))
}

pub fn seek_brace(char_vec: &Vec<char>, mol_ptr: &mut usize) -> Result<Atom, ParseError> {
    let mut s = String::new();
    let mut found = false;
    let mut mine = 0;

    *mol_ptr += 1;    

    while found == false {
        if *mol_ptr >= char_vec.len() {
            return Err("Mismatched parenthesis")
        }
        match char_vec[*mol_ptr] {
            '{' => {
                mine += 1;
                s += &char_vec[*mol_ptr].to_string();
            }
            '}' => {
                if mine == 0 {
                    found = true;
                } else {
                    mine -= 1;
                    s += &char_vec[*mol_ptr].to_string();
                }
            }                            
            _ => {
                s += &char_vec[*mol_ptr].to_string();
            }
        }

        *mol_ptr += 1;
    }

    let n = seek_num(char_vec, mol_ptr);

    Ok((s,n))
}

pub fn reduce(molecule: Molecule) -> Result<Molecule, ParseError> {
    let mut ret: Molecule = Vec::new();
    let mut blacklist: Vec<usize> = Vec::new();

     for i in 0..molecule.len() {
        let mut atom = molecule[i].clone();
        if !blacklist.contains(&i) {
            for j in i+1..molecule.len() {
                if atom.0 == molecule[j].0 {
                    atom.1 += molecule[j].1;
                    blacklist.push(j);
                }
            }
            ret.push(atom);       
        }
    }

    Ok(ret)
}

pub fn parse_molecule(molecule: &str) -> Result<Molecule, ParseError> {
    println!("{:?}", molecule);
    let elems: HashSet<&str> = ELEMS.iter().cloned().collect();
    let mut tokens: Molecule = Vec::new();
    let char_vec: Vec<char> = molecule.chars().collect();
    
    let mut mol_ptr = 0;

    while mol_ptr < char_vec.len() {

        if char_vec[mol_ptr].is_uppercase() {
            let atom = seek_name(&char_vec, &mut mol_ptr)?;

            if elems.contains::<str>(&atom.0) {
                tokens.push(atom);
            } else {
                return Err("Not a valid molecule")
            }

        } else if char_vec[mol_ptr] == '[' {

            let atom = seek_bracket(&char_vec, &mut mol_ptr)?;
            let m = parse_molecule(&atom.0)?;

            for e in m {
                tokens.push((e.0, e.1 * atom.1));
            }
        } else if char_vec[mol_ptr] == '(' {
            let atom = seek_paren(&char_vec, &mut mol_ptr)?;
            let m = parse_molecule(&atom.0)?;

            for e in m {
                tokens.push((e.0, e.1 * atom.1));
            }
        } else if char_vec[mol_ptr] == '{' {
            let atom = seek_brace(&char_vec, &mut mol_ptr)?;
            let m = parse_molecule(&atom.0)?;

            for e in m {
                tokens.push((e.0, e.1 * atom.1));
            }
        } else {
            return Err("Not a valid molecule")
        }
    }
 
    reduce(tokens)
}

// mod molecules {
//   assert_parse!("H", [("H",1)], hydrogen);
//   assert_parse!("O2", [("O",2)], oxygen);
//   assert_parse!("H2O", [("H",2),("O",1)], water);
//   assert_parse!("Mg(OH)2", [("Mg",1),("O",2),("H",2)], magnesium_hydroxide);
//   assert_parse!("K4[ON(SO3)2]2", [("K",4),("O",14),("N",2),("S",4)], fremys_salt);
// }

#[test]
fn errors() {
  assert_eq!(parse_molecule("pie"), Err("Not a valid molecule"));
//   assert_fail("Mg(OH", "Mismatched parenthesis");
//   assert_fail("Mg(OH}2", "Mismatched parenthesis");
}





fn main() {
    println!("{:?}", parse_molecule("UuoK4[ON(SO3)2]2"));
    println!("{:?}", parse_molecule("pie"));
    // parse_molecule("Mg(OH)2");
    // parse_molecule("H2O");
    // parse_molecule("O2");
    // parse_molecule("H");
}
