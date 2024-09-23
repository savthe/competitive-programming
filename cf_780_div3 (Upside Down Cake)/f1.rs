#![allow(unused_imports, dead_code, non_camel_case_types)]

// Заметим, что если в подстроке минусов больше, чем плюсов на величину, кратную 3, тогда она
// перспективная. Пусть плюсов n, а минусов n + 3k, тогда в подстроке найдется по крайней мере
// 2k пар минусов, стоящих справа. Сделаем из них k плюсов. Получим, что плюсов n + k, минусов
// столько же.
// Пусть баланс подстроки: (количество минусов) - (количество плюсов). Если плюсов n, тогда
// баланс: (n + 3k) - (n) = 3k - величина, кратная 3. Подстрока является перспективной, если ее
// баланс делится на 3 и притом положительный (минусов не должно быть меньше чем плюсов).
fn solve() {
    let _: us = inp::read1();
    let v = inp::read_bytes();
    let mut result = 0;
    // Рассматриваем все подстроки с левой границей в a.
    for a in 0..v.len() {
        let mut balance: i32 = 0;
        // Перебираем правые границы, для каждой обновляя баланс.
        for b in &v[a..v.len()] {
            match *b {
                b'-' => balance += 1,
                _ => balance -= 1,
            }
            if balance >= 0 && balance % 3 == 0 {
                result += 1;
            }
        }
    }
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
