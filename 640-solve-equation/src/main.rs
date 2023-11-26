struct Solution;
impl Solution {
    fn parse(s: &[u8]) -> (i64, i64) {
        let mut x = 0;
        let mut val = 0;
        let mut tmp = 0;
        let mut get_val = false;

        let mut sign = 1;
        for idx in 0..s.len() {
            match s[idx] {
                b'+' => {
                    val += sign * tmp;
                    sign = 1;
                    tmp = 0;
                    get_val = false;
                }

                b'-' => {
                    val += sign * tmp;
                    sign = -1;
                    tmp = 0;
                    get_val = false;
                }

                b'x' => {
                    if get_val == false {
                        tmp = 1;
                    }
                    x += sign * tmp;
                    tmp = 0;
                }

                _ => {
                    tmp = tmp * 10 + (s[idx] - b'0') as i64;
                    get_val = true;
                }
            }
        }

        /* process the last value if exist */
        val += tmp * sign;

        (x, val)
    }

    pub fn solve_equation(equation: String) -> String {
        let v: Vec<&str> = equation.split('=').collect();

        let (left_x, left_val) = Self::parse(v[0].as_bytes());
        let (right_x, right_val) = Self::parse(v[1].as_bytes());

        if left_x == right_x {
            if left_val == right_val {
                return "Infinite solutions".to_string();
            } else {
                return "No solution".to_string();
            }
        }

        format!("x={}", (right_val - left_val) / (left_x - right_x))
    }
}

fn main() {
    println!("{}", Solution::solve_equation("x=x".to_string()));
}
