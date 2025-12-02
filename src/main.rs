#![allow(dead_code)]

mod day01;
mod day02;

fn main() {
  let content = std::fs::read_to_string("inputs/2.txt").expect("read file");
  let (p1, p2) = day02::solve(&content);

  println!("part1: {}", p1);
  println!("part2: {}", p2);
}
