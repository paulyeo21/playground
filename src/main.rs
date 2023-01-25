use std::f64::consts::E;

fn main() {
    let n = 2.0_f64.powf(58.0);
    let k = 100000.0_f64;

    let res = 1.0 - E.powf((-k * (k - 1.0)) / (2.0 * n));
    println!("{}", res);
}
