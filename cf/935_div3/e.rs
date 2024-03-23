use std::io;
use std::str::FromStr;

fn solve() {
  let (n, x) = read2::<usize, usize>();
  let mut p = read_vec::<usize>();
  // Shift one element right.
  p.insert(0, 0);

  // Runs suggested binsearch and returns the terminal index (l).
  let binsearch = |v: &Vec<usize>| {
    let mut l = 1;
    let mut r = n + 1;
    loop {
      if r - l == 1 { break; }
      let m = (r + l) / 2;
      if v[m] <= x { l = m; }
      else { r = m; }
    }
    l
  };

  // If x is already an answer.
  if p[binsearch(&p)] == x {
    println!("0");
  }
  else {
    // If not, we swap it with the last element.
    let mut xpos = p.iter().position(|&t| t == x).unwrap();
    let mut result = vec![];
    if xpos < n { 
      p.swap(xpos, n); 
      result.push((xpos, n));
      xpos = n;
    }
    // See where binpow stops if x is the last element.
    let l = binsearch(&p);
    // If it stopped not on x, we suggest to swap last element with l (print it to answer).
    if p[l] != x {
      result.push((xpos, l));
    }

    println!("{}", result.len());
    for &(a, b) in result.iter() {
      println!("{a} {b}");
    }
  }


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
