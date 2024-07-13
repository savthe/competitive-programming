#![allow(unused_imports, dead_code, non_camel_case_types)]

use std::collections::HashMap;
use std::collections::HashSet;

fn solve() {
    let _n = inp::read1::<us>();
    let a: Vec<usize> = inp::read_vec();
    let b: Vec<usize> = inp::read_vec();
    let _m = inp::read1::<us>();
    let d: Vec<us> = inp::read_vec();
    let mut cnt: HashMap<us, us> = HashMap::new();
    // Для каждого значения, которым можно заменить элементы a, находим количество таких значений.
    for &x in d.iter() {
        *cnt.entry(x).or_insert(0) += 1;
    }

    // Значение d.last() -- последнее, которым заменили какой-то элемент из a. Если его нет в b,
    // то b не мог получиться из a.
    if !b.contains(d.last().unwrap()) {
        println!("NO");
        return;
    }

    // Перебираем пары (a[0], b[0]), (a[1], b[1]), ... и проверяем, что для каждой пары, где
    // a[i] != b[i] найдется элемент в d (в достаточном количестве).
    for (&x, &y) in a.iter().zip(b.iter()) {
        if x != y {
            let e = cnt.entry(y).or_default();
            if *e > 0 {
                *e -= 1;
            } else {
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

type us = usize;

mod inp {
    use std::io;
    use std::str::FromStr;

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
