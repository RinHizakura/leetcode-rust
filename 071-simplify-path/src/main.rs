struct Solution;
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut path: Vec<&str> = path.split('/').collect();

        let mut len: i32 = path.len() as i32;
        let mut i: i32 = 0;
        loop {
            if i >= len {
                break;
            }

            if path[i as usize] == "" || path[i as usize] == "." {
                path.remove(i as usize);
                len -= 1;
            } else if path[i as usize] == ".." {
                path.remove(i as usize);
                len -= 1;
                if i == 0 {
                    continue;
                }
                path.remove((i - 1) as usize);
                i -= 1;
                len -= 1;
            } else {
                i += 1;
            }
        }

        if path.is_empty() {
            return "/".to_string();
        }

        let mut s = String::new();
        for p in path {
            s.push('/');
            s.push_str(p);
        }
        s
    }
}

fn main() {
    let s = "///eHx/..";
    println!("{:?}", Solution::simplify_path(s.to_string()));
}
