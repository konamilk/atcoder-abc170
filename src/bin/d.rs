// use proconio::input;
// #[allow(unused_imports)]
// use proconio::source::auto::AutoSource;
// #[allow(unused_imports)]
// use proconio::marker::{Chars, Bytes};
// #[allow(unused_imports)]
// use num::integer::{sqrt, gcd, lcm};
// #[allow(unused_imports)]
// use std::cmp::{max, min, Reverse};

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
    input!{
        // from source,
        n: usize,
        a: [i32; n]
    };

    let max_num = 2_100_000;

    let mut num_tbl = vec![0; max_num];
    let mut yaku_tbl = vec![false; max_num];

    for i in 0..n {
        num_tbl[a[i] as usize] += 1;
    }

    for i in 1..max_num {
        if yaku_tbl[i] == true {
            continue
        }
        if num_tbl[i] == 0{
            continue
        }

        for j in 2..= max_num / i {
            if j * i > max_num - 1{
                break;
            }
            yaku_tbl[j * i] = true
        }
    }

    let mut ans = 0;

    for i in 1..max_num {
        if num_tbl[i] == 1 && !yaku_tbl[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
