#![allow(unused_imports, dead_code, non_camel_case_types)]

// Возвращает степень двойки произведения на данном подмассиве, если оно положительно.
fn prod(a: &[i32]) -> Option<us> {
    let mut pow = 0;
    let mut sign = 1;

    for &x in a.iter() {
        if x < 0 {
            sign = -sign;
        }
        if x == 2 || x == -2 {
            pow += 1;
        }
    }

    if sign == 1 {
        Some(pow)
    } else {
        None
    }
}

fn solve() {
    let n = inp::read1::<us>();
    let a = inp::read_vec::<i32>();

    // max_low - начало полуинтервала с максимальным найденным произведением.
    // max_high - конец полуинтервала -//-
    // max_pow - максимальное произведение (степень двойки).
    let (mut max_low, mut max_high, mut max_pow) = (0, 0, 0);

    // Обрабатывает заданный полуинтервал.
    let mut process = |low: us, high: us| {
        // Если произведение положительно, проверяем его на максимум.
        if let Some(pow) = prod(&a[low..high]) {
            if pow > max_pow {
                max_pow = pow;
                max_low = low;
                max_high = high;
            }
        }
    };

    // Начало текущего полуинтервала.
    let mut offset = 0;
    for i in 0..=a.len() {
        // Ищем конец полуинтервала (ноль или конец массива).
        if i < a.len() && a[i] != 0 {
            continue;
        }

        // Полуинтервал [offset, i) состоит из ненулевых элементов.

        // Проверяем его на максимальность произведения.
        process(offset, i);

        let b = &a[offset..i];

        // Находим первый отрицательный элемент и рассматриваем полуинтервал с началом в следующем
        // элементе.
        if let Some(idx) = b.iter().position(|&x| x < 0) {
            process(offset + idx + 1, i);
        }

        // Находим последний отрицательный элемент и рассматриваем полуинтервал с концом в этом
        // элементе.
        if let Some(idx) = b.iter().rposition(|&x| x < 0) {
            process(offset, offset + idx);
        }

        // Переходим к следующему полуинтервалу.
        offset = i + 1;
    }

    println!("{} {}", max_low, n - max_high);
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
