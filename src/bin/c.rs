use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

use std::collections::BTreeSet;


fn main() {
//     let source = AutoSource::from("6 5
// 4 7 10 6 5
// ");
    input!{
        // from source,
        x: i32,
        n: usize,
        p: [i32; n]
    };

    if n == 0 {
        println!("{}", x);
        return
    }

    let mut bts = BTreeSet::new();

    for i in 0..n{
        bts.insert(p[i]);
    }

    let mut diff = 0;

    let ans;

    loop {
        match bts.get(&(x - diff)) {
            None => { ans = x - diff; break;},
            Some(_) => {}
        }
        match bts.get(&(x + diff)) {
            None => { ans = x + diff; break;},
            Some(_) => {}
        }
        diff += 1;
    }

    println!("{}", ans);

}
