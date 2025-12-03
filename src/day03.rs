pub fn solve(input: &str) -> (u32, u64) {
  let p1 = input.trim().lines().map(find_largest_battery).sum();
  let p2 = input
    .trim()
    .lines()
    .map(|bank| {
      bank
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .fold(Battery12::empty(), |bat, n| bat.expand_with(n as u8))
        .num()
    })
    .sum();

  (p1, p2)
}

fn find_largest_battery(bank: &str) -> u32 {
  bank
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .fold((None, None, 0), |acc, n| match acc {
      (None, None, bat) => (Some(n), None, bat),
      (Some(h), None, bat) => {
        let new_bat = h * 10 + n;
        if n >= h {
          (Some(n), None, new_bat)
        } else {
          (Some(h), Some(n), bat.max(new_bat))
        }
      }
      (Some(h), Some(l), bat) => {
        // should always be h > l
        let new_bat = h * 10 + n;
        if n >= h {
          (Some(n), None, new_bat)
        } else if n > l {
          (Some(h), Some(n), bat.max(new_bat))
        } else {
          acc
        }
      }
      _ => unimplemented!(),
    })
    .2
}

#[derive(Copy, Clone, Debug)]
struct Battery12([u8; 12]);

impl Battery12 {
  fn empty() -> Self {
    Self([0; 12])
  }

  fn num(&self) -> u64 {
    self.0.into_iter().fold(0u64, |acc, n| acc * 10 + (n as u64))
  }

  fn expand_with(&self, n: u8) -> Self {
    let mut max = self.clone();
    for i in 0..12 {
      if self.0[i] == 0 {
        max.0[i] = n;
        return max;
      }
    }
    for i in 0..12 {
      let mut new_bat = self.0.clone();
      if i > 0 {
        new_bat.copy_within(0..i, 0);
      }
      if i < 11 {
        new_bat.copy_within((i + 1)..12, i);
      }
      new_bat[11] = n;

      let new_bat = Battery12(new_bat);
      if new_bat.num() > max.num() {
        max = new_bat;
      }
    }
    max
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
987654321111111
811111111111119
234234234234278
818181911112111
";

    assert_eq!((357, 3121910778619), solve(input));
  }
}
