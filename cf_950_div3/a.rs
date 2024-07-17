#![allow(unused_imports, dead_code, non_camel_case_types)]

use std::collections::HashMap;
use std::collections::HashSet;

fn solve() {
    let (_, m) = inp::read2::<us>();
    let a = inp::read_bytes();

    let mut s = 0;
    for c in b'A'..=b'G' {
        let count = a.iter().filter(|&x| *x == c).count();

        if count < m {
            s += m - count;
        }
    }

    println!("{}", s);
}

fn main() {
    for _ in 0..inp::read1() {
        solve();
    }
}

type us = usize;

mod inp {
    use std::io;
    use std::str::FromStr;

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

    pub fn read2<T: FromStr>() -> (T, T) {
        let input = read_line();
        let mut i = input.split_whitespace();
        let a: T = i.next().unwrap().parse().ok().unwrap();
        let b: T = i.next().unwrap().parse().ok().unwrap();
        (a, b)
    }

    pub fn read3<T: FromStr>() -> (T, T, T) {
        let input = read_line();
        let mut i = input.split_whitespace();
        let a: T = i.next().unwrap().parse().ok().unwrap();
        let b: T = i.next().unwrap().parse().ok().unwrap();
        let c: T = i.next().unwrap().parse().ok().unwrap();
        (a, b, c)
    }
}
