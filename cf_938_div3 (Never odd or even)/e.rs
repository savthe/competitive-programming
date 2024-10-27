#![allow(unused_imports, dead_code, non_camel_case_types)]

fn test(v: &[us], k: us) -> bool {
    // Будем жадно применять инверсию к отрезку, начиная с первого встретившегося нуля. Реально
    // менять массив потребует на каждом шаге O(k) замен, что дорого. При необходимости применить
    // инверсию, запишем индекс последнего инвертированного элемента в очередь, а когда дойдем
    // до этого элемента, удалим его из очереди. Тогда значение элемента i с учетом инверсий
    // будет равно (v[i] + q.len()) % 2.
    let mut q: VecDeque<us> = VecDeque::new();

    for i in 0..v.len() {
        // Если текущий элемент равен нулю, инвертируем его (просто записываем в очередь индекс
        // конца инверсии).
        if (v[i] + q.len()) % 2 == 0 {
            if i + k - 1 < v.len() {
                q.push_back(i + k - 1);
            } else {
                // Если встретили 0, а места для инверсии не осталось.
                return false;
            }
        }

        // Если первый элемент очереди совпал с текущим индексом, то на данной позиции закончился
        // некоторый отрезок инверсии.
        if !q.is_empty() && *q.front().unwrap() == i {
            q.pop_front();
        }
    }

    return true;
}

fn solve() {
    let n: us = read1();
    // s - массив из 0 и 1.
    let s = read_line()
        .bytes()
        .map(|x| (x - b'0') as us)
        .collect::<Vec<us>>();

    let mut res = 0;
    // d - длина последовательности. Найдем наибольшую, при которой возможно превратить все 0 в 1.
    for d in 1..=n {
        if test(&s, d) {
            res = d;
        }
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
