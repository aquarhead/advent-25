use itertools::Itertools;

pub fn solve(input: &str) -> (u64, u64) {
  (p1(input), p2(input))
}

fn p1(input: &str) -> u64 {
  let mut lines = input.trim().lines().rev();
  let ops = lines.next().unwrap().split_ascii_whitespace().collect::<Vec<_>>();
  lines
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
        .map(|(idx, (a, b))| match ops[idx] {
          "*" => a * b,
          "+" => a + b,
          _ => unimplemented!(),
        })
        .collect::<Vec<_>>()
    })
    .unwrap()
    .into_iter()
    .sum()
}

fn p2(input: &str) -> u64 {
  let mut lines = input.trim_end().lines().rev();

  let (split_indices, ops): (Vec<_>, Vec<_>) = lines
    .next()
    .unwrap()
    .chars()
    .enumerate()
    .filter(|(_, ch)| *ch != ' ')
    .unzip();

  lines
    .fold(vec![vec![]; ops.len()], |mut acc, line| {
      for (idx, (s, e)) in split_indices.iter().tuple_windows().enumerate() {
        acc[idx].push(line.get(*s..(*e - 1)).unwrap().to_string());
      }

      acc
        .last_mut()
        .unwrap()
        .push(line.get(*split_indices.last().unwrap()..).unwrap().to_string());

      acc
    })
    .into_iter()
    .enumerate()
    .map(|(idx, nums)| {
      let init = vec![0u64; nums.first().unwrap().len()];
      let vnums = nums.into_iter().rev().fold(init, |mut acc, num_str| {
        acc.iter_mut().zip(num_str.chars()).for_each(|(n, ch)| {
          if ch != ' ' {
            *n *= 10;
            *n += ch.to_digit(10).unwrap() as u64
          }
        });

        acc
      });

      if ops[idx] == '*' {
        vnums.into_iter().product::<u64>()
      } else {
        vnums.into_iter().sum()
      }
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    assert_eq!((4277556, 3263827), solve(input));
  }
}
