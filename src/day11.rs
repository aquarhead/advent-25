use pathfinding::directed::count_paths::count_paths;
use std::collections::HashMap;

type Connections = HashMap<String, Vec<String>>;

pub fn solve(input: &str) -> (usize, usize) {
  let conns = parse(input);
  (p1(&conns), p2(&conns))
}

fn parse(input: &str) -> Connections {
  input
    .trim()
    .lines()
    .map(|line| {
      let (origin, dests) = line.split_once(": ").unwrap();

      (
        origin.to_string(),
        dests.split(' ').map(|s| s.to_string()).collect::<Vec<_>>(),
      )
    })
    .collect()
}

fn p1(conns: &Connections) -> usize {
  let cp = |from: &str, to: &str| count_paths(from.to_string(), |o| conns.get(o).unwrap().clone(), |n| n == to);

  cp("you", "out")
}

fn p2(conns: &Connections) -> usize {
  let cp = |from: &str, to: &str| {
    count_paths(
      from.to_string(),
      |o| conns.get(o).cloned().unwrap_or_default(),
      |n| n == to,
    )
  };

  (cp("svr", "dac") * cp("dac", "fft") * cp("fft", "out")) + (cp("svr", "fft") * cp("fft", "dac") * cp("dac", "out"))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_p1() {
    let input = "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    assert_eq!(5, p1(&parse(input)));
  }

  #[test]
  fn test_p2() {
    let input = "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
    assert_eq!(2, p2(&parse(input)));
  }
}
