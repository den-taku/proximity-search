#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

struct Graph {
    vertices: usize,
    edges: HashSet<(usize, usize)>,
}

pub struct MaximalConnectedInducedBipartiteSubgraph {
    graph: Graph,
    pub solutions: HashSet<Vec<usize>>,
    pub index: HashMap<Vec<usize>, usize>,
    pub edges: Vec<(usize, usize)>,
}

impl MaximalConnectedInducedBipartiteSubgraph {
    pub fn init(vertices: usize, edges: HashSet<(usize, usize)>) -> Self {
        Self {
            graph: Graph { vertices, edges },
            solutions: HashSet::new(),
            index: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let first_solution = self.comp(HashSet::new());
        self.index
            .insert(set_to_vec(&first_solution), self.solutions.len());
        println!("maximal: {:?}", print_vec(&set_to_vec(&first_solution)));
        self.enume(first_solution, 1);
    }

    fn enume(&mut self, solution: HashSet<usize>, deps: usize) {
        let solution_vec = set_to_vec(&solution);
        let u = *self.index.get(&solution_vec).unwrap();
        self.solutions.insert(solution_vec.clone());
        // For archieve polynomial delay, use `alternative output`

        // if deps % 2 == 0 {
        // println!("maximal: {:?}", print_vec(&solution_vec));
        // }

        for s in self.neighbors(solution) {
            let s_vec = set_to_vec(&s);
            if !self.solutions.contains(&s_vec) {
                println!("maximal: {:?}", print_vec(&s_vec));
                let v = self.solutions.len();
                self.index.insert(s_vec, v);
                self.edges.push((u, v));
                self.enume(s, deps + 1);
            } else {
                let v = *self.index.get(&s_vec).unwrap();
                self.edges.push((u, v));
                println!("duplicated: {:?}", print_vec(&s_vec));
            }
        }

        // if deps % 2 == 1 {
        //     println!("maximal: {:?}", print_vec(&solution_vec));
        // }
    }

    fn neighbors(&self, solution: HashSet<usize>) -> Vec<HashSet<usize>> {
        let (b_0, b_1) = self.bipartition(&solution);

        let mut neighbors = Vec::new();
        for v in 0..self.graph.vertices {
            if solution.contains(&v) {
                continue;
            }
            let n_v = (0..self.graph.vertices)
                .map(|i| self.graph.edges.contains(&(i, v)) || self.graph.edges.contains(&(v, i)))
                .enumerate()
                .filter(|e| e.1)
                .map(|e| e.0)
                .collect::<HashSet<_>>();
            // B_0 U (B_1 \ N(v))
            let comp1 = {
                let mut comp1 = b_0
                    .union(&b_1.difference(&n_v).copied().collect::<HashSet<_>>())
                    .copied()
                    .collect::<HashSet<_>>();
                comp1.insert(v);
                comp1
            };
            print!("{}: ", v + 1);
            neighbors.push(self.comp(self.cc(&comp1, v)));
            // (B_0 \ N(v)) U B_1
            let comp2 = {
                let mut comp2 = b_1
                    .union(&b_0.difference(&n_v).copied().collect::<HashSet<_>>())
                    .copied()
                    .collect::<HashSet<_>>();
                comp2.insert(v);
                comp2
            };
            print!("{}: ", v + 1);
            neighbors.push(self.comp(self.cc(&comp2, v)));
        }
        neighbors
    }

    fn bipartition(&self, solution: &HashSet<usize>) -> (HashSet<usize>, HashSet<usize>) {
        let solution_vec = set_to_vec(&solution);
        let mut visited = solution_vec.iter().map(|_| None).collect::<Vec<_>>();

        let mut b_0 = HashSet::new();
        let mut b_1 = HashSet::new();

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, solution_vec[0]));
        b_0.insert(solution_vec[0]);
        visited[0] = Some(0);

