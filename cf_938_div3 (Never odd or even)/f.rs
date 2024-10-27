#![allow(unused_imports, dead_code, non_camel_case_types)]

fn solve() {
    let (n1, n2, n3, n4): (us, us, us, us) = inp::read4();
    // Предположим, что Ева переставила элементы так, чтобы удалять каждый раз число с конца.
    // Из-за инволютивности XOR, получить элемент, отличный от нуля можно не более, чем в
    // половине игр.
    // Пусть все количества - четные числа. Тогда ответ - половина от общего количества
    // (например, расставим все числа в отсортированном порядке). Пусть четверок нечетное
    // количество (4 = 100 в двоичном виде). Тогда одну четверку не получится занулить
    // никакой комбинацией 1 (0001), 2(0010), 3(011). Ее Ева поставит в конец и после
    // первой же игры, четверок станет четное количество. А что с числами 1,2,3. Если
    // одно из них вошло нечетное число раз, то отменить его можно только суммой двух
    // других (010 = 011 ^ 001, 011 = 010 ^ 001, 001 = 011 ^ 010).
    // Максимальное количество побед: сумма половин от каждого количества с округлением
    // вниз и плюс 1, если имеется нечетное количество 1, 2 и 3.
    let mut res = n1 / 2 + n2 / 2 + n3 / 2 + n4 / 2;
    if n1 % 2 == 1 && n2 % 2 == 1 && n3 % 2 == 1 {
        res += 1;
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
