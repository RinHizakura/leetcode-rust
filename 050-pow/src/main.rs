pub fn my_pow(x: f64, n: i32) -> f64 {

    if n == 0 {
        return 1.0;
    } else if n == 1 {
        return x;
    } else if n == -1 {
        return 1.0 /x;
    } else if n == 2 {
        return x * x;
    } else if n == -2 {
       return 1.0 / (x * x);
    }

    if n & 1 == 0 {
        let r = my_pow(x, n >> 1);
        return r * r;
    } else {
        return x * my_pow(x, n -1);
    }
}

fn main() {
    println!("{}", my_pow(2.0, -2));
}
