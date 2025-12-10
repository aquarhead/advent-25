use itertools::Itertools;
use rayon::prelude::*;

pub fn solve(input: &str) -> (usize, usize) {
  (
    input.trim().lines().map(p1).sum(),
    input.trim().par_lines().map(p2).sum(),
  )
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
  let jolts: Vec<usize> = jolts[1..jolts.len() - 1]
    .split(',')
    .map(|j| j.parse().unwrap())
    .collect();

  // let max_press = jolts.iter().max().unwrap().clone();

  let buttons: Vec<_> = buttons
    .split(' ')
    .map(|btn| {
      btn[1..btn.len() - 1]
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
    })
    .collect();

  search(Vec::new(), None, &buttons, &jolts).unwrap()
}

fn search(
  pressed: Vec<usize>,
  mut best: Option<usize>,
  buttons: &Vec<Vec<usize>>,
  jolts: &Vec<usize>,
) -> Option<usize> {
  let mut totals = vec![0; jolts.len()];
  for (p, btn) in pressed.iter().zip(buttons.iter()) {
    for idx in btn {
      totals[*idx] += p;
    }
  }
  let mut diff = totals.iter().zip_eq(jolts.iter()).map(|(p, j)| j - p);

  if pressed.len() == buttons.len() {
    if diff.all(|d| d == 0) {
      Some(pressed.iter().sum())
    } else {
      None
    }
  } else {
    let press_idx = pressed.len();
    let diff = diff.collect::<Vec<_>>();
    let mut max_press = buttons[press_idx].iter().map(|idx| diff[*idx]).min().unwrap();
    if let Some(b) = best {
      max_press = max_press.min(b - pressed.iter().sum::<usize>());
    }

    for p in 0..=max_press {
      let mut pp = pressed.clone();
      pp.push(p);
      if let Some(b) = search(pp, best, buttons, jolts) {
        best = match best {
          None => Some(b),
          Some(eb) => Some(eb.min(b)),
        }
      }
    }

    best
  }
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

    assert_eq!((7, 33), solve(input));
  }
}
