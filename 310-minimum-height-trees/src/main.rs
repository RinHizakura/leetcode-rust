use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut degrees = vec![0; n];
        let mut adj = vec![Vec::new(); n];

        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            adj[a].push(b);
            adj[b].push(a);
            degrees[a] += 1;
            degrees[b] += 1;
        }

        let mut queue = VecDeque::new();
        for (idx, d) in degrees.iter().enumerate() {
            if *d == 1 {
                queue.push_back(idx);
            }
        }
        let mut ans = Vec::new();

        while !queue.is_empty() {
            ans.clear();

            let sz = queue.len();
            for _ in 0..sz {
                if let Some(num) = queue.pop_front() {
                    ans.push(num as i32);
                    for neighbor in &adj[num as usize] {
                        degrees[*neighbor] -= 1;
                        if degrees[*neighbor] == 1 {
                            queue.push_back(*neighbor);
                        }
                    }
                }
            }
        }

        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]])
    );
}
