use std::collections::HashMap;

struct Solution;
impl Solution {
    fn finish(needs: &Vec<i32>) -> bool {
        needs.iter().all(|&x| x == 0)
    }

    fn go(
        price: &Vec<i32>,
        special: &Vec<Vec<i32>>,
        needs: &mut Vec<i32>,
        map: &mut HashMap<Vec<i32>, i32>,
    ) -> i32 {
        if Self::finish(needs) {
            return 0;
        }

        if let Some(result) = map.get(needs) {
            return *result;
        }

        let mut total = i32::MAX;
        /* Pick special offer as possible */
        for idx in 0..special.len() {
            let offer = &special[idx];

            let mut zip = needs.iter().zip(offer);
            if zip.any(|x| x.0 < x.1) {
                continue;
            }

            for n in 0..needs.len() {
                needs[n] -= offer[n];
            }

            total = total.min(offer[needs.len()] + Self::go(price, special, needs, map));

            for n in 0..needs.len() {
                needs[n] += offer[n];
            }
        }

        let mut t = 0;
        for idx in 0..price.len() {
            t += needs[idx] * price[idx];
        }

        total = total.min(t);

        map.insert(needs.to_vec(), total);

        return total;
    }

    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, mut needs: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        Self::go(&price, &special, &mut needs, &mut map)
    }
}

fn main() {
    println!(
        "{}",
        Solution::shopping_offers(vec![3, 2], vec![vec![1, 2, 1]], vec![5, 5])
    );
}
