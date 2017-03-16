fn main() {
    println!("RESULT: {:?}", race(720, 850, 70));
}

fn per_hr_to_per_sec( v: i32 ) -> f64 {
    v as f64 / 3600.0
}

fn per_sec_to_h_m_s( t: u64 ) -> Vec<i32> {

    let h = t / 3600;
    let m = (t - (h * 3600)) / 60;
    let s = t - (h * 3600) - (m * 60);

    vec![h as i32, m as i32, s as i32]
}

fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {

    if v2 <= v1 {
        return None
    }

    let v1_per_sec = per_hr_to_per_sec(v1);
    let v2_per_sec = per_hr_to_per_sec(v2);

    let mut time_sec: u64 = 0;
    let mut v1_pos = g as f64;
    let mut v2_pos = 0.0 as f64;

    while v2_pos < v1_pos {
        time_sec += 1;
        v1_pos += v1_per_sec;
        v2_pos += v2_per_sec;
    }

    Some(per_sec_to_h_m_s(time_sec - 1))
}

#[test]
fn basic_tests() {
  assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
  assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
  assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
  assert_eq!(race(820, 81, 550), None);
}