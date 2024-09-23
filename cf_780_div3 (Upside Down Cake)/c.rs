#![allow(unused_imports, dead_code, non_camel_case_types)]

fn solve() {
    let s = inp::read_bytes();

    let mut result = 0;

    // Начало текущего полуинтервала.
    let mut offset = 0;

    while offset < s.len() {
        // Будем искать первый символ на полуинтервале [offset + 1, s.len), который встретился
        // второй раз.
        let mut seen: Vec<bool> = vec![false; 256];
        // Отмечаем символ s[offset] как уже увиденный.
        seen[s[offset] as us] = true;

        // Перебираем остальные символы.
        let mut i = offset + 1;
        while i < s.len() && !seen[s[i] as us] {
            seen[s[i] as us] = true;
            i += 1;
        }

        // Если нашли символ, а не дошли до конца строки.
        if i < s.len() {
            // Нужно удалить столько символов, сколько на отрезке [offset, i], минус два (они -
            // парные).
            result += i - offset - 1;
        } else {
            // Если дошли до конца строки, удаляем все символы на данном полуинтервале.
            result += i - offset;
        }

        // Переходим к следующему полуинтервалу.
        offset = i + 1;
    }

    println!("{result}");
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
