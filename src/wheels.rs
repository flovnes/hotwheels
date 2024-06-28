pub fn get_wheels() -> Vec<Vec<(&'static str, u8)>> {
  let wheel_1 = vec![("i love your mom", 1), ("i love your mom", 3), ("i love your mom", 1), ("i love your mom", 4)];
  let wheel_2 = vec![("i love your mom", 1), ("i love your mom", 3), ("i love your mom", 1), ("i love your mom", 4)];
  let wheel_3 = vec![("your mom loves me", 1), ("your mom loves me", 3), ("your mom loves me", 1), ("your mom loves me", 4)];
  let wheel_4 = vec![("your mom loves me", 1), ("your mom loves me", 3), ("your mom loves me", 1), ("your mom loves me", 4)];

  vec![wheel_1, wheel_2, wheel_3, wheel_4]
}