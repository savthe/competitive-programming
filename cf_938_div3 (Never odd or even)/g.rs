#![allow(unused_imports, dead_code, non_camel_case_types)]

// Алгоритм Евклида
fn gcd(a: us, b: us) -> us {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn solve() {
    let (n, m): (us, us) = inp::read2();
  
    // ДП. dp[i][j] = {множество всех НОДов, которые можно получить в этой клетке}. Достаточно
    // хранить только одну строку (v) и спускаться построчно вниз.
    let mut v: Vec<Vec<us>> = Vec::new();

    // Считываем первую строки и заменяем каждый элемент НОДом на префиксе, от 0 до данного
    // элемента: a0, gcd(a0, a1), gcd(a0, a1, a2), gcd(a0, a1, a2, a3), ...
    let mut t = 0;
    for &x in inp::read_vec::<us>().iter() {
        t = gcd(t, x);
        v.push(vec![t]);
    }

    // u - список наибольших делителей. Функция combine добавляет x в этот список, если ни один
    // элемент списка не делится на x. Если в списке есть делитель x, то x заменяет его. 
    let combine = |u: &mut Vec<us>, x: us| {
        for t in u.iter_mut() {
            if *t % x == 0 {
                return;
            }
            if x % *t == 0 {
                *t = x;
                return;
            }
        }
        u.push(x);
    };

    // Считываем остальные строки матрицы.
    for j in 1..n {
        let a = inp::read_vec();
        let mut s = vec![];
        for x in v[0].iter() {
            combine(&mut s, gcd(a[0], *x));
        }
        v[0] = s;

        for i in 1..m {
            let mut s = vec![];
            for x in v[i - 1].iter() {
                combine(&mut s, gcd(*x, a[i]));
            }
            for x in v[i].iter() {
                combine(&mut s, gcd(*x, a[i]));
            }
            v[i] = s;
        }
    }

    v[m - 1].sort_unstable();
    println!("{}", v[m - 1].last().unwrap());
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

use inp::read1;
use inp::read2;
use inp::read3;
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
}
