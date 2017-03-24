use std::collections::HashSet;

type Atom = (String, usize);
type Molecule = Vec<Atom>;

#[derive(Debug)]
pub struct ParseError {}

static ELEMS: &'static [&str] =
&[ "Ac", "Al", "Am", "Sb", "Ar", "As", "At", "Ba", "Bk", "Be", "Bi", "Bh", "B", "Br",
"Cd", "Ca", "Cf", "C", "Ce", "Cs", "Cl", "Cr", "Co", "Cn", "Cu", "Cm", "Ds", "Db",
"Dy", "Es", "Er", "Eu", "Fm", "Fl", "F", "Fr", "Gd", "Ga", "Ge", "Au", "Hf", "Hs",
"He", "Ho", "H", "In", "I", "Ir", "Fe", "Kr", "La", "Lr", "Pb", "Li", "Lv", "Lu",
"Mg", "Mn", "Mt", "Md", "Hg", "Mo", "Nd", "Ne", "Np", "Ni", "Nb", "N", "No", "Os",
"O", "Pd", "P", "Pt", "Pu", "Po", "K", "Pr", "Pm", "Pa", "Ra", "Rn", "Re", "Rh",
"Rg", "Rb", "Ru", "Rf", "Sm", "Sc", "Sg", "Se", "Si", "Ag", "Na", "Sr", "S", "Ta",
"Tc", "Te", "Tb", "Tl", "Th", "Tm", "Sn", "Ti", "W", "Uuo", "Uup", "Uus", "Uut",
"U", "V", "Xe", "Yb", "Y", "Zn", "Zr",];


pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let elems: HashSet<&str> = ELEMS.iter().cloned().collect();

  Err(ParseError{})
}

// mod molecules {
//   assert_parse!("H", [("H",1)], hydrogen);
//   assert_parse!("O2", [("O",2)], oxygen);
//   assert_parse!("H2O", [("H",2),("O",1)], water);
//   assert_parse!("Mg(OH)2", [("Mg",1),("O",2),("H",2)], magnesium_hydroxide);
//   assert_parse!("K4[ON(SO3)2]2", [("K",4),("O",14),("N",2),("S",4)], fremys_salt);
// }

// #[test]
// fn errors() {
//   assert_fail("pie", "Not a valid molecule");
//   assert_fail("Mg(OH", "Mismatched parenthesis");
//   assert_fail("Mg(OH}2", "Mismatched parenthesis");
// }

fn main() {
    println!("Hello, world!");
}
