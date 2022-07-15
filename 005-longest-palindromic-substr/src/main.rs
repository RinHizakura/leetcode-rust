/* Such an ugly solution...... */
pub fn __longest_palindrome(
    s: &Vec<u8>,
    len: isize,
    mut start: isize,
    mut end: isize,
) -> (usize, isize, isize) {
    let mut ans = 0;

    while start >= 0 && end < len && s[start as usize] == s[end as usize] {
        ans += 2;
        start -= 1;
        end += 1;
    }

    return (ans, start + 1, end - 1);
}

pub fn longest_palindrome(s: String) -> String {
    let len = s.bytes().count() as isize;
    let s = s.into_bytes();

    let mut max = 1;
    let mut max_start = 0;
    let mut max_end = 0;

    for i in 0..len - 1 {
        let (ans1, start1, end1) = __longest_palindrome(&s, len, i as isize, i as isize);
        let (ans2, start2, end2) = __longest_palindrome(&s, len, i as isize, i + 1 as isize);

        // Adjust length for the middle character of pattern 1
        let ans1 = ans1 - 1;

        if max < ans1 {
            max_start = start1 as usize;
            max_end = end1 as usize;
            max = ans1;
        }

        if max < ans2 {
            max_start = start2 as usize;
            max_end = end2 as usize;
            max = ans2;
        }
    }

    std::str::from_utf8(&s[max_start..max_end + 1])
        .unwrap()
        .to_string()
}

fn main() {
    //println!("{}", longest_palindrome("cbbd".to_string()));
    println!("{}", longest_palindrome("bb".to_string()));
}
