pub fn solve(input: &str) -> (u64, u64) {
  let (p1, _) = input.trim().lines().fold((0u64, 50i32), |(mut p1, pos), s| {
    let (dir, num) = s.split_at(1);
    let mut new_pos = match dir {
      "L" => pos - num.parse::<i32>().unwrap(),
      "R" => pos + num.parse::<i32>().unwrap(),
      _ => unimplemented!(),
    };
    new_pos %= 100;

    if new_pos == 0 {
      p1 += 1;
    }

    (p1, new_pos)
  });

  let (p2, _) = input.trim().lines().fold((0u64, 50i32), |(mut p2, pos), s| {
    let (dir, num) = s.split_at(1);
    let new_pos = match dir {
      "L" => {
        let mut p = pos - num.parse::<i32>().unwrap();
        while p < 0 {
          p += 100;
          p2 += 1;
        }
        // treat starting from 0 specially
        if pos == 0 && p != 0 {
          p2 -= 1;
        }
        if p == 0 {
          p2 += 1;
        }
        p
      }
      "R" => {
        let mut p = pos + num.parse::<i32>().unwrap();
        while p > 99 {
          p -= 100;
          p2 += 1;
        }
        p
      }
      _ => unimplemented!(),
    };

    (p2, new_pos)
  });
  (p1, p2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    assert_eq!((3, 6), solve(input));
  }

  #[test]
  fn test_example2() {
    let input = "
R1000
";

    assert_eq!((0, 10), solve(input));
  }

  #[test]
  fn test_example3() {
    let input = "
L50
L1
";

    assert_eq!((1, 1), solve(input));
  }
}
