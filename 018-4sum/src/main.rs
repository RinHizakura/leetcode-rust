struct Solution;
impl Solution {
    fn inc(nums: &Vec<i32>, i: usize, len: usize) -> usize {
        let n = nums[i];
        let mut i = i + 1;
        while i < len && n == nums[i] {
            i += 1;
        }

        i
    }

    fn dec(nums: &Vec<i32>, i: usize) -> usize {
        let n = nums[i];

        let mut i = i - 1;
        while i > 0 && n == nums[i] {
            i -= 1;
        }

        i
    }

    fn safe_add(a:i32, b:i32) -> (bool, i32) {
        let t = a.checked_add(b);

        match t {
            Some(t) => (true, t),
            None => (false, 0),
        }
    }

    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut nums = nums.clone();
        nums.sort();

        let mut ans_vec = Vec::new();
        for j in 0..len {
            if j > 0 && nums[j] == nums[j - 1] {
                continue;
            }
            for i in j+1..len {
                if i > j+1 && nums[i] == nums[i - 1] {
                    continue;
                }
                let (check, t) = Solution::safe_add(nums[i], nums[j]);

                if check == false {
                    continue;
                }

                let mut left = i + 1;
                let mut right = len - 1;

                while left < right {
                    let (check, sum) = Solution::safe_add(t, nums[left]);
                    if check == false {
                        left = Solution::inc(&nums, left, len);
                        continue;
                    }
                    let (check, sum) = Solution::safe_add(sum, nums[right]);
                    if check == false {
                        right = Solution::dec(&nums, left);
                        continue;
                    }

                    if sum == target {
                        ans_vec.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                        left = Solution::inc(&nums, left, len);
                        right = Solution::dec(&nums, right);
                    } else if sum < target {
                        left = Solution::inc(&nums, left, len);
                    } else {
                        right = Solution::dec(&nums, right);
                    }
                }
            }
        }

        ans_vec
    }
}


fn main() {
    println!("{:?}", Solution::four_sum(vec![1000000000,1000000000,1000000000,1000000000], -294967296));
}
