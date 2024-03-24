use std::io;
use std::str::FromStr;

fn solve() {
}

fn main() {
  for _ in 0..read1() {
    solve();
  }
}

























#[allow(non_camel_case_types, dead_code)]
type us = usize;

#[allow(dead_code)]
fn read_vec_1<T: FromStr>(first: T) -> Vec<T> {
  let mut v = read_line().split_whitespace().map(|x| x.parse().ok().unwrap()).collect::<Vec<T>>();
  v.insert(0, first);
  v
}

#[allow(dead_code)]
fn read_vec<T: FromStr>() -> Vec<T> {
  read_line().split_whitespace().map(|x| x.parse().ok().unwrap()).collect::<Vec<T>>()
}

#[allow(dead_code)]
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  input
}

#[allow(dead_code)]
fn read1<T: FromStr>() -> T {
  read_line().trim().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read2<T1: FromStr, T2: FromStr>() -> (T1, T2) {
  let input = read_line();
  let mut i = input.split_whitespace();
  let a: T1 = i.next().unwrap().parse().ok().unwrap();
  let b: T2 = i.next().unwrap().parse().ok().unwrap();
  (a, b)
}

#[allow(dead_code)]
fn read3<T1: FromStr, T2: FromStr, T3: FromStr>() -> (T1, T2, T3) {
  let input = read_line();
  let mut i = input.split_whitespace();
  let a: T1 = i.next().unwrap().parse().ok().unwrap();
  let b: T2 = i.next().unwrap().parse().ok().unwrap();
  let c: T3 = i.next().unwrap().parse().ok().unwrap();
  (a, b, c)
}
