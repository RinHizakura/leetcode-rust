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
        for i in 1..len {
            let h = people[i][0];
            let f = people[i][1];
            let mut front = people[i][1];

            let mut j = 0;
            while front > 0 {
                if people[j][0] >= h {
                    front -= 1;
                }
                j += 1;
            }
            /* Until here, j is the place we should insert */

            for idx in (j..i).rev() {
                people[idx + 1][0] = people[idx][0];
                people[idx + 1][1] = people[idx][1];
            }

            people[j][0] = h;
            people[j][1] = f;
        }

        people
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::reconstruct_queue(vec![vec![8, 0], vec![7, 0]])
    );
}
