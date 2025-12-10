use itertools::Itertools;

pub fn solve(input: &str) -> (usize, usize) {
  (input.trim().lines().map(p1).sum(), input.trim().lines().map(p2).sum())
}

fn p1(machine: &str) -> usize {
  let (diag, rest) = machine.split_once(' ').unwrap();
  let (buttons, _) = rest.rsplit_once(' ').unwrap();
  let diag: Vec<bool> = diag[1..diag.len() - 1]
    .chars()
    .map(|ch| if ch == '#' { true } else { false })
    .collect();

  buttons
    .split(' ')
    .map(|btn| {
      btn[1..btn.len() - 1]
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
    })
    .powerset()
    .find(|buttons| {
      let idx_pressed = buttons.into_iter().flatten().counts();

      diag.iter().enumerate().all(|(idx, is_on)| {
        if *is_on {
          idx_pressed.get(&idx).map_or(false, |n| n % 2 == 1)
        } else {
          idx_pressed.get(&idx).map_or(true, |n| n % 2 == 0)
        }
      })
    })
    .unwrap()
    .len()
}

fn p2(machine: &str) -> usize {
  let (_, rest) = machine.split_once(' ').unwrap();
  let (buttons, jolts) = rest.rsplit_once(' ').unwrap();
  let jolts: Vec<u32> = jolts[1..jolts.len() - 1]
    .split(',')
    .map(|j| j.parse().unwrap())
    .collect();
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    assert_eq!((7, 0), solve(input));
  }
}
