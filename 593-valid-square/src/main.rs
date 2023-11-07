struct Solution;
impl Solution {
    fn edge_len(p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
        let a = p1[0] - p2[0];
        let b = p1[1] - p2[1];
        a * a + b * b
    }

    fn check(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>, p4: &Vec<i32>) -> bool {
        let v1 = vec![p2[0] - p1[0], p2[1] - p1[1]];
        let v2 = vec![p3[0] - p1[0], p3[1] - p1[1]];

        if p1[0] + v1[0] + v2[0] == p4[0] && p1[1] + v1[1] + v2[1] == p4[1] {
            return true;
        }
        false
    }

    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        /* Handle edge case */
        if p1[0] == 0
            && p1[1] == 0
            && p2[0] == 0
            && p2[1] == 0
            && p3[0] == 0
            && p3[1] == 0
            && p4[0] == 0
            && p4[1] == 0
        {
            return false;
        }

        /* Pick the first three point, check if there're
         * two edges with same length */
        let e1 = Self::edge_len(&p1, &p2);
        let e2 = Self::edge_len(&p2, &p3);
        let e3 = Self::edge_len(&p1, &p3);

        if e1 == e2 && (2 * e1 == e3) {
            return Self::check(&p2, &p1, &p3, &p4);
        }

        if e1 == e3 && (2 * e1 == e2) {
            return Self::check(&p1, &p2, &p3, &p4);
        }

        if e2 == e3 && (2 * e2 == e1) {
            return Self::check(&p3, &p1, &p2, &p4);
        }

        return false;
    }
}

fn main() {
    println!(
        "{}",
        Solution::valid_square(vec![0, 0], vec![1, 0], vec![0, 1], vec![1, 1])
    );
}
