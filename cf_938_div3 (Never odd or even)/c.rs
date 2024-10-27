#![allow(unused_imports, dead_code, non_camel_case_types)]

fn solve() {
    let (n, mut k): (us, us) = inp::read2();
    let mut a = read_vec::<us>();

    // Два указателя, i - слева, j - справа.
    let mut i = 0;
    let mut j = n - 1;
    while k > 0 && i < j {
        // Если левый и правый корабль имеют здоровье больше 1, то атакуем пока один из них не
        // станет здоров на 1.
        if a[i] > 1 && a[j] > 1 {
            let d = (a[i] - 1).min(a[j] - 1);
            // Если не хватило сил, чтобы снять 2d здоровья.
            if k <= 2 * d {
                k = 0;
                break;
            }
            // Здоровье каждого корабля уменьшается на d, а сила кракена - на 2d.
            k -= 2 * d;
            a[i] -= d;
            a[j] -= d;
        } else {
            // Один из кораблей имеет здоровье 1.

            // Кракен атакует левый корабль.
            a[i] -= 1;
            k -= 1;
            if a[i] == 0 {
                i += 1;
            }

            // Кракен атакует правый корабль.
            if k > 0 {
                a[j] -= 1;
                k -= 1;
            }
            if a[j] == 0 {
                j -= 1;
            }
        }
    }

    // Если можно потопить левый корабль.
    if k >= a[i] {
        i += 1;
    }

    let mut r = i + n - j - 1;
    if i > j {
        r = n;
    }
    println!("{}", r);
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
