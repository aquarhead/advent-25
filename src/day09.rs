use itertools::Itertools;

type Pos = (i64, i64);

pub fn solve(input: &str) -> (i64, i64) {
  let poses = input.trim().lines().map(|line| {
    let (x, y) = line.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap()) as Pos
  });

  let p1 = poses
    .clone()
    .tuple_combinations::<(_, _)>()
    .map(|(a, b)| ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1))
    .max()
    .unwrap();

  // https://aoc.oppi.li/2.5-day-9.html#day-9

  let edges: Vec<_> = poses
    .clone()
    .collect::<Vec<_>>()
    .into_iter()
    .circular_tuple_windows::<(_, _)>()
    .collect();

  let in_bound = |a: Pos, b: Pos| -> bool {
    edges.iter().all(|(aa, bb)| {
      aa.0.min(bb.0) >= a.0.max(b.0)
        || aa.0.max(bb.0) <= a.0.min(b.0)
        || aa.1.min(bb.1) >= a.1.max(b.1)
        || aa.1.max(bb.1) <= a.1.min(b.1)
    })
  };

  let p2 = poses
    .clone()
    .tuple_combinations::<(_, _)>()
    .flat_map(|(a, b)| (in_bound(a, b)).then(|| ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1)))
    .max()
    .unwrap();

  (p1, p2)
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

    assert_eq!((50, 24), solve(input));
  }
}
