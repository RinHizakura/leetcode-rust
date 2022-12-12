struct Solution;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;

        /* After taking class 'i', you complete one of the prerequisites
         * of class check[i][0], check[i][1], ... check[i][j] */
        let mut check: Vec<Vec<usize>> = vec![vec![]; num_courses];
        /* To take class 'i', there're pre_cnt[i] numbers of class
         * you have to done */
        let mut pre_cnt: Vec<i32> = vec![0; num_courses];

        for p in prerequisites {
            check[p[1] as usize].push(p[0] as usize);
            pre_cnt[p[0] as usize] += 1;
        }

        let mut total = num_courses;
        while total != 0 {
            if let Some(idx) = pre_cnt.iter().position(|&x| x == 0) {
                pre_cnt[idx] -= 1;

                for class in &check[idx] {
                    pre_cnt[*class] -= 1;
                }
                total -= 1;
            } else {
                return false;
            }
        }

        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::can_finish(
            20,
            vec![
                vec![0, 10],
                vec![3, 18],
                vec![5, 5],
                vec![6, 11],
                vec![11, 14],
                vec![13, 1],
                vec![15, 1],
                vec![17, 4]
            ]
        )
    );
}
