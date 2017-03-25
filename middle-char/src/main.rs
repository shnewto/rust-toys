fn main() {
    println!("{}", get_middle("test"));
    println!("{}", get_middle("testing"));
    println!("{}", get_middle("middle"));
    println!("{}", get_middle("A"));
    println!("{}", get_middle("of"));
}


fn get_middle(s:&str) -> &str {
    let s_len = s.len() + 1;
    let fmid = s_len as f32 / 2.0;
    let mid = s_len/2;

    if fmid == mid as f32 {
        &s[mid-1..mid]
    } else {
        &s[mid-1..mid+1]
    }
}

#[test]
fn example_tests() {
  assert_eq!(get_middle("test"),"es");
  assert_eq!(get_middle("testing"),"t");
  assert_eq!(get_middle("middle"),"dd");
  assert_eq!(get_middle("A"),"A");
  assert_eq!(get_middle("of"),"of");
}