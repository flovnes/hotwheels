mod wheel;
use wheel::Wheel;

fn main() {
  let mut wheel = Wheel::new();
  wheel.add_entry("Apple", 3);
  wheel.add_entry("Banana", 1);
  wheel.add_entry("Me", 12);

  for _ in 0..20 {
    match wheel.spin() {
      Some(entry) => println!("Your mom loves: {}", entry),
      None => panic!("you should help yourself."),
    }
  }
}