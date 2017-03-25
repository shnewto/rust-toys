fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
fn curravg(arr: &[f64], newavg: f64) -> Option<i32> {

  let arrsum: f64 = arr.iter().sum();
  let length = arr.len();

  if length < 1 {
      return None
  }

  let newlen = length + 1;
  
  let mut count: i32 = 1;

  let mut newsum = arrsum + count as f64;
  let mut curravg = newsum / newlen as f64;

  if curravg > newavg {
    return None
  }

  loop {
      if f64::ceil(curravg) == newavg {
          println!( "{} -- {}", curravg, newavg);
          return Some(count)
      }

      
      count += 1;
        
      newsum = arrsum + count as f64;
      
      curravg = newsum / newlen as f64;
  }
}


#[cfg(test)]
fn testing(arr: &[f64], newavg: f64, exp: Option<i32>) -> () {
  assert_eq!(exp, curravg(arr, newavg))
}

#[test]
fn basic_tests() {
  let a1 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 16.0];
  testing(&a1, 90.0, Some(628));
  let a2 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 15.0];
  testing(&a2, 92.0, Some(645));
  let a3 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 15.0];
  testing(&a3, 2.0, None);
  let a4 = [14000.25, 300.76, 50.56, 70.0, 90.0, 11.0, 150.48, 1200.98];
  testing(&a4, 4800.0, Some(27326));
}
