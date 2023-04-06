struct Solution;
impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut path_stack = vec![];

        let input = input.as_bytes();
        let len = input.len();

        let mut i = 0;
        let mut path_len = 0;
        let mut max: i32 = 0;
        let mut is_file = false;

        while i < len {
            let c = input[i];

            match c {
                b'\n' => {
                    if is_file == true {
                        max = max.max(path_len);
                        is_file = false;
                    } else {
                        /* Add one for the extra '\' character in the path */
                        path_stack.push(path_len + 1);
                    }
                    path_len = 0;
                    i += 1;

                    /* deal with the following '\t' directly */
                    let mut depth = 0;
                    while i < len && input[i] == b'\t' {
                        path_len = path_stack[depth];
                        i += 1;
                        depth += 1;
                    }

                    while depth < path_stack.len() {
                        path_stack.pop();
                    }
                }
                b'\t' => unreachable!(),
                b'.' => {
                    path_len += 1;
                    is_file = true;
                    i += 1;
                }
                _ => {
                    path_len += 1;
                    i += 1;
                }
            }
        }

        /* Take care of the case when the last entry in also a file */
        if is_file {
            max = max.max(path_len);
        }

        max
    }
}

fn main() {
    println!(
        "{}",
        Solution::length_longest_path("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_string())
    );
}
