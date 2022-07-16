pub fn convert(s: String, num_rows: i32) -> String {

    if num_rows == 1 {
        return s;
    }

    let len = s.len();
    let s = s.into_bytes();
    let mut string = String::new();
    let d = 2;
    for i in 0..num_rows as usize {
        let mut idx = i;
        let i = i as i32;

        while idx < len {
            string.push(s[idx] as char);

            // extra word in zigzag is required
            let extra = idx as usize + d * (num_rows - 1 - i) as usize;
            if i != 0 && i != num_rows -1 && extra < len {
                string.push(s[extra] as char);
            }

            idx += d * (num_rows - 1) as usize;
        }
    }

    string
}

fn main() {
    println!("{}", convert("PAYPALISHIRING".to_string(), 3));
    println!("{}", convert("PAYPALISHIRING".to_string(), 4));
    println!("{}", convert("PAYPALISHIRING".to_string(), 1));
}
