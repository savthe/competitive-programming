#![allow(unused_imports, dead_code, non_camel_case_types)]

fn dfs(
    now: us,
    parent: us,
    tree: &Vec<Vec<us>>,
    dp_incl: &mut Vec<i64>,
    dp_excl: &mut Vec<i64>,
    c: i64,
) {
    for &nb in tree[now].iter() {
        if nb == parent {
            continue;
        }

        dfs(nb, now, tree, dp_incl, dp_excl, c);
        // Here we attach subtree with root 'nb' to new root 'now'.
        // If 'now' is included and 'nb' is included, then score is their sum minus 2c (both take c
        // from each other), If 'nb' is not included, it won't take c from 'now', and we won't pay
        // for including 'now'.
        dp_incl[now] += dp_excl[nb].max(dp_incl[nb] - 2 * c);

        // If 'now' is not included, we don't pay for including or not including 'nb'.
        dp_excl[now] += dp_excl[nb].max(dp_incl[nb]);
    }
}

fn solve() {
    let (n, c): (us, i64) = read2();
    let mut a: Vec<i64> = read_vec();
    a.insert(0, 0);
    let mut tree: Vec<Vec<us>> = vec![Vec::new(); n + 1];
    for _ in 0..n - 1 {
        let (u, v): (us, us) = read2();
        tree[u].push(v);
        tree[v].push(u);
    }

    // Let's root our tree at vertex 1 and consider its subtrees walking from leaves. For each
    // vertex we keep pair of values:
    // dp_incl[i] = maximum score for a subtree rooted at i if i is included.
    // dp_excl[i] = maximum score for a subtree rooted at i if i is excluded.
    let mut dp_incl = a.clone();
    let mut dp_excl = vec![0i64; n + 1];
    dfs(1, 0, &tree, &mut dp_incl, &mut dp_excl, c);

    let res = dp_incl[1].max(dp_excl[1]);
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
