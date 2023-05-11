struct Solution;

struct BitMap {
    pub m: usize,
    pub n: usize,
    pub v: Vec<u64>,
}

impl BitMap {
    pub fn new(m: usize, n: usize) -> Self {
        BitMap {
            m: m,
            n: n,
            v: vec![0; (m * n) / 64 + 1],
        }
    }

    pub fn set(&mut self, i: usize, j: usize) {
        let bit = i * self.n + j;
        let slot = bit / 64;
        let pos = bit % 64;
        self.v[slot] |= 1 << pos;
    }

    pub fn is_set(&mut self, i: usize, j: usize) -> bool {
        let bit = i * self.n + j;
        let slot = bit / 64;
        let pos = bit % 64;
        ((self.v[slot] >> pos) & 1) == 1
    }
}

impl Solution {
    fn walk(
        heights: &mut Vec<Vec<i32>>,
        map: &mut BitMap,
        pre_h: i32,
        i: usize,
        j: usize,
        m: usize,
        n: usize,
    ) {
        if heights[i][j] < pre_h || map.is_set(i, j) {
            return;
        }

        let h = heights[i][j];
        map.set(i, j);

        if i + 1 < m {
            Self::walk(heights, map, h, i + 1, j, m, n);
        }

        if i > 0 {
            Self::walk(heights, map, h, i - 1, j, m, n);
        }

        if j + 1 < n {
            Self::walk(heights, map, h, i, j + 1, m, n);
        }

        if j > 0 {
            Self::walk(heights, map, h, i, j - 1, m, n);
        }
    }

    pub fn pacific_atlantic(mut heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();

        // For the point (i, j), map to bit "i * n + j"
        let mut pacific_map = BitMap::new(m, n);
        let mut atlantic_map = BitMap::new(m, n);

        for i in 0..m {
            Self::walk(&mut heights, &mut pacific_map, -1, i, 0, m, n);
            Self::walk(&mut heights, &mut atlantic_map, -1, i, n - 1, m, n);
        }

        for j in 0..n {
            Self::walk(&mut heights, &mut pacific_map, -1, 0, j, m, n);
            Self::walk(&mut heights, &mut atlantic_map, -1, m - 1, j, m, n);
        }

        let mut ans: Vec<Vec<i32>> = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if atlantic_map.is_set(i, j) && pacific_map.is_set(i, j) {
                    ans.push(vec![i as i32, j as i32]);
                }
            }
        }

        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::pacific_atlantic(vec![vec![1, 2], vec![2, 1]])
    );
}
