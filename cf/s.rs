use std::io;
use std::str::FromStr;

fn solve() {
}

fn main() {
  for _ in 0..read1() {
    solve();
  }
}

fn read_vec<T: FromStr>() -> Vec<T> {
  read_line().split_whitespace().map(|x| x.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  input
}

fn read1<T: FromStr>() -> T {
  read_line().trim().parse().ok().unwrap()
}

fn read2<T1: FromStr, T2: FromStr>() -> (T1, T2) {
  let input = read_line();
  let mut i = input.split_whitespace();
  let a: T1 = i.next().unwrap().parse().ok().unwrap();
  let b: T2 = i.next().unwrap().parse().ok().unwrap();
  (a, b)
}
