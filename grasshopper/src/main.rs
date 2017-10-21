fn main() {
    println!("Hello, world!");
}

#[test]
fn example_tests() {
    assert_eq!(combat(100.0, 5.0), 95.0);
    assert_eq!(combat(92.0, 8.0), 84.0);
    assert_eq!(combat(20.0, 30.0), 0.0, "Health cannot go below 0");
}

fn combat(health: f32, damage: f32) -> f32 {
    let new_health = health - damage;
    if new_health > 0.0 { return new_health }
    0.0
}