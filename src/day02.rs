use std::collections::HashSet;
use std::iter::repeat;

pub fn solve(input: &str) -> (u64, u64) {
  let p1 = input
    .trim()
    .split(',')
    .flat_map(|r| find_repeats(r, 2))
    .collect::<HashSet<_>>()
    .iter()
    .sum();

  let p2 = input
    .trim()
    .split(',')
    .flat_map(find_invalid_ids)
    .collect::<HashSet<_>>()
    .iter()
    .sum();

  (p1, p2)
}

fn find_invalid_ids(range: &str) -> impl Iterator<Item = u64> {
  let mut parts = range.split('-');
  let start_len = parts.next().unwrap().len();
  let end_len = parts.next().unwrap().len();

  let max_len = start_len.max(end_len);
  (2..=max_len).flat_map(|n| find_repeats(range, n))
}

fn find_repeats(range: &str, split: usize) -> Vec<u64> {
  let mut ret = Vec::new();
  let mut parts = range.split('-');
  let start_str = parts.next().unwrap();
  let start = start_str.parse().unwrap();
  let end_str = parts.next().unwrap();
  let end = end_str.parse::<u64>().unwrap();

  let mut check: u64 = if start_str.len() % split == 0 {
    let (first, _) = start_str.split_at(start_str.len() / split);
    first.parse().unwrap()
  } else {
    10u64.pow((start_str.len() / split) as u32)
  };

  loop {
    let num = repeated(check, split);
    check += 1;
    if num < start {
      continue;
    }
    if num <= end {
      ret.push(num);
    } else {
      break;
    }
  }

  let mut check: u64 = if end_str.len() % split == 0 {
    let (first, _) = end_str.split_at(end_str.len() / split);
    first.parse().unwrap()
  } else {
    10u64.pow((end_str.len() / split) as u32)
  };

  loop {
    if check < 1 {
      break;
    }
    let num = repeated(check, split);
    check -= 1;
    if num > end {
      continue;
    }
    if num >= start {
      ret.push(num);
    } else {
      break;
    }
  }

  ret
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
