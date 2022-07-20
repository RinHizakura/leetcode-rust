struct Solution;
impl Solution {
    fn calc(s: &mut String, n: &mut i32, v:i32, sym: &str) {
        while *n >= v {
            let cnt = *n / v;
            *n -= cnt * v;
            for _i in 0..cnt {
                s.push_str(sym);
            }
        }
    }

    pub fn int_to_roman(num: i32) -> String {
        let mut s = String::new();
        let mut n = num;
        let symbol = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
        let value = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

        for i in 0..13 {
            Solution::calc(&mut s, &mut n, value[i], symbol[i]);
        }

        return s;
    }
}

fn main() {
    println!("{}", Solution::int_to_roman(1994));
}
