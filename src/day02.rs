use std::collections::HashSet;
use std::iter::repeat;

pub fn solve(input: &str) -> (u64, u64) {
  let p1 = input.trim().split(',').map(|r| find_repeats(r, 2)).sum();
  let p2 = input.trim().split(',').flat_map(find_invalid_ids).sum();

  (p1, p2)
}

fn find_start(s: &str, split: usize) -> u64 {
  if s.len() < split {
    // start will always be lower than end, so we may try splits more than length of start
    // in those cases we just try from 1
    1
  } else {
    if s.len() % split == 0 {
      let (first, _) = s.split_at(s.len() / split);
      first.parse().unwrap()
    } else {
      // if the start is not divisble by split, the next possible number will be 1_(a few)0 repeated
      10u64.pow((s.len() / split) as u32)
    }
  }
}

fn find_end(s: &str, split: usize) -> u64 {
  if s.len() % split == 0 {
    let (first, _) = s.split_at(s.len() / split);
    first.parse().unwrap()
  } else {
    // for end, the max possible number in an odd split is (a few)9s
    10u64.pow((s.len() / split) as u32) - 1
  }
}

fn find_invalid_ids(range: &str) -> impl Iterator<Item = u64> {
  let mut parts = range.split('-');
  let start_len = parts.next().unwrap().len();
  let end_len = parts.next().unwrap().len();

  let max_len = start_len.max(end_len);
  (2..=max_len).map(|n| find_repeats(range, n))
}

fn find_repeats(range: &str, split: usize) -> u64 {
  let mut ret = HashSet::new();
  let mut parts = range.split('-');
  let start_str = parts.next().unwrap();
  let start = start_str.parse().unwrap();
  let check_start = find_start(start_str, split);
  let end_str = parts.next().unwrap();
  let end = end_str.parse().unwrap();
  let check_end = find_end(end_str, split);

  for n in check_start..=check_end {
    let num = repeated(n, split);
    if num >= start && num <= end {
      ret.insert(num);
    }
  }

  ret.iter().sum()
}

fn repeated(half: u64, num: usize) -> u64 {
  repeat(half.to_string())
    .take(num)
    .collect::<Vec<String>>()
    .join("")
    .parse()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    assert_eq!((1227775554, 4174379265), solve(input));
  }

  #[test]
  fn test_example2() {
    let input = "95-115";

    assert_eq!((99, 210), solve(input));
  }
}
