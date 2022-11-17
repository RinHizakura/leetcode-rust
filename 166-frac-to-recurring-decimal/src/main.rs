use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let sign = if numerator as i64 * denominator as i64 >= 0 {
            "".to_string()
        } else {
            "-".to_string()
        };
        let mut n = (numerator as i64).abs();
        let d = (denominator as i64).abs();
        let rslt = sign + &(n / d).to_string();
        n %= d;

        if n == 0 {
            return rslt;
        }

        let mut count = 0;
        let mut repeat_start = 0;
        let mut frac = String::new();
        let mut map: HashMap<i64, usize> = HashMap::new();
        let mut repeat = false;
        while n != 0 {
            if map.contains_key(&n) {
                repeat_start = *map.get(&n).unwrap();
                repeat = true;
                break;
            } else {
                map.insert(n, count);
                n *= 10;
                frac += &(n / d).to_string();
                n %= d;
                count += 1;
            }
        }

        if repeat == true {
            rslt + "."
                + frac.get(0..repeat_start).unwrap()
                + "("
                + frac.get(repeat_start..count).unwrap()
                + ")"
        } else {
            rslt + "." + &frac
        }
    }
}

fn main() {
    println!("{}", Solution::fraction_to_decimal(1, 2));
}
