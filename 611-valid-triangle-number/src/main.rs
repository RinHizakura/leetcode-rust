struct Solution;
impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();

        if len < 3 {
            return 0;
        }

        let mut ans = 0;

        nums.sort();

        for i in 0..(len - 1) {
            let a = nums[i];
            if a == 0 {
                continue;
            }
            for j in i + 1..(len - 1) {
                let b = nums[j];
                if b == 0 {
                    continue;
                }
                let sum = a + b;

                /* If the third edge with length c making c < a + b, a triangle
                 * can be made. Thus we search the index to insert a + b to
                 * find the range of availible edge length. */
                let mut result;
                let tmp = nums.binary_search(&sum);
                if tmp.is_ok() {
                    result = tmp.unwrap();
                    /* Adjust the index to the first found */
                    while result > 0 && result < len && nums[result - 1] == nums[result] {
                        result -= 1;
                    }
                } else {
                    result = tmp.unwrap_err();
                }

                ans += result - j - 1;
            }
        }

        ans as i32
    }
}

fn main() {
    println!("{}", Solution::triangle_number(vec![1, 2, 3, 5]));
}
