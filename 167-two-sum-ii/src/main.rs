struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let len = numbers.len();

        let mut index1 = 0;
        let mut index2 = len - 1;

        while index1 < index2 {
            if numbers[index1] + numbers[index2] == target {
                break;
            }

            if numbers[index1] + numbers[index2] > target {
                index2 -= 1;
            } else {
                index1 += 1;
            }
        }

        vec![index1 as i32 + 1, index2 as i32 + 1]
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![0, 1, 2, 5], 7));
}
