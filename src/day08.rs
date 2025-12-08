use itertools::Itertools;
use std::collections::HashMap;

type Coord = (i64, i64, i64);

pub fn solve(input: &str) -> (usize, i64) {
  solve_with_n(input, 1000)
}

fn solve_with_n(input: &str, num: usize) -> (usize, i64) {
  let boxes: Vec<Coord> = input
    .trim()
    .lines()
    .map(|line| {
      let mut parts = line.split(',');
      (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
      )
    })
    .collect();

  let neighbours: HashMap<Coord, Vec<Coord>> = boxes
    .iter()
    .cloned()
    .tuple_combinations::<(_, _)>()
    .k_smallest_by_key(num, |(a, b)| distance(*a, *b))
    .fold(HashMap::new(), |mut acc, (a, b)| {
      acc.entry(a).or_default().push(b);
      acc.entry(b).or_default().push(a);
      acc
    });

  let p1 = pathfinding::undirected::connected_components::connected_components(&boxes, |b| {
    neighbours.get(b).cloned().unwrap_or_default()
  })
  .into_iter()
  .map(|circuit| circuit.len())
  .k_largest(3)
  .product();

  let edges: Vec<_> = boxes
    .iter()
    .cloned()
    .tuple_combinations::<(_, _)>()
    .map(|(a, b)| (a, b, distance(a, b)))
    .collect();

  let (a, b, _) = pathfinding::undirected::kruskal::kruskal(&edges)
    .max_by_key(|(_, _, d)| *d)
    .unwrap();

  (p1, a.0 * b.0)
}

fn distance(a: Coord, b: Coord) -> i64 {
  (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    assert_eq!((40, 25272), solve_with_n(input, 10));
  }
}
