use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;

        /* After taking class 'i', you complete one of the prerequisites
         * of class check[i][0], check[i][1], ... check[i][j] */
        let mut check: Vec<Vec<usize>> = vec![vec![]; num_courses];
        /* To take class 'i', there're wait[i] numbers of class
         * you have to done */
        let mut wait = vec![0; num_courses];

        for pre in prerequisites {
            check[pre[1] as usize].push(pre[0] as usize);
            wait[pre[0] as usize] += 1;
        }

        let mut ans = vec![];
        let mut queue: VecDeque<_> = (0..num_courses)
            .filter(|c| wait[*c as usize] == 0)
            .collect();
        while let Some(idx) = queue.pop_front() {
            ans.push(idx as i32);
            wait[idx] -= 1;

            for class in &check[idx] {
                wait[*class] -= 1;
                if wait[*class] == 0 {
                    queue.push_back(*class);
                }
            }
        }

        if ans.len() == num_courses {
            ans
        } else {
            vec![]
        }
    }
}
fn main() {
    println!("Hello, world!");
}
