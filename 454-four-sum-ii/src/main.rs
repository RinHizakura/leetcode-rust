use std::collections::HashMap;

struct Solution;
impl Solution {
    fn create_map(nums1: &Vec<i32>, nums2: &Vec<i32>) -> HashMap<i32, i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for n in nums1 {
            for m in nums2 {
                let sum = n + m;
                if let Some(cnt) = map.get_mut(&sum) {
                    *cnt += 1;
                } else {
                    map.insert(sum, 1);
                }
            }
        }

        map
    }

    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let map = Self::create_map(&nums1, &nums2);

        let mut ans = 0;
        for n in &nums3 {
            for m in &nums4 {
                let sum = -(n + m);
                if let Some(cnt) = map.get(&sum) {
                    ans += cnt;
                }
            }
        }

        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::four_sum_count(vec![0], vec![0], vec![0], vec![0])
    );
}
