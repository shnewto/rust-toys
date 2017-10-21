fn main() {
    assert_eq!(hq9("H"), Some("Hello World!".to_string()));
    assert_eq!(hq9("Q"), Some("Q".to_string()));
    assert_eq!(hq9("ZZZZ"), None);
    // println!("{}", hq9("9").unwrap());
}


fn hq9(code: &str) -> Option<String> {

    match code.as_ref() {
        "H" => return Some("Hello World!".to_string()),
        "Q" => return Some(code.to_string()),
        "9" => return Some(nn_bottles()),
        _ => return None,
    }

}

fn nn_bottles() -> String {
    let mut lyrics: String = String::new();

    lyrics += format!(
        "{ammt} bottles of beer on the wall, {ammt} bottles of beer.\n",
        ammt=99).as_str();

    for b in (2..99).rev() {
        lyrics += format!(
        "Take one down and pass it around, {ammt} bottles of beer on the wall.\n\
        {ammt} bottles of beer on the wall, {ammt} bottles of beer.\n",
        ammt=b).as_str();
    }

    lyrics += format!(
    "Take one down and pass it around, {ammt} bottle of beer on the wall.\n\
    {ammt} bottle of beer on the wall, {ammt} bottle of beer.\n\
    Take one down and pass it around, {less} bottles of beer on the wall",
    ammt=1, less="no more").as_str();

    lyrics
}