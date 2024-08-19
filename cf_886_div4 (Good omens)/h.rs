#![allow(unused_imports, dead_code, non_camel_case_types)]

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

fn solve() {
    let (n, m): (us, us) = inp::read2();

    // Рассмотрим граф, в котором каждая вершина обозначает одного солдата. Если солдат a должен
    // быть левее солдата b на расстоянии d, то из вершины a проведем в b ребро с весом d.
    // graph[x] - вектор соседей вершины x, где каждый сосед указан с весом ребра, то есть парой
    // (вершина, вес).
    let mut graph: Vec<Vec<(us, i64)>> = vec![];
    graph.resize(n + 1, vec![]);

    for _ in 0..m {
        let (a, b, d): (us, us, i64) = inp::read3();
        // Если a левее b на d метров, то b "левее" a на -d метров.
        graph[a].push((b, d));
        graph[b].push((a, -d));
    }

    // Будем линейно перебирать вершины. Если вершина еще не посещена, то запустим из нее поиск в
    // глубину и обойдем всю компоненту связности, содержащую данную вершину. Во время обхода будем
    // вычислять расстояние от исходной вершины, проверяя их согласованность (расстояние до каждой
    // вершины не должно зависеть от пути, по которому мы в нее пришли, иначе "NO").
    let mut dist = vec![0; n + 1];
    let mut used = vec![false; n + 1];
    let mut stack = vec![];
    for i in 1..=n {
        // Если вершина не посещена, то начинаем обход. Кладем ее в стек.
        if !used[i] {
            stack.push(i);
        }

        while let Some(now) = stack.pop() {
            // Берем вершину со стека (now) и перебираем ее соседей.
            for (nb, d) in graph[now].iter() {
                // Если сосед не встречался, отмечаем, что встречался, вычисляем расстояние до него
                // и помещаем в стек для дальнейшего обхода.
                if !used[*nb] {
                    used[*nb] = true;
                    dist[*nb] = dist[now] + d;
                    stack.push(*nb);
                }

                // Если расстояние до соседа уже найдено и не равно расстоянию, полученному
                // нынешним путем, то расстояния не согласованы.
                if dist[*nb] != dist[now] + d {
                    println!("NO");
                    return;
                }
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
