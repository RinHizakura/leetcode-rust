struct Solution;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<String> = nums.iter().map(|num| num.to_string()).collect();

        let f = |a: &String, b: &String| {
            let s1 = a.to_owned() + b;
            let s2 = b.to_owned() + a;
            s2.cmp(&s1)
        };
        nums.sort_by(f);

        if nums[0] == "0".to_string() && nums.len() > 1 {
            return "0".to_string();
        }

        nums.join("")
    }
}

fn main() {
    println!("{:?}", Solution::largest_number(vec![1, 2, 3, 32]));
}
