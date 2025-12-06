pub fn solve(input: &str) -> (u64, u64) {
  let mut lines = input.trim().lines().rev();
  let ops = lines.next().unwrap().split_ascii_whitespace().collect::<Vec<_>>();
  let p1 = lines
    .map(|line| {
      line
        .split_ascii_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
    })
    .reduce(|aa, bb| {
      aa.into_iter()
        .zip(bb.into_iter())
        .enumerate()
        .map(|(idx, (a, b))| {
          //
          match ops[idx] {
            "*" => a * b,
            "+" => a + b,
            _ => unimplemented!(),
          }
        })
        .collect::<Vec<_>>()
    })
    .unwrap()
    .into_iter()
    .sum();

  (p1, 0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

    assert_eq!((4277556, 0), solve(input));
  }
}
