use std::collections::HashMap;

pub fn solve(input: &str) -> (u32, u64) {
  let (beams, p1) = input
    .trim()
    .lines()
    .fold((HashMap::new(), 0), |(mut beams, mut p1), line| {
      for (idx, ch) in line.char_indices() {
        match ch {
          'S' => {
            beams.insert(idx, 1u64);
          }
          '^' => {
            if let Some(n) = beams.remove(&idx) {
              p1 += 1;
              beams.entry(idx - 1).and_modify(|a| *a += n).or_insert(n);
              beams.entry(idx + 1).and_modify(|a| *a += n).or_insert(n);
            }
          }
          '.' => {}
          _ => unimplemented!(),
        }
      }

      (beams, p1)
    });

  (p1, beams.into_iter().map(|x| x.1).sum())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    assert_eq!((21, 40), solve(input));
  }
}
