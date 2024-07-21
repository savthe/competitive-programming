#![allow(unused_imports, dead_code, non_camel_case_types)]

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
struct Fontain {
    col: us,
    row: us,
    idx: us,
    result: us,
}

// Pivot - ключевой/поворотный фонтан. Это внешние угловые фонтаны, которые ограничивают область
// Алисы.
// Функция принимает массив фонтанов, считая, что он отсортирован лексикографически по парам (col,
// row), а также начальный и конечный фонтаны. Возвращает индексы фонтанов, которые на отрезке от
// first до last включительно являются поворотными. При этом фонтан с номером skip не считается
// поворотным и пропускается. 
fn get_pivots(fnts: &[Fontain], first: us, last: us, skip: us) -> Vec<us> {
    let mut pivots = vec![];

    // Предполагаем, что первый фонтан не будут скипать.
    // k - последний увиденный фонтан в данном столбце, он же -- кандидат на то чтобы быть
    // поворотным. Если он последний в столбце, то он поворотный, так как фонтаны отсортированы по
    // номерам столбцов, а в данном столбце -- по строкам.
    let mut k = first;

    // Перебираем все фонтаны, кроме skip.
    for i in (first + 1..=last).filter(|&x| x != skip) {
        // Если новый фонтан ниже фонтана k, то он нам интересен, может оказаться новым поворотным.
        if fnts[i].row > fnts[k].row {
            // Если он еще и правее, то точно поворотный.
            if fnts[i].col > fnts[k].col {
                pivots.push(k);
            }
            // Фонтан i ниже k и либо в этом столбце, либо правее. Запоминаем его как нового
            // кандидата на поворотный фонтан.
            k = i;
        }
    }

    // k -- самый нижний фонтан в своем столбце, он поворотный.
    pivots.push(k);

    pivots
}

// Вычисляет площадь Алисы по строкам от первого до последнего поворотного фонтана на отрезке
// [first, last] массива fnts. На рисунке отмечены строки, которые входят в площадь между двумя
// поворотными фонтанами (обозначены P):
// ...P....      ...P....
// ........      xxxxxx..
// ........  ->  xxxxxx..
// ......P.      xxxxxxP.
// ........      ........
// До есть площадь между двумя поворотными фонтанами считается со следующей строки после первого
// до строки с последним фонтаном включительно.
fn get_area(fnts: &[Fontain], first: us, last: us, skip: us) -> us {
    let pivots = get_pivots(fnts, first, last, skip);
    // Итератор, порождающий последовательные пары поворотных фонтанов.
    let iter = pivots
        .iter()
        .zip(pivots.iter().skip(1))
        .map(|(&i, &j)| (&fnts[i], &fnts[j]));

    let mut s = 0;
    for (f1, f2) in iter {
        s += (f2.row - f1.row) * (f2.col - 1);
    }
    s
}

fn solve() {
    let (n, m, k) = inp::read3::<us>();
    let mut fnts: Vec<Fontain> = Vec::new();

    // Значение, которое не совпадает ни с одним индексом, чтобы не скипать фонтаны.
    let fantom = k + 2;

    for i in 0..k {
        let (row, col) = inp::read2::<us>();
        fnts.push(Fontain {
            col,
            row,
            idx: i,
            result: 0,
        });
    }

    // Добавляем два фиктивных фонтана (слева сверху и справа снизу), заведомо поворотных.
    fnts.push(Fontain {
        row: 0,
        col: 0,
        idx: k,
        result: 0,
    });
    fnts.push(Fontain {
        row: n + 1,
        col: m + 1,
        idx: k + 1,
        result: 0,
    });
    fnts.sort_unstable();

    // Вычисляем площадь, при условии, что Алисе не подарили фонтаны. Второй фиктивный фонтан
    // вносит лишний вклад -- m, убираем его.
    let area = get_area(&fnts, 0, fnts.len() - 1, fantom) - m;
    println!("{}", area);

    // Находим все поворотные фонтаны, среди них фиктивные находятся на перовом и последнем местах.
    let pivots_idx = get_pivots(&fnts, 0, fnts.len() - 1, fantom);

    // Перебираем все последовательные тройки фонтанов.
    for i in 1..pivots_idx.len() - 1 {
        let cur = pivots_idx[i];
        let prev = pivots_idx[i - 1];
        let next = pivots_idx[i + 1];
        // Считаем площадь со скипом среднего фонтана и без скипа.
        fnts[cur].result = get_area(&fnts, prev, next, cur) - get_area(&fnts, prev, next, fantom);
    }

    // Восстанавливаем порядок фонтанов. Добавленные фонтаны окажутся последними.
    fnts.sort_unstable_by_key(|f| f.idx);

    // Выводим результат для первых k фонтанов.
    for f in fnts.iter().take(k) {
        print!("{} ", f.result);
    }
    println!();
}

fn main() {
    for _ in 0..inp::read1() {
        solve();
    }
}

/* Auxiliary stuff */
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
