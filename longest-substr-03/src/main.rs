pub fn length_of_longest_substring(s: String) -> i32 {

    if s == "" {
        return 0;
    }

    // 2^8 = 256
    let mut bitmap: [u64; 4] = [0; 4];
    let mut ans = 0;
    let mut ret = 0;

    let mut v = Vec::new();
    for c in s.bytes() {
        let slot = (c / 64) as usize;
        let bit = (c % 64) as usize;
        if bitmap[slot] & 1 << bit == 0 {
            bitmap[slot] |= 1 << bit;
            v.push(c);
            ans += 1;
        } else {
            if ans > ret {
                ret = ans;
            }

            loop {
                let tmp = v[0];
                v.remove(0);
                let slot = (tmp / 64) as usize;
                let bit = (tmp % 64) as usize;
                ans -= 1;
                bitmap[slot] &= !(1 << bit);

                if tmp == c {
                    bitmap[slot] |= 1 << bit;
                    v.push(c);
                    ans += 1;
                    break;
                }
            }
        }
    }

    // cheat
    if ans > ret {
        ret = ans;
    }

    ret
}

fn main() {
    let s = "dvdf";
    println!("{}", length_of_longest_substring(s.to_string()));
}
