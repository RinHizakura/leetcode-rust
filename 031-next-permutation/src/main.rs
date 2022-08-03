struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();

        let mut change = false;
        for i in (0..len - 1).rev() {
            /* We can binary search this, but RinHizakura is a lazy guy zz */
            if nums[i] < nums[i + 1] {
                let pivot = i;
                change = true;
                for j in (pivot + 1..len).rev() {
                    if nums[j] > nums[pivot] {
                        nums.swap(j, pivot);
                        let start = pivot + 1;
                        let end = len;
                        nums[start..end].reverse();
                        break;
                    }
                }

                break;
            }
        }

        /* handle the special case */
        if change == false {
            nums.reverse();
        }
    }
}

fn main() {
    let mut v = vec![1, 5, 1];
    Solution::next_permutation(&mut v);
    println!("{:?}", v);
}
