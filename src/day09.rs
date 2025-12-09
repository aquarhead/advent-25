use itertools::Itertools;

type Pos = (i64, i64);

pub fn solve(input: &str) -> (i64, u64) {
  let p1 = input
    .trim()
    .lines()
    .map(|line| {
      let (x, y) = line.split_once(',').unwrap();
      (x.parse().unwrap(), y.parse().unwrap()) as Pos
    })
    .tuple_combinations::<(_, _)>()
    .map(|(a, b)| ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1))
    .max()
    .unwrap();

  (p1, 0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    assert_eq!((50, 0), solve(input));
  }
}
