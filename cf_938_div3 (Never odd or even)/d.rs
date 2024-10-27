#![allow(unused_imports, dead_code, non_camel_case_types)]

fn solve() {
    let (n, m, k): (us, us, us) = read3();
    let a: Vec<us> = read_vec();
    let b: Vec<us> = read_vec();

    // Счетчик элементов в массиве b.
    let mut count_b: HashMap<us, us> = HashMap::new();
    for &x in b.iter() {
        *count_b.entry(x).or_insert(0) += 1;
    }

    // Счетчик элементов в скользящем окне frame длины m.
    let mut frame: HashMap<us, us> = HashMap::new();
    for &x in a.iter().take(m) {
        *frame.entry(x).or_insert(0) += 1;
    }

    // s - число элементов в b, покрытых рамкой frame.
    let mut s = 0;
    for (&x, &c) in count_b.iter() {
        let f = *frame.entry(x).or_default();
        s += c.min(f);
    }

    let mut res = 0;
    if s >= k {
        res += 1;
    }

    // Двигаем рамку.
    let mut i = 0;
    while i + m < n {
        // Из рамки выпадает элемент a[i].
        let drop = a[i];
        let f = frame.entry(drop).and_modify(|x| *x -= 1).or_default();
        let c = count_b.entry(drop).or_default();
        if f < c {
            s -= 1;
        }

        // Добавляем в рамку a[i + m].
        let get = a[i + m];
        let f = frame.entry(get).and_modify(|x| *x += 1).or_insert(1);
        let c = count_b.entry(get).or_default();
        if f <= c {
            s += 1;
        }

        if s >= k {
            res += 1;
        }
        i += 1;
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
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use inp::read2;
use inp::read3;
use inp::read_bytes;
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
}
