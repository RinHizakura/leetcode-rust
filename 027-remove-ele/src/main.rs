struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let len = nums.len();

        let mut i = 0;
        loop {
            if i >= len {
                break;
            }

            if nums[i] != val {
                i += 1;
                continue;
            }

            let mut found = false;
            for j in i+1..len {
                if nums[j] != val {
                    found = true;
                    let tmp = nums[i];
                    nums[i] = nums[j];
                    nums[j] = tmp;
                    break;
                }
            }

            if found == false {
                break;
            }

            i += 1;
        }

        i as i32
    }
}

fn main() {
    println!("{}", Solution::remove_element(&mut vec![0,1,2,2,3,0,4,2], 2));
}
