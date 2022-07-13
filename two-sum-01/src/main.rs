pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // I don't like the hashmap solution, so...?

    let mut sort_nums = nums.clone();
    sort_nums.sort();

    let mut n1: i32 = 0;
    let mut n2: i32 = 0;

    for n in &sort_nums {
        let search = target - n;

        if let Ok(n_match) = sort_nums.binary_search(&search) {
            n1 = *n;
            n2 = sort_nums[n_match] as i32;
            break;
        }
    }

    let mut i1: i32 = -1;
    let mut i2: i32 = -1;
    for (i, n) in nums.iter().enumerate() {
        if *n == n1 || *n == n2 {
            if i1 == -1 {
                i1 = i as i32;
            } else {
                i2 = i as i32;
                break;
            }
        }
    }

    return vec![i1, i2];
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    println!("{:?}", two_sum(nums, target));
}
