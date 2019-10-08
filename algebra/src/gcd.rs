// Copyright (c) BohuTANG
// Code is licensed with BSD

use std::mem;

/// The function `gcd` implements Euclidean algorithm with non-recursive
///
/// ```text
/// Algorithm:
/// gcd(37,14)
/// 37 = 14(2) + 9
/// 14 = 9(1)  + 5
/// 9  = 5(1)  + 4
/// 5  = 4(1)  + 1
/// ```
/// # Examples
///
/// ```rust
/// use algebra::gcd;
///
/// fn main() {
///     let g = gcd::gcd(37, 14);
///     println!("{}", g);
/// }
/// ```
///
pub fn gcd(mut a: i8, mut b: i8) -> i8 {
    while b != 0 {
        a %= b;
        let tmp = a;
        a = b;
        b = tmp;
    }
    return a;
}

/// The function `xgcd` implements Extended Euclidean algorithm with non-recursive
///
/// Given integers a and b, compute integers a and b such that
/// ```text
/// a * x + b * y == gcd(a, b).
/// ```
///
/// # Examples
///
/// ```rust
/// use algebra::gcd;
///
/// fn main() {
///     let (g, x, y) = gcd::xgcd(37, 14);
///     println!("{},{},{}", g,x,y);
/// }
/// ```
///
pub fn xgcd(a: i8, b: i8) -> (i8, i8, i8) {
    let (mut s, mut s_old) = (0, 1);
    let (mut t, mut t_old) = (1, 0);
    let (mut r, mut r_old) = (b, a);
    while r != 0 {
        let quotient = r_old / r;
        r_old -= quotient * r;
        s_old -= quotient * s;
        t_old -= quotient * t;
        mem::swap(&mut r, &mut r_old);
        mem::swap(&mut s, &mut s_old);
        mem::swap(&mut t, &mut t_old);
    }
    (r_old, s_old, t_old) // (gcd, coeff_a, coeff_b)
}
