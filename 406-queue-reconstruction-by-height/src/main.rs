struct Solution;
impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        /* In the first round, put the tallest people in the front. If
         * two people get same height, then the one who need less people
         * to get front of him/her go first. */
        people.sort_by_key(|p| (-p[0], p[1]));

        /* In the second round, insert the entry to the right place. This
         * sort works because of the assumption that the answer should always
         * exist */
        let len = people.len();
        let mut result = Vec::with_capacity(len);
        result.push(people[0].clone());

        for i in 1..len {
            result.insert(people[i][1] as usize, people[i].clone());
        }

        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::reconstruct_queue(vec![vec![8, 0], vec![7, 0]])
    );
}
