#![allow(unused_imports, dead_code, non_camel_case_types)]

fn solve() {
    let (n, m, k): (us, us, us) = read3();
    let mut s: Vec<(us, us)> = Vec::new();
    let mut res: us = 0;

    // x is the current day at which Little John wants to drink milk.
    let mut x = 0;
    for i in 0..=n {
        let (r, cur_a): (us, us) = if i < n { read2() } else { (1000_000_000, 0) };

        while let Some((d, mut a)) = s.pop() {
            // Want to drink in days [x, r), for period until d the problem is solved before.
            x = x.max(d);

            // If the milk is bad.
            if d + k - 1 < x {
                s.clear();
                break;
            }

            // min(d + k - x, r - x) is the amount of milk we want to drink.
            // a / m is the amount we have in current jar.
            let c = (r.min(d + k) - x).min(a / m);
            // Drinc c pints.
            x += c;
            res += c;
            a -= c * m;
            // If LJ could drink fully all days, there might something left, put it back.
            if x == r {
                s.push((d, a));
                break;
            } else if let Some(last) = s.last_mut() {
                // If current jar is not enough, put leftovers into next available jar, and
                // continue drinking.
                last.1 += a;
            }
        }

        s.push((r, cur_a));
    }
    println!("{}", res);
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
