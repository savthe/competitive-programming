#![allow(unused_imports, dead_code, non_camel_case_types)]

fn solve() {
    let (n, d, k): (us, us, us) = read3();
    // Number of jobs that start at i.
    let mut lefts: Vec<us> = vec![0; n + 1];
    // Number of jobs that finish at i.
    let mut rights: Vec<us> = vec![0; n + 1];
    for _ in 0 .. k {
        let (l, r): (us, us) = read2();
        lefts[l] += 1;
        rights[r] += 1;
    }
 
    // Number of jobs started before d + 1.
    let mut sl = lefts.iter().take(d + 1).sum::<us>();

    // Number of jobs finished before 1.
    let mut sr = 0;
    let mut robert = (sl - sr, 1);
    let mut mom = robert;
    for i in 2..=n - d + 1 {
        sr += rights[i - 1];
        sl += lefts[i + d - 1];
        // Number of jobs that intersect range [i, i + d - 1] is:
        // sl[i + d - 1] - sr[i - 1] which is the number of jobs started before i + d minus number of
        // jobs finished before i.
        let x = sl - sr;
        if robert.0 < x {
            robert = (x, i);
        }
        if mom.0 > x {
            mom = (x, i);
        }
    }
    
    println!("{} {}", robert.1, mom.1);
}

fn main() {
    for _ in 0..inp::read1() {
        solve();
    }
}

/* Auxiliary stuff */
type us = usize;
    
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use inp::read1;
use inp::read2;
use inp::read3;
use inp::read4;
use inp::read5;
use inp::read_bytes;
use inp::read_line;
use inp::read_vec;

mod inp {
    use std::io;
    use std::str::FromStr;

    pub fn read_matrix<T: FromStr>(n: usize) -> Vec<Vec<T>> {
        let mut a: Vec<Vec<T>> = vec![];
        for _ in 0..n {
            a.push(read_vec::<T>());
        }
        a
    }

    pub fn read_vec<T: FromStr>() -> Vec<T> {
        read_line()
            .split_whitespace()
            .map(|x| x.parse().ok().unwrap())
            .collect::<Vec<T>>()
    }

    pub fn read_line() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim_end().to_string()
    }

    pub fn read_bytes() -> Vec<u8> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim_end().bytes().collect()
    }

    pub fn read1<T: FromStr>() -> T {
        read_line().trim().parse().ok().unwrap()
    }

    pub fn read2<T1: FromStr, T2: FromStr>() -> (T1, T2) {
        let input = read_line();
        let mut i = input.split_whitespace();
        let a: T1 = i.next().unwrap().parse().ok().unwrap();
        let b: T2 = i.next().unwrap().parse().ok().unwrap();
        (a, b)
    }

    pub fn read3<T1: FromStr, T2: FromStr, T3: FromStr>() -> (T1, T2, T3) {
        let input = read_line();
        let mut i = input.split_whitespace();
        let a: T1 = i.next().unwrap().parse().ok().unwrap();
        let b: T2 = i.next().unwrap().parse().ok().unwrap();
        let c: T3 = i.next().unwrap().parse().ok().unwrap();
        (a, b, c)
    }

    pub fn read4<T1: FromStr, T2: FromStr, T3: FromStr, T4: FromStr>() -> (T1, T2, T3, T4) {
        let input = read_line();
        let mut i = input.split_whitespace();
        let a: T1 = i.next().unwrap().parse().ok().unwrap();
        let b: T2 = i.next().unwrap().parse().ok().unwrap();
        let c: T3 = i.next().unwrap().parse().ok().unwrap();
        let d: T4 = i.next().unwrap().parse().ok().unwrap();
        (a, b, c, d)
    }

    pub fn read5<T1: FromStr, T2: FromStr, T3: FromStr, T4: FromStr, T5: FromStr>(
    ) -> (T1, T2, T3, T4, T5) {
        let input = read_line();
        let mut i = input.split_whitespace();
        let a: T1 = i.next().unwrap().parse().ok().unwrap();
        let b: T2 = i.next().unwrap().parse().ok().unwrap();
        let c: T3 = i.next().unwrap().parse().ok().unwrap();
        let d: T4 = i.next().unwrap().parse().ok().unwrap();
        let e: T5 = i.next().unwrap().parse().ok().unwrap();
        (a, b, c, d, e)
    }
}
