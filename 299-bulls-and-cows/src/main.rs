struct Solution;
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let len = secret.len();

        let secret = secret.into_bytes();
        let guess = guess.into_bytes();

        let mut secret_collect = vec![0; 10];
        let mut guess_collect = vec![0; 10];

        let mut a_total = 0;
        for idx in 0..len {
            if secret[idx] == guess[idx] {
                a_total += 1;
                continue;
            }

            secret_collect[(secret[idx] - '0' as u8) as usize] += 1;
            guess_collect[(guess[idx] - '0' as u8) as usize] += 1;
        }

        let mut b_total = 0;

        for (s, g) in secret_collect.iter().zip(guess_collect.iter()) {
            b_total += s.min(g);
        }

        a_total.to_string() + "A" + &b_total.to_string() + "B"
    }
}

fn main() {
    println!(
        "{}",
        Solution::get_hint("1234".to_string(), "1789".to_string())
    );
}
