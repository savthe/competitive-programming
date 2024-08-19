#![allow(unused_imports, dead_code, non_camel_case_types)]

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

fn solve() {
    let n = inp::read1::<us>();
    // Для решения задачи посчитаем количество точек на каждой линии (на каждой вертикали,
    // горизонтали, восходящей диагонали, нисходящей диагонали).
    // Линии каждого типа можно однозначно сопоставить целое число. Например, семейство клеток с
    // координатам (x, 2) - горизонтальная линия на высоте 2. Ей поставим в соответствие значение
    // ординаты (2).
    // vert[t] = количество точек на вертикальной линии (t, y). y здесь -- переменная.
    let mut vert: HashMap<i32, us> = HashMap::new();
    let mut hor: HashMap<i32, us> = HashMap::new();
    let mut desc: HashMap<i32, us> = HashMap::new();
    let mut asc: HashMap<i32, us> = HashMap::new();
    for _ in 0..n {
        let (x, y): (i32, i32) = inp::read2();
        // Инавриант вертикальной прямиой (x, y) -> x, т.к. первая координата не меняется.
        *vert.entry(x).or_default() += 1;

        // Инавриант горизонтальной прямиой (x, y) -> y, т.к. вторая координата не меняется.
        *hor.entry(y).or_default() += 1;

        // Инвариант нисходящей прямой (x, y) -> x + y. Рассмотрим две точки на нисходящей прямой:
        // (x, y) и (x + t, y - t). Заметим, что сумма координат не зависит от t и равна x + y.
        *desc.entry(x + y).or_default() += 1;

        // Инвариант восходящей прямой (x, y) -> x - y.
        *asc.entry(x - y).or_default() += 1;
    }

    // Лямбда, вычисляющая количество вариантов выбрать две неравнозначные точки на прямой.
    // Если всего точке k, то первую можно выбрать k способами, а вторую: k - 1.
    let f = |hm: &HashMap<i32, us>| {
        let mut s = 0;
        for (_, &k) in hm.iter() {
            s += k * (k - 1);
        }
        s
    };

    let res = f(&vert) + f(&hor) + f(&desc) + f(&asc);

    println!("{}", res);
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
