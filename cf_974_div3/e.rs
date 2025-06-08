#![allow(unused_imports, dead_code, non_camel_case_types)]

fn solve() {
    let (n, m, _) = read3::<us, us, us>();
    let mut horsed = vec![false; n + 1];
    for a in read_vec::<us>() {
        horsed[a] = true;
    }
 
    let mut graph: Vec<Vec<(us, us)>> = vec![Vec::new(); n + 1];
    for _ in 0..m {
        let (u, v, w) = read3::<us, us, us>();
        graph[u].push((w, v));
        graph[v].push((w, u));
    }
 
    let max = std::usize::MAX / 2;
    // Runs Dijkstra allowing horse riding.
    let traverse = |start: us| -> Vec<us>{
        // A priority queue of elements (-min_distance, vertex)
        let mut q: BinaryHeap<(i64, us)> = BinaryHeap::new();
        q.push((0, start));

        // i-th element is the minimal distance for reaching i walking.
        let mut walk_dist = vec![max; n + 1];
        walk_dist[start] = 0;
        // i-th element is the minimal distance for reaching i by horse
        // (includes taking horse in i).
        let mut horse_dist = vec![max; n + 1];
        // false if we've never walked into i.
        let mut walk_used = vec![false; n + 1];
        // false if we've never horsed into in i.
        let mut horse_used = vec![false; n + 1];
        while let Some((_, now)) = q.pop() {
            // If there is a horse in this vertex, then we are riding now.
            if horsed[now] {
                horse_dist[now] = horse_dist[now].min(walk_dist[now]);
            }
            if !walk_used[now] {
                walk_used[now] = true;
                for &(d, v) in graph[now].iter() {
                    if walk_dist[now] + d < walk_dist[v] {
                        walk_dist[v] = walk_dist[now] + d;
                        q.push((-(walk_dist[v] as i64), v));
                    }
                }
            }
 
            if !horse_used[now] && horse_dist[now] < max {
                horse_used[now] = true;
                for &(d, v) in graph[now].iter() {
                    if horse_dist[now] + d / 2 < horse_dist[v] {
                        horse_dist[v] = horse_dist[now] + d / 2;
                        q.push((-(horse_dist[v] as i64), v));
                    }
                }
            }
        }
        // Minimal distance for reaching i is the minimal of reaching it walking and by horse.
        walk_dist.iter().zip(horse_dist.iter()).map(|(&a, &b)| a.min(b)).collect::<Vec<us>>()
    };
 
    // Mariam's minimal distances to each vertex.
    let marian = traverse(1);
    // Robin's minimal distances to each vertex.
    let robin = traverse(n);
 
    // Minimal time to meet in i is max(marian[i], robin[i]) -- someone will have to wait.
    // Result is the minmal of these maximums.
    let res = marian.iter().zip(robin.iter()).map(|(a, b)| *(a.max(b))).min().unwrap();
    
    if res < max {
        println!("{}", res);
    }
    else {
        println!("-1");
    }
}

fn main() {
    for _ in 0..inp::read1() {
        solve();
    }
}

/* Auxiliary stuff */
type us = usize;
    
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use inp::read1;
use inp::read2;
use inp::read3;
use inp::read4;
use inp::read5;
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

    pub fn read5<T1: FromStr, T2: FromStr, T3: FromStr, T4: FromStr, T5: FromStr>(
    ) -> (T1, T2, T3, T4, T5) {
        let input = read_line();
        let mut i = input.split_whitespace();
        let a: T1 = i.next().unwrap().parse().ok().unwrap();
        let b: T2 = i.next().unwrap().parse().ok().unwrap();
        let c: T3 = i.next().unwrap().parse().ok().unwrap();
        let d: T4 = i.next().unwrap().parse().ok().unwrap();
        let e: T5 = i.next().unwrap().parse().ok().unwrap();
        (a, b, c, d, e)
    }
}
