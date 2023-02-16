struct Solution;
impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut sequence = Vec::new();
        sequence.push(1);

        let len = primes.len();
        let mut prod = vec![0; len];

        for _ in 1..n {
            let mut min = u32::MAX;
            for prime_idx in 0..len {
                let tmp: u32 = primes[prime_idx] as u32 * sequence[prod[prime_idx]];
                if min > tmp {
                    min = tmp;
                }
            }

            for prime_idx in 0..len {
                let tmp: u32 = primes[prime_idx] as u32 * sequence[prod[prime_idx]];
                if min == tmp {
                    prod[prime_idx] += 1;
                }
            }

            sequence.push(min);
        }

        sequence[n as usize - 1] as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19])
    );
}
