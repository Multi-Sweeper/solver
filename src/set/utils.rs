use factorial::Factorial;
use std::{cmp::max, cmp::min, ops::Range};

// pub struct Graph

pub fn product<T: Iterator<Item = i32>>(n: T) -> i32 {
    n.fold(1, |acc, next| acc * next)
}

pub fn fact_div(a: i32, b: i32) -> f64 {
    // this needs to take int and float?
    if a >= b {
        // product((b + 1)..(a + 1))
        product(Range {
            start: b + 1,
            end: a + 1,
        })
        .into()
    } else {
        1.0 / fact_div(b, a)
    }
}

pub fn choose(n: i32, k: i32) -> f64 {
    if n == 1 {
        1.0
    } else {
        fact_div(n, max(k, n - k)) / <u16 as Into<f64>>::into((min(k, n - k) as u16).factorial())
    }
}

pub fn peek<T: Iterator + Clone>(iter: T) -> Option<<T as Iterator>::Item> {
    iter.clone().next()
}
