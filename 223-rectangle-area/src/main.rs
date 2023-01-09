struct Solution;
impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let overlap;
        if ay1 >= by2 || by1 >= ay2 || ax1 >= bx2 || bx1 >= ax2 {
            overlap = 0;
        } else {
            let y2 = ay2.min(by2);
            let y1 = ay1.max(by1);

            let x2 = ax2.min(bx2);
            let x1 = ax1.max(bx1);

            overlap = (x2 - x1) * (y2 - y1);
        }
        let a_area = (ax2 - ax1) * (ay2 - ay1);
        let b_area = (bx2 - bx1) * (by2 - by1);

        a_area + b_area - overlap
    }
}

fn main() {
    println!("{}", Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2));
}
