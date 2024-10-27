#![allow(unused_imports, dead_code, non_camel_case_types)]

fn solve() {
    let (n, m, k): (us, us, us) = read3();

    // board -- матрица, заполненная '.' и '#'.
    let mut board: Vec<Vec<u8>> = vec![];
    for _ in 0..n {
        board.push(read_bytes());
    }

    // Допустим, что выбрано k башен с радиусами r_1, ..., r_k и силами поражения p_1, ..., p_k. Пусть
    // для каждой i-й башни известно число a_i(r) -- количество клеток пути монстра, которые башня
    // простреливает при радиусе r.
    // Здоровье монстра будет равно h + 3^r_1 + 3^r_2 + ... + 3^r_k, оно должно быть меньше суммарного
    // урона от всех башен: p_1 * a_1(r_1) + p_2 * a_2(r_2) + ... + p_k * a_k(r_k). Отсюда следует,
    // что h <= Сумма (p_i * a_i(r_i) - 3^r_i), где i = 1..k. Будем искать максимальное значение
    // этой суммы, она и будет значением максимального допустимого здоровья монстра.

    // Заметим, что максимальный радиус достаточно мал, он не превосходит 12. Окружность с
    // радиусом r содержит примерно Pi * r^2 клеток. Предположим, что путь монстра проходит
    // по всем клеткам, которые простреливает самая мощная башня. Тогда она нанесет урон
    // примерно 500 * Pi * r^2, что меньше чем 3^12. При всех r >= 12 величина
    // 500 * Pi * r^2 - 3^r отрицательна.

    // Построим матрицу, в которой i-я строка соответствует i-й башне, а j-е значение в i-й строке
    // равно p_i * a_i(j) - 3^j. То есть для каждой башни посчитаем ее урон при всех возможных
    // радиусах (значение радиуса -- номер столбца). Ответом для исходной задачи будет максимальная
    // сумма некоторых элементов матрицы, среди которых никакие два не находятся на одной строке
    // или столбце.

    // Читаем информацию о башнях и для каждой башни вычисляем новую строку матрицы.
    let max_r = 12;
    let mut matrix: Vec<Vec<i64>> = Vec::new();
    for _ in 0..k {
        let (mut x0, mut y0, p): (i32, i32, i64) = read3();

        // Индексируем от нуля.
        x0 -= 1;
        y0 -= 1;

        let n = n as i32;
        let m = m as i32;
        // Формируем строку матрицы.
        let mut v: Vec<i64> = vec![0; max_r as us + 1];
        // Перебираем все возможные радиусы.
        for radius in 1..=max_r {
            // area -- количество клеток пути монстра, которые поражаются башней (x0, y0) с
            // радиусом radius
            let mut area = 0;
            for x in x0 - max_r..x0 + max_r {
                for y in y0 - max_r..y0 + max_r {
                    if x >= 0
                        && x < n
                        && y >= 0
                        && y < m
                        && (x - x0).pow(2) + (y - y0).pow(2) <= radius * radius
                        && board[x as us][y as us] == b'#'
                    {
                        area += 1;
                    }
                }
            }
            v[radius as us] = area * p - 3i64.pow(radius as u32);
        }
        matrix.push(v);
    }

    // Найдем элементы с максимальной суммой и с попарно не совпадающими строками и столбцами.
    // Используем ДП по множеству столбцов.
    // Инвариант: dp[T] = (R, S), где T - некоторое множество столбцов матрицы, S - наибольшая
    // сумма, которую можно получить из элементов со столбцами T, R_S - множество строк, из
    // которых выбраны эти элементы.
    // Динамический переход. Как найти dp[T]? Рассмотрим все dp[T\t], где t \in T.
    // Пусть T = {t_1, ..., t_h}, тогда dp[T\t] = {dp[T\t_1], dp[T\t_2], ..., dp[T\t_h]}, или:
    // {(R_1, S_1), ..., (R_h, S_h)}. Каждая пара отражает максимум на T без одного элемента.
    // Для каждой пары (R_i, S_i) будем находить максимальный элемент в столбце t_i, не
    // находящийся в строках R_i. В полученных парах выберем ту, у которой максимальная сумма.
    // Это и будет значением dp[T].
    // Заметим, что множество удобно задавать битовой маской.
    let max_bits = max_r as us;
    let mut dp: Vec<(HashSet<us>, i64)> = vec![(HashSet::new(), 0); 1 << max_bits];

    let mut res = 0;
    // Перебираем все возможные множества (bitmask = T из описания выше).
    for bitmask in 0..dp.len() {
        // Перебираем возможные значения столбцов (col = t из описания выше).
        for col in 0..max_bits {
            // Если столбец col входит в множество bitmask
            if ((1 << col) & bitmask) > 0 {
                // j - множество bitmask без столбца col. j = T \ t
                let j = bitmask & !(1 << col);
                let (rows, s) = &dp[j];

                // Ищем максимум в столбце col, с номером строки не содержащимся в rows.
                let (mut max_row, mut max_val) = (0, 0);
                for (k, v) in matrix.iter().enumerate() {
                    if v[col] > max_val && !rows.contains(&k) {
                        max_val = v[col];
                        max_row = k;
                    }
                }

                // Обновляем максимум на множестве bitmask
                if s + max_val > dp[bitmask].1 {
                    let mut rows = rows.clone();
                    let s = s + max_val;
                    res = res.max(s);
                    rows.insert(max_row);
                    dp[bitmask] = (rows, s);
                }
            }
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

use inp::read2;
use inp::read3;
use inp::read_bytes;

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
