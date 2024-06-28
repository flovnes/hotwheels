use rand::Rng;

pub fn spin_wheel(mut options: &mut Vec<(&'static str, u8)>) -> &'static str {
  eval_sums(&mut options);

  let max_roll = options.last().unwrap().1;
  let roll_key = rand::thread_rng().gen::<u8>() % max_roll;

  return get_value_by_key(&mut options, roll_key);
}

// weights to partial sums
fn eval_sums(options: &mut Vec<(&'static str, u8)>) {
  let mut sum = 0;
  for pair in options {
    sum += pair.1;
    pair.1 = sum;
  }
}

fn get_value_by_key(options: &Vec<(&'static str, u8)>, key: u8) -> &'static str {
  for pair in options {
    if key <= pair.1 { return pair.0 } 
  }
  // i love erorr handing
  "No values found. Wheel is likely empty."
}