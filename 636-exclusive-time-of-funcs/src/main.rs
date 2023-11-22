struct Solution;
impl Solution {
    fn parse(log: &String) -> (usize, usize, bool) {
        let token: Vec<&str> = log.split(':').collect();
        let task = token[0].parse::<usize>().unwrap();
        let time = token[2].parse::<usize>().unwrap();

        let is_start = if token[1].eq("start") { true } else { false };

        (task, time, is_start)
    }

    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut ans = vec![0; n as usize];
        let mut call_stack = Vec::new();
        let len = logs.len();

        let mut prev = 0;
        let (task, time, is_start) = Self::parse(&logs[0]);
        assert!(is_start);
        assert!(time == 0);
        let mut cur = task;
        call_stack.push(task);

        for idx in 1..len {
            let log = &logs[idx];
            let (task, time, is_start) = Self::parse(log);

            if is_start {
                let t = time - prev;
                ans[cur] += t as i32;
                cur = task;
                call_stack.push(task);
                prev = time;
            } else {
                let t = time - prev + 1;
                ans[cur] += t as i32;
                call_stack.pop();
                prev = time + 1;

                if let Some(last) = call_stack.last() {
                    cur = *last;
                }
            }
        }

        ans
    }
}

fn main() {
    let v = vec![
        "0:start:0".to_owned(),
        "1:start:2".to_owned(),
        "1:end:5".to_owned(),
        "0:end:6".to_owned(),
    ];
    println!("{:?}", Solution::exclusive_time(2, v));
}
