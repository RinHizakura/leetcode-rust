use std::collections::HashMap;

struct Solution;
impl Solution {
    fn dis(dis_map: &mut Vec<Vec<i32>>, i: usize, j: usize, p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
        if dis_map[i][j] == 0 {
            let d1 = p1[0] - p2[0];
            let d2 = p1[1] - p2[1];
            dis_map[i][j] = d1 * d1 + d2 * d2;
        }
        return dis_map[i][j];
    }

    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let len = points.len();

        let mut map = HashMap::new();
        let mut dis_map = vec![vec![0; len]; len];
        let mut ans = 0;
        for i in 0..len {
            map.clear();

            for j in 0..len {
                let d = Solution::dis(&mut dis_map, i, j, &points[i], &points[j]);

                if let Some(cnt) = map.get_mut(&d) {
                    *cnt += 1;
                } else {
                    map.insert(d, 1);
                }
            }

            for (_, val) in map.iter() {
                if *val >= 2 {
                    ans += val * (val - 1);
                }
            }
        }

        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]])
    );
}
