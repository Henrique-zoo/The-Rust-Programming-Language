use std::collections::VecDeque;

struct TwoSATSolver {
    n: usize,
    graph: Vec<Vec<usize>>,
    transposed: Vec<Vec<usize>>,
}

impl TwoSATSolver {
    fn new(num_vars: usize) -> Self {
        let num_nodes = 2 * num_vars;
        TwoSATSolver {
            n: num_vars,
            graph: vec![Vec::new(); num_nodes],
            transposed: vec![Vec::new(); num_nodes],
        }
    }

    // Adiciona uma cláusula (a ∨ b) ao solver
    fn add_clause(&mut self, a: bool, a_var: usize, b: bool, b_var: usize) {
        // Mapeia variáveis booleanas para literais
        let literal_a = if a { 2 * a_var } else { 2 * a_var + 1 };
        let literal_b = if b { 2 * b_var } else { 2 * b_var + 1 };
        let not_a = literal_a ^ 1;
        let not_b = literal_b ^ 1;

        // Adiciona arestas de implicação: (¬a → b) e (¬b → a)
        self.graph[not_a].push(literal_b);
        self.graph[not_b].push(literal_a);
        self.transposed[literal_b].push(not_a);
        self.transposed[literal_a].push(not_b);
    }

    // Resolve o problema 2-SAT
    fn solve(&self) -> Option<Vec<bool>> {
        let (order, comp) = self.kosaraju();
        
        // Verifica se alguma variável e sua negação estão na mesma SCC
        for i in 0..self.n {
            if comp[2 * i] == comp[2 * i + 1] {
                return None;
            }
        }

        // Constrói a atribuição de valores
        let mut assignment = vec![false; self.n];
        for i in 0..self.n {
            assignment[i] = comp[2 * i] > comp[2 * i + 1];
        }
        Some(assignment)
    }

    // Algoritmo de Kosaraju para encontrar SCCs
    fn kosaraju(&self) -> (Vec<usize>, Vec<usize>) {
        let n = self.graph.len();
        let mut visited = vec![false; n];
        let mut order = Vec::with_capacity(n);
        let mut stack = Vec::new();

        // Primeira DFS: ordenação por tempo de finalização
        for i in 0..n {
            if !visited[i] {
                stack.push(i);
                visited[i] = true;
                
                while let Some(&u) = stack.last() {
                    let mut pushed = false;
                    for &v in &self.graph[u] {
                        if !visited[v] {
                            visited[v] = true;
                            stack.push(v);
                            pushed = true;
                            break;
                        }
                    }
                    if !pushed {
                        order.push(stack.pop().unwrap());
                    }
                }
            }
        }

        // Segunda DFS: componente fortemente conexas
        let mut comp = vec![0; n];
        let mut comp_id = 0;
        visited = vec![false; n];
        
        for &u in order.iter().rev() {
            if !visited[u] {
                let mut stack = VecDeque::new();
                stack.push_back(u);
                visited[u] = true;
                comp[u] = comp_id;
                
                while let Some(u) = stack.pop_back() {
                    for &v in &self.transposed[u] {
                        if !visited[v] {
                            visited[v] = true;
                            comp[v] = comp_id;
                            stack.push_back(v);
                        }
                    }
                }
                comp_id += 1;
            }
        }
        (order, comp)
    }
}

fn main() {
    // Exemplo: (x0 ∨ x1) ∧ (¬x0 ∨ x1)
    let mut solver = TwoSATSolver::new(2);
    
    // Cláusula 1: (x0 ∨ x1)
    solver.add_clause(true, 0, true, 1);
    
    // Cláusula 2: (¬x0 ∨ x1)
    solver.add_clause(false, 0, true, 1);
    
    if let Some(assignment) = solver.solve() {
        println!("Satisfatível! Atribuição: {:?}", assignment);
        // Output esperado: [false, true] (x0=false, x1=true)
    } else {
        println!("Insatisfatível!");
    }
}