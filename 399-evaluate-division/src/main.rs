struct Solution;
use std::collections::HashMap;

struct Evaluator {
    symtab: HashMap<String, usize>,
    reverse_map: Vec<String>,
    index: usize,
}

impl Evaluator {
    fn add_symbol(&mut self, s: &str) {
        self.reverse_map.push(s.to_string());
        self.symtab.insert(s.to_string(), self.index);
        self.index += 1;
    }

    pub fn new() -> Self {
        Evaluator {
            symtab: HashMap::new(),
            reverse_map: Vec::new(),
            index: 0,
        }
    }

    pub fn init(&mut self, equations: &Vec<Vec<String>>) {
        // In the first pass, we decide how many differrent variables are there
        for e in equations {
            if self.symtab.get(&e[0]).is_none() {
                self.add_symbol(&e[0]);
            }

            if self.symtab.get(&e[1]).is_none() {
                self.add_symbol(&e[1]);
            }
        }
    }

    fn dfs(
        current: usize,
        target: usize,
        map: &Vec<Vec<f64>>,
        visited: &mut Vec<bool>,
        distance: f64,
    ) -> Result<f64, ()> {
        /* This is a special DFS designed for this question. It will
         * finishes early if the target node is found */
        if map[current][target] != 0.0 {
            return Ok(distance * map[current][target]);
        }
        visited[current] = true;

        for (idx, &next_d) in map[current].iter().enumerate() {
            if next_d != 0.0 && visited[idx] == false {
                if let Ok(result) = Self::dfs(idx, target, map, visited, distance * next_d) {
                    return Ok(result);
                }
            }
        }

        Err(())
    }

    pub fn eval(
        &mut self,
        equations: &Vec<Vec<String>>,
        values: &Vec<f64>,
        queries: &Vec<Vec<String>>,
    ) -> Vec<f64> {
        // In the second pass, we build the map
        let len = self.index;
        let mut map = vec![vec![0.0; len]; len];
        for (idx, e) in equations.into_iter().enumerate() {
            let e_0 = *self.symtab.get(&e[0]).unwrap();
            let e_1 = *self.symtab.get(&e[1]).unwrap();

            // The value of map[e_0][e_1] is e_0 / e_1
            map[e_0][e_1] = values[idx];
            map[e_1][e_0] = 1.0 / values[idx];
        }

        let mut ans = vec![];
        for q in queries {
            let q_0 = self.symtab.get(&q[0]);
            let q_1 = self.symtab.get(&q[1]);

            if q_0.is_none() || q_1.is_none() {
                ans.push(-1.0);
                continue;
            }

            let q_0 = *q_0.unwrap();
            let q_1 = *q_1.unwrap();
            let mut visited = vec![false; len];
            if let Ok(result) = Self::dfs(q_0, q_1, &map, &mut visited, 1.0) {
                ans.push(result);
            } else {
                ans.push(-1.0);
            }
        }
        ans
    }
}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut evaluator = Evaluator::new();
        evaluator.init(&equations);
        evaluator.eval(&equations, &values, &queries)
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::calc_equation(
            vec![vec!["a".to_string(), "b".to_string()]],
            vec![1.0],
            vec![vec!["a".to_string(), "b".to_string()]]
        )
    );
}
