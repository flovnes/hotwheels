use rand::prelude::*;

pub struct Wheel<'a> {
  entries: Vec<(&'a str, u32)>,
  total_weight: u32,
}

impl<'a> Wheel<'a> {
  pub fn new() -> Self {
    Wheel {
      entries: Vec::new(),
      total_weight: 0,
    }
  }

  pub fn add_entry(&mut self, entry: &'a str, weight: u32) {
    self.entries.push((entry, weight));
    self.total_weight += weight;
  }

  pub fn spin(&self) -> Option<&'a str> {
    if self.entries.is_empty() {
      return None;
    }

    let mut rng = thread_rng();
    let mut key = rng.gen_range(0..self.total_weight);

    for (entry, weight) in &self.entries {
      if key < *weight {
        return Some(entry);
      }
      key -= weight;
    }

    None
  }
}
