struct Solution;
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let len = digits.len();

        let mut carry = 1;
        for i in (0..len).rev() {
            let tmp = digits[i] + carry;
            carry = tmp / 10;
            digits[i] = tmp % 10;

            if carry == 0 {
                break;
            }
        }

        if carry != 0 {
            digits.insert(0, carry);
        }
        digits
    }
}

fn main() {
    println!("{:?}", Solution::plus_one(vec![9, 9]));
}
