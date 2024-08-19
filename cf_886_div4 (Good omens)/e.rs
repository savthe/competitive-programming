#![allow(unused_imports, dead_code, non_camel_case_types)]

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

// Вычисляет суммарную площадь в предположении, что ширина рамки равна w.
fn f(w: us, s: &[us], c: us) -> us {
    let mut sum = 0;
    for x in s.iter() {
        sum += (x + 2 * w) * (x + 2 * w);
        if sum > c {
            break;
        }
    }
    sum
}

fn solve() {
    let (_, c) = inp::read2::<us, us>();
    let s = inp::read_vec::<us>();

    // Бинпоиск по ответу. Будем наращивать ширину w на d. При условии, что d слишком большая,
    // уменьшим d в два раза.
    let mut w = 0;

    // Оценим ширину рамки сверху некоторой степенью двойки. Предположим, что картина всего одна,
    // а ее ширина равна нулю.
    let mut d = 1;
    while 4 * d * d < c {
        d *= 2;
    }

    while d > 0 {
        // Пробуем нарастить ширину на d.
        if f(w + d, &s, c) <= c {
            w += d;
        }
        d /= 2;
    }
    println!("{w}");
}

fn main() {
    for _ in 0..inp::read1() {
        solve();
    }
}

// Auxiliary stuff
type us = usize;

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
}
