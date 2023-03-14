struct Solution;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Eq, PartialEq)]
struct State {
    cost: i32,
    idx: (usize, usize),
}

impl State {
    fn new(n1: &Vec<i32>, n2: &Vec<i32>, idx: (usize, usize)) -> Self {
        State {
            cost: n1[idx.0] + n2[idx.1],
            idx: idx,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut k = k as usize;
        let l1 = nums1.len();
        let l2 = nums2.len();

        let mut heap = BinaryHeap::new();
        heap.push(State::new(&nums1, &nums2, (0, 0)));
        let mut set = HashSet::new();
        set.insert((0, 0));

        let mut ans = vec![];
        while let Some(State { cost: _, idx }) = heap.pop() {
            ans.push(vec![nums1[idx.0], nums2[idx.1]]);

            let next_idx_one = (idx.0 + 1, idx.1);
            if next_idx_one.0 < l1 && !set.contains(&next_idx_one) {
                heap.push(State::new(&nums1, &nums2, next_idx_one));
                set.insert(next_idx_one);
            }

            let next_idx_two = (idx.0, idx.1 + 1);
            if next_idx_two.1 < l2 && !set.contains(&next_idx_two) {
                heap.push(State::new(&nums1, &nums2, next_idx_two));
                set.insert(next_idx_two);
            }

            k -= 1;
            if k == 0 {
                break;
            }
        }

        ans
    }
}

fn main() {
    println!("{:?}", Solution::k_smallest_pairs(vec![1], vec![2, 3], 2));
}
