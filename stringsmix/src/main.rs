fn main() {
    println!("{}", mix("aaaaaa;lk;lk;lwewew", "trreaasdasdaw"));
}


fn mix(s1: &str, s2: &str) -> String {
  
  let mut v1: Vec<Vec<char>> = vec![vec![]];
  let mut v2: Vec<Vec<char>> = vec![vec![]];

  for c in s1.chars() {
    if c.is_lowercase() {
        match v1.iter().position(|l| l.contains(&c)) {
            Some(x) => { v1[x].push(c) }
            None => { v1.push(vec![c]); }
        }
    }   
  }

  for c in s2.chars() {
    if c.is_lowercase() {
        match v2.iter().position(|l| l.contains(&c)) {
            Some(x) => { v2[x].push(c) }
            None => { v2.push(vec![c]); }
        }
    }   
  }

  v1.sort_by_key(|e| e.len());
  v2.sort_by_key(|e| e.len());

  let mut ret: Vec<String> = vec![];
  
  while v1.len() > 0 && v2.len() > 0 {

    match v1.pop() {
        Some(v) => {
            match v2.iter().position(|x| x.contains(&v[0])) {
                Some(i) => {
                    if v.len() > 0 {                    
                        if v.len() > v2[i].len() {
                            ret.push("1:".to_string() + &std::iter::repeat(v[0].to_string()).take(v.len()).collect::<String>());
                        } else if v.len() == v2[i].len() { 
                            ret.push("=:".to_string() + &std::iter::repeat(v[0].to_string()).take(v.len()).collect::<String>());
                        } else {
                            ret.push("2:".to_string() + &std::iter::repeat(v[0].to_string()).take(v2[i].len()).collect::<String>());
                        }
                        v2.remove(i);
                    }
                }
                None => {
                    ret.push("1:".to_string() + &std::iter::repeat(v[0].to_string()).take(v.len()).collect::<String>());            
                }
            }
        }
        None => { }
    }

    match v2.pop() {
        Some(v) => {
            if v.len() > 0 {
                match v1.iter().position(|x| x.contains(&v[0])) {
                    Some(i) => {
                        if v.len() > v1[i].len() {
                            ret.push("1:".to_string() + &std::iter::repeat(v[0].to_string()).take(v.len()).collect::<String>());
                        } else if v.len() == v1[i].len() { 
                            ret.push("=:".to_string() + &std::iter::repeat(v[0].to_string()).take(v.len()).collect::<String>());
                        } else {
                            ret.push("2:".to_string() + &std::iter::repeat(v[0].to_string()).take(v1[i].len()).collect::<String>());
                        }
                        v1.remove(i);
                    }
                    None => {
                        ret.push("1:".to_string() + &std::iter::repeat(v[0].to_string()).take(v.len()).collect::<String>());            
                    }
                }
            }
        }
        None => { }
    }

  }

    ret.sort_by_key(|s| s.len());

    let mut retstr = String::new();
    while retstr.len() > 0 {
        if retstr.len() > 0 {
          retstr += "/"
        }

        retstr += &ret.pop().unwrap();
    }

    retstr
}

fn testing(s1: &str, s2: &str, exp: &str) -> () {
    assert_eq!(&mix(s1, s2), exp)
}

#[test]
fn basics_mix() {

    testing("Are they here", "yes, they are here", 
        "2:eeeee/2:yy/=:hh/=:rr");
    testing("looping is fun but dangerous", "less dangerous than coding", 
        "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg");
    testing(" In many languages", " there's a pair of functions", 
        "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt");
    testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
    testing("codewars", "codewars", "");
    testing("A generation must confront the looming ", "codewarrs", 
        "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr");
    
}