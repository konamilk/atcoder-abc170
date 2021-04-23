use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        x: i32,
        y: i32
    };

    for xi in 1..=x {
        for yi in 1..=y {
            if 4 * x - y < 0{
                println!("No");
                return
            }
        }
        if -2 * x + y < 0{
            println!("No");
            return
        }
        if y % 2 == 1 {
            println!("No");
            return
        }
    }

    println!("Yes");
}