        while let Some((i, v)) = queue.pop_front() {
            let now = visited[i].unwrap();
            let next = 1 - now;
            for (j, &u) in solution_vec.iter().enumerate() {
                if visited[j].is_none()
                    && (self.graph.edges.contains(&(v, u)) || self.graph.edges.contains(&(u, v)))
                {
                    visited[j] = Some(next);
                    if next == 0 {
                        b_0.insert(u);
                    } else {
                        b_1.insert(u);
                    }
                    queue.push_back((j, u));
                }
            }
        }
        println!(
            "b_0: {:?}",
            set_to_vec(&b_0.iter().copied().map(|e| e + 1).collect::<HashSet<_>>())
        );
        println!(
            "b_1: {:?}",
            set_to_vec(&b_1.iter().copied().map(|e| e + 1).collect::<HashSet<_>>())
        );
        (b_0, b_1)
    }

    fn cc(&self, set: &HashSet<usize>, v: usize) -> HashSet<usize> {
        if set.is_empty() {
            return HashSet::new();
        }
        let mut new = HashSet::new();
        let set_vec = set_to_vec(&set);
        let mut visited = set_vec.iter().map(|_| false).collect::<Vec<_>>();

        let i = set_vec.iter().position(|&e| e == v).unwrap();
        visited[i] = true;
        new.insert(v);
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(v);

        while let Some(v) = queue.pop_front() {
            for (j, &u) in set_vec.iter().enumerate() {
                if !visited[j]
                    && (self.graph.edges.contains(&(v, u)) || self.graph.edges.contains(&(u, v)))
                {
                    visited[j] = true;
                    new.insert(u);
                    queue.push_back(u);
                }
            }
        }

        new
    }

    fn comp(&self, mut component: HashSet<usize>) -> HashSet<usize> {
        print!("component: {:?}", print_vec(&set_to_vec(&component)));
        let mut n = 0;
        while n < self.graph.vertices {
            if component.contains(&n) {
                n += 1;
                continue;
            }
            component.insert(n);
            if self.is_bipartite(&component) && self.is_connected(&component) {
                n = 0;
            } else {
                component.remove(&n);
                n += 1;
            }
        }
        println!("→ {:?}", print_vec(&set_to_vec(&component)));
        component
    }

    fn is_bipartite(&self, set: &HashSet<usize>) -> bool {
        use union_find_library::UnionFind;
        let mut tree = UnionFind::new(self.graph.vertices * 2);
        for &u in set {
            for &v in set {
                if self.graph.edges.contains(&(u, v)) || self.graph.edges.contains(&(v, u)) {
                    tree.unite(u, v + self.graph.vertices);
                    tree.unite(u + self.graph.vertices, v);
                }
            }
        }
        (0..self.graph.vertices).all(|i| !(tree.same(i, i + self.graph.vertices)))
    }

    fn is_connected(&self, set: &HashSet<usize>) -> bool {
        if set.is_empty() {
            return true;
        }
        let set_vec = set_to_vec(&set);
        let mut visited = set_vec.iter().map(|_| false).collect::<Vec<_>>();

        visited[0] = true;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(set_vec[0]);

        while let Some(v) = queue.pop_front() {
            for (j, &u) in set_vec.iter().enumerate() {
                if !visited[j]
                    && (self.graph.edges.contains(&(v, u)) || self.graph.edges.contains(&(u, v)))
                {
                    visited[j] = true;
                    queue.push_back(u);
                }
            }
        }
        visited.iter().all(|&e| e)
    }
}

fn print_vec(vec: &Vec<usize>) -> Vec<usize> {
    vec.iter().copied().map(|e| e + 1).collect()
}

fn set_to_vec(set: &HashSet<usize>) -> Vec<usize> {
    let mut vec = set.iter().copied().collect::<Vec<_>>();
    vec.sort();
    vec
}

/// Disjoint Set
///
/// new
/// unite
/// find
/// same
/// count
/// to_vec
pub mod union_find_library {
    /// Union-Find Tree, that treats disjoint sets efficiently.
    /// verified by this(https://atcoder.jp/contests/atc001/submissions/24929276).
    /// and (https://atcoder.jp/contests/abc214/submissions/26399785)
    pub struct UnionFind {
        par: Vec<usize>,
        rank: Vec<usize>,
        count: Vec<usize>,
    }

