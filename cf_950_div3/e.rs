#![allow(unused_imports, dead_code, non_camel_case_types)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;

fn solve() {
    let (n, m) = inp::read2::<us>();

    let a = inp::read_matrix::<i32>(n);
    let b = inp::read_matrix::<i32>(n);
 
    // Для каждого элемента в первой строке храним пару (значение, номер столбца).
    let mut col_pivots: BTreeMap<us, us> = BTreeMap::new();
    for (i, x) in a.first().unwrap().iter().enumerate() {
        col_pivots.entry(*x as us).or_insert(i);
    }
    
    // Для каждого элемента в первом столбце храним пару (значение, номер строки).
    let mut row_pivots: BTreeMap<us, us> = BTreeMap::new();
    for (i, v) in a.iter().enumerate() {
        row_pivots.entry(*v.first().unwrap() as us).or_insert(i);
    }
 
    // Перестановка строк. row_map[i] = j  <=>  строка i в матрице a соответствет строке j в b.
    let mut row_map = vec![0; n];
    // Перестановка столбцов.
    let mut col_map = vec![0; m];

    // Ищем в матрице b элементы, по которым восстановим перестановку -- элементы из первой строки
    // и первого столбца матрицы a.
    for (row_idx, v) in b.iter().enumerate() {
        for (col_idx, x) in v.iter().enumerate() {
            row_pivots.entry(*x as us).and_modify(|i| row_map[*i] = row_idx);
            col_pivots.entry(*x as us).and_modify(|i| col_map[*i] = col_idx);
        }
    }
 
    // Проверяем, что перестановки row_map и col_map переводят все элементы a в b.
    for (i, v) in a.iter().enumerate() {
        let ii = row_map[i];
        for (j, x) in v.iter().enumerate() {
            let jj = col_map[j];
            if *x != b[ii][jj] {
                println!("NO");
                return;
            }
        }
    }
 
    println!("YES");
}

fn main() {
    for _ in 0..inp::read1() {
        solve();
    }
}

/* Auxiliary stuff */
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
