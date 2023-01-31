struct Solution;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut k = Vec::new();
        k.push(1);

        let mut prod_2 = 0;
        let mut prod_3 = 0;
        let mut prod_5 = 0;

        let mut count = 1;

        while count < n {
            let a = k[prod_2] * 2;
            let b = k[prod_3] * 3;
            let c = k[prod_5] * 5;
            let next_ugly = a.min(b).min(c);
            k.push(next_ugly);

            if next_ugly == a {
                prod_2 += 1;
            }

            if next_ugly == b {
                prod_3 += 1;
            }

            if next_ugly == c {
                prod_5 += 1;
            }
            count += 1;
        }

        k[n as usize - 1]
    }
}

fn main() {
    println!("{}", Solution::nth_ugly_number(10));
}
