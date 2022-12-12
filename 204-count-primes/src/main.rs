struct Solution;
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }

        let n = n as usize;
        let mut is_prime = vec![1; n];
        is_prime[0] = 0; // 0 is not prime
        is_prime[1] = 0; // 1 is not prime
        let mut count = n - 2;

        let mut i = 2;

        // check i from 2 to sqrt(n)
        while i * i < n {
            if is_prime[i] == 0 {
                i += 1;
                continue;
            }

            let mut test = i * i;
            // mark i * j, where j = i, i+1, ... to prime
            while test < n {
                if is_prime[test] == 1 {
                    count -= 1;
                    is_prime[test] = 0;
                }
                test += i;
            }

            i += 1;
        }

        count as i32
    }
}

fn main() {
    println!("{}", Solution::count_primes(10));
}