    impl UnionFind {
        #[inline(always)]
        /// Create a new Union-Find Tree contains n elements.
        /// At the first state, the elements are mutually disjoint.
        pub fn new(n: usize) -> Self {
            UnionFind {
                par: (0..n).collect(),
                rank: vec![0; n],
                count: vec![1; n],
            }
        }

        #[inline(always)]
        /// Return representative representing set containing x.
        pub fn find(&mut self, x: usize) -> usize {
            if x >= self.par.len() {
                panic!("out of bound.")
            }
            unsafe {
                if *self.par.get_unchecked(x) == x {
                    x
                } else {
                    let mut represent = x;
                    while {
                        represent = *self.par.get_unchecked(represent);
                        *self.par.get_unchecked(represent) != represent
                    } {}
                    *self.par.get_unchecked_mut(x) = represent;
                    represent
                }
            }
        }

        #[inline(always)]
        /// Unite 2 sets, one containing x and the other containing y.
        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            let x_par = self.find(x);
            let y_par = self.find(y);
            if x_par != y_par {
                unsafe {
                    if *self.rank.get_unchecked(x_par) < *self.rank.get_unchecked(y_par) {
                        *self.par.get_unchecked_mut(x_par) = y_par;
                        *self.count.get_unchecked_mut(y_par) += *self.count.get_unchecked(x_par);
                    } else {
                        *self.par.get_unchecked_mut(y_par) = x_par;
                        *self.count.get_unchecked_mut(x_par) += *self.count.get_unchecked(y_par);
                        if *self.rank.get_unchecked(x_par) == *self.rank.get_unchecked(y_par) {
                            self.rank[x_par] += 1;
                        }
                    }
                }
            }
            x_par != y_par
        }

        #[inline(always)]
        /// Decide whether set, containing x, contains y or not.
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }

        #[inline(always)]
        /// Convert UnionFind to Vec\<Vec\<usize\>\>
        pub fn to_vec(&mut self) -> Vec<Vec<usize>> {
            let mut set = vec![Vec::new(); self.par.len()];
            for i in 0..self.par.len() {
                unsafe {
                    set.get_unchecked_mut(self.find(i)).push(i);
                }
            }
            set.into_iter().filter(|s| !s.is_empty()).collect()
        }

        #[inline(always)]
        /// count connected component's size of x
        pub fn count(&mut self, x: usize) -> usize {
            let x_par = self.find(x);
            unsafe { *self.count.get_unchecked(x_par) }
        }
    }

    #[cfg(test)]
    mod tests_union_find {
        use super::*;

        #[test]
        fn for_union_find() {
            let queries = vec![
                (0, 1, 2),
                (0, 3, 2),
                (1, 1, 3),
                (1, 1, 4),
                (0, 2, 4),
                (1, 4, 1),
                (0, 4, 2),
                (0, 0, 0),
                (1, 0, 0),
            ];
            let ans = vec![true, false, true, true];
            let n = 8;
            let mut uf_tree = UnionFind::new(n);
            let mut index = 0;
            for &(i, x, y) in queries.iter() {
                if i == 0 {
                    uf_tree.unite(x, y);
                } else {
                    assert_eq!(uf_tree.same(x, y), ans[index]);
                    index += 1;
                }
            }
        }

        #[test]
        fn for_count_uftree() {
            let n = 5;
            let edges = vec![(1, 2, 1), (2, 3, 2), (4, 2, 5), (3, 5, 14)];
            let mut tree = UnionFind::new(n);
            let mut value = 0;
            for (u, v, w) in edges {
                value += w * tree.count(u - 1) * tree.count(v - 1);
                tree.unite(u - 1, v - 1);
            }
            assert_eq!(value, 76);
        }
    }
}
