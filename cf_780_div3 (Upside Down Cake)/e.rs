#![allow(unused_imports, dead_code, non_camel_case_types)]

// Будем искать наибольшее количество единиц на какой-нибудь диагонали с учетом циклических сдвигов.
fn solve() {
    let _ = inp::read_line();
    let n = inp::read1::<us>();
    let mut m: Vec<Vec<u8>> = vec![];
    let mut ones = 0;
    for _ in 0..n {
        let s = inp::read_line();
        let v: Vec<u8> = s.bytes().map(|x| x - b'0').collect();
        // При вводе считаем количество единиц.
        ones += v.iter().filter(|&x| *x == 1).count();
        m.push(v);
    }

    // Максимальное число единиц на некоторой диагонали.
    let mut diag_ones = 0;
    // i - номер столбца. Удобно представлять, что мы рассматриваем все диагонали, которые
    // начинаются с элементов нулевой строки.
    for i in 0..n {
        // Счетчик единиц на данной диагонали.
        let mut cur = 0;
        // Идем по диагонали с началом в m[0][i] вниз с учетом циклических сдвигов столбцов.
        for j in 0..n {
            let row = j % n;
            let col = (i + j) % n;
            cur += m[row][col] as us;
            diag_ones = diag_ones.max(cur);
        }
    }

    // Нужно обнулить все элементы, кроме тех, что на найденной диагонали, их: ones - diag_ones.
    // Нужно добавить недостающие единицы, их: n - diag_ones.
    let result = ones - diag_ones + n - diag_ones;
    println!("{}", result);
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
