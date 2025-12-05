pub fn solve(input: &str) -> (usize, u64) {
  let (ranges, ids) = input.trim().split_once("\n\n").unwrap();
  let mut ranges = ranges
    .lines()
    .map(|r| {
      let (start, end) = r.split_once('-').unwrap();
      (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    })
    .collect::<Vec<_>>();

  let p1 = ids
    .lines()
    .map(|i| i.parse::<u64>().unwrap())
    .filter(|i| ranges.iter().any(|r| *i >= r.0 && *i <= r.1))
    .count();

  ranges.sort_by_key(|r| r.0);
  let mut p2 = 0;
  let last_range = ranges
    .into_iter()
    .reduce(|last_range, r| {
      if r.0 > last_range.1 {
        // non-overlapping ranges, count the last and track new
        p2 += last_range.1 - last_range.0 + 1;
        r
      } else {
        (last_range.0, r.1.max(last_range.1))
      }
    })
    .unwrap();

  p2 += last_range.1 - last_range.0 + 1;

  (p1, p2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    assert_eq!((3, 14), solve(input));
  }
}
