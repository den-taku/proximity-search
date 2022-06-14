// struct Graph {
//     vertices: usize,
//     adjacent_list: Vec<Vec<usize>>,
// }

// #[derive(Debug, Clone, Eq, PartialEq)]
// struct SubGraph {
//     vertices: std::collections::HashSet<usize>,
// }

// impl std::fmt::Display for SubGraph {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{:?}", self.vertices)
//     }
// }

// impl Graph {
//     fn new(vertices: usize, adjacent_list: Vec<Vec<usize>>) -> Self {
//         Self {
//             vertices,
//             adjacent_list,
//         }
//     }
// }

// /// Graph B is bipartite subgraph ⇔
// /// B allows a bipartition of its vertices B0, B1 s.t. B0 ∩ B1 = ∅, B0 ∪ B1 ⊆ V(G), both G[B0] and G[B1] are empty graphs
// pub struct MaximalConnectedInducedBipartileSubgraph {
//     graph: Graph,
// }

// impl MaximalConnectedInducedBipartileSubgraph {
//     pub fn new(vertices: usize, adjacent_list: Vec<Vec<usize>>) -> Self {
//         Self {
//             graph: Graph::new(vertices, adjacent_list),
//         }
//     }
// }
