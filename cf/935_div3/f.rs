use std::io;
use std::str::FromStr;
use std::collections::HashSet;

fn solve() {
  let n = read1::<usize>();
  // Read vecs starting from index 1.
  let mut v = read_vec_1::<us>(0);
  let p = read_vec_1::<us>(0);

  // Sort sequence of indexes 0..n according to descending of v[i].
  let mut indexes: Vec<us> = (0..=n).collect();
  indexes.sort_by(|&i, &j| v[j].partial_cmp(&v[i]).unwrap());

  // Keep indexes of shrooms which we picked.
  let mut hs: HashSet<us> = HashSet::new();

  // Next shroom to pick.
  let mut pick_index = 0;

  let mut max_power = 0;
  let mut min_shrooms = n;

  for take in 0..=n {
    // If we should consider permutation.
    if take >= 2 {
      let remove = p[take - 1];
      v[remove] = 0;
      hs.remove(&remove);
    }
    while hs.len() < take && pick_index <= n {
      // Picking shroom with index indexes[pick_index].
      let index = indexes[pick_index];
      // If it is valid.
      if v[index] > 0 {
        hs.insert(index);
      }
      if hs.len() * v[index] > max_power {
        max_power = hs.len() * v[index];
        min_shrooms = hs.len();
      }
      pick_index += 1;
    }
  }

  println!("{max_power} {min_shrooms}");
}

fn main() {
  for _ in 0..read1() {
    solve();
  }
}

#[allow(non_camel_case_types)]
type us = usize;

fn read_vec_1<T: FromStr>(first: T) -> Vec<T> {
  let mut v = read_vec::<T>();
  v.insert(0, first);
  v
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
