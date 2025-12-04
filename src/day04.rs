use std::collections::HashSet;

type Pos = (i32, i32);

pub fn solve(input: &str) -> (usize, usize) {
  let mut papers = HashSet::new();
  input.trim().lines().enumerate().for_each(|(row, line)| {
    line.trim().chars().enumerate().for_each(|(col, ch)| {
      let pos: Pos = (row as i32, col as i32);
      if ch == '@' {
        papers.insert(pos);
      }
    });
  });

  let p1 = papers.iter().filter(|p| can_access(**p, &papers)).count();

  let mut p2 = 0;
  loop {
    let remove = papers
      .iter()
      .filter(|p| can_access(**p, &papers))
      .cloned()
      .collect::<Vec<_>>();
    if remove.len() == 0 {
      break;
    }

    p2 += remove.len();
    remove.into_iter().for_each(|p| {
      papers.remove(&p);
    });
  }

  (p1, p2)
}

fn can_access(pos: Pos, papers: &HashSet<Pos>) -> bool {
  let mut surround = 0;
  for dx in [-1, 0, 1] {
    for dy in [-1, 0, 1] {
      if dx != 0 || dy != 0 {
        let check_pos = (pos.0 + dx, pos.1 + dy);
        if papers.contains(&check_pos) {
          surround += 1;
        }
      }
    }
  }

  surround < 4
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    assert_eq!((13, 43), solve(input));
  }
}
