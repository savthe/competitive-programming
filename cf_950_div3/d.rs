use std::collections::BTreeMap;
use std::collections::BTreeSet;

// Алгоритм Евклида.
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Построение НОД-последовательности с вычеркиванием элемента skip.
fn make_b(a: &[i32], skip: usize) -> Vec<i32> {
    let mut b = vec![];
    for i in 0..a.len() - 1 {
        if i == skip {
            continue;
        }
        let mut next = i + 1;
        if next == skip {
            next += 1
        }
        if next < a.len() {
            b.push(gcd(a[i], a[next]));
        }
    }
    b
}

// Возвращает последнюю позицию нарушения монотонности и количество найденных нарушений.
fn test(b: &[i32]) -> (usize, usize) {
    let mut index = 0;
    let mut count = 0;
    for i in 1..b.len() {
        if b[i] < b[i - 1] {
            index = i;
            count += 1;
        }
    }
    (index, count)
}

fn solve() {
    let n = inp::read1::<usize>();
    let a = inp::read_vec::<i32>();
    // Вычеркивается индекс, которого и так нет в массиве.
    let b = make_b(&a, 1_000_000);

    // Находим позицию нарушения монотонности и количество нарушений.
    let (index, count) = test(&b);
    if count == 0 {
        println!("YES");
        return;
    }

    // Рассматриваем 3 варианта: 1) исключить предыдущий элемент, 2) исключить текущий, 
    // 3) исключить следующий.
    let b = make_b(&a, index - 1);
    if test(&b) == (0, 0) {
        println!("YES");
        return;
    }
    let b = make_b(&a, index);
    if test(&b) == (0, 0) {
        println!("YES");
        return;
    }
    let b = make_b(&a, index + 1);
    if test(&b) == (0, 0) {
        println!("YES");
        return;
    }

    println!("NO");
}

fn main() {
    for _ in 0..inp::read1() {
        solve();
    }
}

#[allow(non_camel_case_types, dead_code)]
type us = usize;

mod inp {
    use std::io;
    use std::str::FromStr;
    #[allow(dead_code)]
    pub fn read_vec_1<T: FromStr>(first: T) -> Vec<T> {
        let mut v: Vec<T> = read_line()
            .split_whitespace()
            .map(|x| x.parse().ok().unwrap())
            .collect::<Vec<T>>();
        v.insert(0, first);
        v
    }

    #[allow(dead_code)]
    pub fn read_vec<T: FromStr>() -> Vec<T> {
        read_line()
            .split_whitespace()
            .map(|x| x.parse().ok().unwrap())
            .collect::<Vec<T>>()
    }

    #[allow(dead_code)]
    pub fn read_line() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim_end().to_string()
    }

    #[allow(dead_code)]
    pub fn read1<T: FromStr>() -> T {
        read_line().trim().parse().ok().unwrap()
    }

    #[allow(dead_code)]
    pub fn read2<T1: FromStr, T2: FromStr>() -> (T1, T2) {
        let input = read_line();
        let mut i = input.split_whitespace();
        let a: T1 = i.next().unwrap().parse().ok().unwrap();
        let b: T2 = i.next().unwrap().parse().ok().unwrap();
        (a, b)
    }

    #[allow(dead_code)]
    pub fn read3<T1: FromStr, T2: FromStr, T3: FromStr>() -> (T1, T2, T3) {
        let input = read_line();
        let mut i = input.split_whitespace();
        let a: T1 = i.next().unwrap().parse().ok().unwrap();
        let b: T2 = i.next().unwrap().parse().ok().unwrap();
        let c: T3 = i.next().unwrap().parse().ok().unwrap();
        (a, b, c)
    }
}
