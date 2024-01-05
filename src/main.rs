#![allow(dead_code)]

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    collections::{HashMap, LinkedList},
    sync::mpsc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum T {
    A(usize, usize),
    B(usize, usize, usize),
    C(usize, usize, usize),
}

fn main() {
    let n = 3;
    let mut m = vec![
        vec![2., 1., 3., 6.],
        vec![4., 3., 8., 15.],
        vec![6., 5., 16., 27.],
    ];

    if n < 2 {
        return;
    }

    // alfabet
    let mut alphabet: Vec<T> = vec![];

    // relacja zależności
    let mut d: Vec<(T, T)> = vec![];

    // graf Diekerta
    let mut g: HashMap<T, Vec<T>> = HashMap::new();

    for i in 0..n - 1 {
        for k in i + 1..n {
            let va: Vec<T> = (i..n + 1).map(|j| T::B(i, j, k)).collect();
            let a = T::A(i, k);

            for t in &va {
                d.push((a, *t));
                d.push((*t, a));
            }

            alphabet.push(a);
            d.push((a, a));
            g.insert(a, va);

            for j in i..n + 1 {
                let vb = vec![T::C(i, j, k)];
                let b = T::B(i, j, k);

                for t in &vb {
                    d.push((b, *t));
                    d.push((*t, b));
                }

                alphabet.push(b);
                d.push((b, b));
                g.insert(b, vb);
            }

            let vc = vec![];
            let c = T::C(i, i, k);

            alphabet.push(c);
            d.push((c, c));
            g.insert(c, vc);

            if k == n - 1 && i == k - 1 {
                for j in i + 1..n + 1 {
                    let vc = vec![];
                    let c = T::C(i, j, k);

                    alphabet.push(c);
                    d.push((c, c));
                    g.insert(c, vc);
                }
            } else {
                let vc = match i + 1 {
                    k if k == i + 1 => (k + 1..n).map(|x| T::A(k, x)).collect(),
                    _ => vec![T::A(i + 1, k)],
                };

                let c = T::C(i, i + 1, k);

                for t in &vc {
                    d.push((c, *t));
                    d.push((*t, c));
                }

                alphabet.push(c);
                d.push((c, c));
                g.insert(c, vc);

                for j in i + 2..n + 1 {
                    let mut vc = vec![];

                    if k + 1 < n {
                        vc.push(T::B(i + 1, j, k + 1));
                    }
                    if k > i + 1 {
                        vc.push(T::C(i + 1, j, k));
                    }

                    let c = T::C(i, j, k);

                    for t in &vc {
                        d.push((c, *t));
                        d.push((*t, c));
                    }

                    alphabet.push(c);
                    d.push((c, c));
                    g.insert(c, vc);
                }
            }
        }
    }

    let l_a = alphabet.len();
    let task_index: HashMap<T, usize> = HashMap::from_iter((0..l_a).map(|i| (alphabet[i], i)));

    let mut stack: LinkedList<T> = LinkedList::new();
    let mut max_paths = vec![0; l_a];

    // wyznaczanie klas do FNF zmodyfikowanym bfsem
    for k in 1..n {
        stack.push_back(T::A(0, k));
    }

    while !stack.is_empty() {
        let task = stack.pop_front().unwrap();
        let v = task_index[&task];
        let curr_path = max_paths[v];

        let dependant_tasks = &g[&task];

        for dt in dependant_tasks {
            let u = task_index[dt];
            max_paths[u] = usize::max(curr_path + 1, max_paths[u]);
            stack.push_back(*dt);
        }
    }

    let iterations = max_paths.iter().max().unwrap() + 1;
    alphabet.sort_by_key(|t| max_paths[task_index[t]]);

    let mut calc_a = vec![vec![0.0; n]; n];
    let mut calc_b = vec![vec![vec![0.0; n]; n + 1]; n];

    let mut groups: Vec<&[T]> = vec![];
    let mut i = 0;

    for it in 0..=iterations {
        let start = i;

        while i < l_a && it == max_paths[task_index[&alphabet[i]]] {
            i += 1;
        }
        let end = i;

        groups.push(&alphabet[start..end]);
    }

    for group in groups {
        let (send, recv) = mpsc::channel();
        let l_g = group.len();

        group.into_par_iter().for_each(|t| {
            let to_send = match t {
                T::A(i, k) => (t, m[*k][*i] / m[*i][*i]),
                T::B(i, j, k) => (t, m[*i][*j] * calc_a[*k][*i]),
                T::C(i, j, k) => (t, m[*k][*j] - calc_b[*k][*j][*i]),
            };
            send.send(to_send).unwrap();
        });

        for _ in 0..l_g {
            let received = recv.recv().unwrap();

            match received {
                (T::A(i, k), x) => {
                    calc_a[*k][*i] = x;
                }
                (T::B(i, j, k), x) => {
                    calc_b[*k][*j][*i] = x;
                }
                (T::C(_i, j, k), x) => {
                    m[*k][*j] = x;
                }
            }
        }
    }

    println!("{m:#?}");
}
