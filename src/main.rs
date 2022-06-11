/// Given
/// - a set system (U, S), where S is a family of solutions that is to be enemurated.
/// - a set system (U, C), where C is a family of (not necessarily maximal) solutions.
///
/// Input
/// - Universe U
///
/// Output
/// - all solutions S
trait ProsimitySearchable {
    /// a set of elements, e.g., V(G)
    type Universe;
    /// (not necessarily maximal) solutions that meets property at hand.
    type Components;
    /// maximul solutions that is a subset of the universe and meets property at hand.
    type Solutions: Eq + std::hash::Hash + std::fmt::Display + Clone;
    /// not actually used in algorithm, but needed to prove the correctness.
    /// proximity: Self::Solutions x Self::Solutions -> 2^Universe
    fn proximity(
        _solution: Self::Solutions,
        _solution_: Self::Solutions,
    ) -> std::collections::HashSet<usize> {
        std::collections::HashSet::new()
    }
    /// 1. computable in time polynomial in |U|.
    /// 2. For all S, S* in Self::Solutions, there exists S' in neighbors(S) s.t. |proximity(S', S*)| > |proximity(S, S*)|
    /// 3. For any fixed S*, |proximity(S, S*)| is maximized for (and only for) S = S*
    fn neighbors(&self, solution: Self::Solutions) -> Vec<Self::Solutions>;
    /// one solution of the problem is to be idintified in time polynomial in |U|.
    fn start(&self) -> Self::Solutions;
    /// enemurate all solutions
    fn enemurate(&self) -> std::collections::HashSet<Self::Solutions> {
        let mut solutions = std::collections::HashSet::new();
        let first_solution = self.start();
        self.enume(first_solution, &mut solutions, 0);
        solutions
    }

    fn enume(
        &self,
        solution: Self::Solutions,
        solutions: &mut std::collections::HashSet<Self::Solutions>,
        deps: usize,
    ) {
        solutions.insert(solution.clone());
        if deps % 2 == 0 {
            println!("{solution}");
            for s in self.neighbors(solution) {
                if !solutions.contains(&s) {
                    self.enume(s, solutions, deps + 1)
                }
            }
        } else if deps % 2 != 0 {
            for s in self.neighbors(solution.clone()) {
                if !solutions.contains(&s) {
                    self.enume(s, solutions, deps + 1)
                }
            }
            println!("{solution}");
        }
    }
}

fn main() {
    println!("Hello, world!");
}
