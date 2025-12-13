type Pos = (i32, i32);
type Gift = Vec<Pos>;
type Gifts = Vec<Gift>;

pub fn solve(input: &str) -> (usize, usize) {
  let (_, trees) = input.trim().rsplit_once("\n\n").unwrap();

  let p1 = trees
    .lines()
    .filter(|line| {
      let (size, count) = line.split_once(": ").unwrap();
      let (a, b) = {
        let (a, b) = size.split_once('x').unwrap();
        (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
      };

      let count: u32 = count.split(' ').map(|x| x.parse::<u32>().unwrap()).sum();

      (a / 3) * (b / 3) >= count
    })
    .count();

  (p1, 0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    assert_eq!((2, 0), solve(input));
  }
}
