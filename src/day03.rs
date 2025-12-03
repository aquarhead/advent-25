pub fn solve(input: &str) -> (u64, u64) {
  let p1 = input.trim().lines().map(|b| find_largest_bank(b, 2)).sum();
  let p2 = input.trim().lines().map(|b| find_largest_bank(b, 12)).sum();

  (p1, p2)
}

fn find_largest_bank(batteries: &str, num: usize) -> u64 {
  let bs = batteries
    .chars()
    .map(|c| c.to_digit(10).unwrap() as u64)
    .collect::<Vec<_>>();
  let mut start_idx = 0;
  let mut bank = 0;
  for n in (1..=num).rev() {
    let end_idx = bs.len() - n + 1;
    let pick_idx = (start_idx..end_idx).rev().into_iter().max_by_key(|i| &bs[*i]).unwrap();
    bank *= 10;
    bank += &bs[pick_idx];
    start_idx = pick_idx + 1;
  }

  bank
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
