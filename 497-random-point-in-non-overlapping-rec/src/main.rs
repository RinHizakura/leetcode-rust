use rand::Rng;

struct Rec {
    x_min: i32,
    x_range: i32,
    y_min: i32,
    total_point: usize,
}

impl Rec {
    fn new(x_min: i32, x_range: i32, y_min: i32, y_range: i32) -> Rec {
        Rec {
            x_min,
            x_range,
            y_min,
            total_point: (x_range as usize * y_range as usize),
        }
    }
}

struct Solution {
    recs: Vec<Rec>,
    total_point: usize,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut recs: Vec<Rec> = Vec::new();
        let mut total_point = 0;

        for rect in rects {
            let x_range = rect[2] - rect[0] + 1;
            let y_range = rect[3] - rect[1] + 1;

            let rec = Rec::new(rect[0], x_range, rect[1], y_range);

            total_point += rec.total_point;
            recs.push(rec);
        }

        Solution { recs, total_point }
    }

    fn pick(&self) -> Vec<i32> {
        let mut rng = rand::thread_rng();

        let mut idx: usize = rng.gen::<usize>() % self.total_point;

        let mut rec_idx = 0;
        while idx >= self.recs[rec_idx].total_point {
            idx -= self.recs[rec_idx].total_point;
            rec_idx += 1;
        }

        let s_x = idx % self.recs[rec_idx].x_range as usize;
        let s_y = idx / self.recs[rec_idx].x_range as usize;
        let rand_x = (self.recs[rec_idx].x_min as usize).wrapping_add(s_x);
        let rand_y = (self.recs[rec_idx].y_min as usize).wrapping_add(s_y);

        vec![rand_x as i32, rand_y as i32]
    }
}

fn main() {
    let obj = Solution::new(vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]]);
    let ret_1: Vec<i32> = obj.pick();
    println!("{:?}", ret_1);
}
