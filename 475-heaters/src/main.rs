struct Solution;
impl Solution {
    pub fn find_radius(houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let m = heaters.len();

        heaters.sort();

        for house in houses {
            let pos = heaters.binary_search(&house);

            let dis = if pos.is_ok() {
                0
            } else {
                let pos = pos.unwrap_err();
                let tmp1 = if pos < m {
                    heaters[pos] - house
                } else {
                    i32::MAX
                };
                let tmp2 = if pos > 0 {
                    house - heaters[pos - 1]
                } else {
                    i32::MAX
                };
                tmp1.min(tmp2)
            };
            ans = ans.max(dis);
        }

        ans
    }
}

fn main() {
    println!("{}", Solution::find_radius(vec![1, 2, 3], vec![2]));
}
