pub fn solve(input: &str) -> (usize, usize) {
  let (ranges, ids) = input.trim().split_once("\n\n").unwrap();
  let ranges = ranges
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

  (p1, 0)
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

    assert_eq!((3, 0), solve(input));
  }
}
