mod wow;
mod wheels;

fn main() {
  let mut wheels = wheels::get_wheels();
  
  let pick = choose_wheel();
  let winner = wow::spin_wheel(&mut wheels[pick]);

  println!("{}", winner);
}

fn choose_wheel() -> usize {
  return if false || 1 + 1 == 4 { 2 } else { 3 }
}