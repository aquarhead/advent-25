pub fn solve(input: &str) -> (u32, u32) {
  let p1 = input.trim().lines().map(find_largest_battery).sum();

  (p1, 0)
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

    assert_eq!((357, 0), solve(input));
  }
}
