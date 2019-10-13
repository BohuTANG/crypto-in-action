// Copyright (c) BohuTANG
// Code is licensed with BSD

use crate::gcd;

/// Computes two numbers sum in modulo arithmetic.
///
/// ```text
/// a + b (mod m)
/// ```
///
/// # Examples
///
/// ```rust
/// use algebra::arith;
///
/// fn main() {
///     let x = arith::mod_add(14, 35, 37);
///     println!("{:?}", x);
/// }
/// ```
pub fn mod_add(a: i8, b: i8, m: i8) -> i8 {
    let a1 = a % m;
    let b1 = b % m;

    if b1 > (m - a1) {
        b1 - (m - a1)
    } else {
        (a1 + b1 + m) % m
    }
}

/// Computes two numbers subtract in modulo arithmetic.
///
/// ```text
/// a - b (mod m)
/// ```
///
/// # Examples
///
/// ```rust
/// use algebra::arith;
///
/// fn main() {
///     let x = arith::mod_sub(14, 35, 37);
///     println!("{:?}", x);
/// }
/// ```
pub fn mod_sub(a: i8, b: i8, m: i8) -> i8 {
    mod_add(a, -b, m)
}

/// Computes two numbers product in modulo arithmetic.
///
/// ```text
/// a * b (mod m)
/// ```
///
/// # Examples
///
/// ```rust
/// use algebra::arith;
///
/// fn main() {
///     let x = arith::mod_mul(14, 35, 37);
///     println!("{:?}", x);
/// }
/// ```
pub fn mod_mul(a: i8, b: i8, m: i8) -> i8 {
    let mut res = 0;
    for _ in 1..=b {
        res += a;
        res %= m;
    }
    res
}

/// Computes two numbers division in modulo arithmetic.
///
/// ```text
/// a / b (mod m)
/// ```
///
/// # Examples
///
/// ```rust
/// use algebra::arith;
///
/// fn main() {
///     let x = arith::mod_div(4, 14, 37);
///     println!("{:?}", x);
/// }
/// ```
pub fn mod_div(a: i8, b: i8, m: i8) -> i8 {
    let inv = mod_inv(b, m);
    mod_mul(a, inv, m)
}

/// Computes the inverse of x in modulo arithmetic.
///
/// ```text
/// x = a ^ -1 (mod m)
/// ```
///
/// ```text
/// Algorithm:
/// x == a ^ -1 (mod m)
/// a * x == 1 (mod m)
/// a * x + m * y == 1 (mod m)
/// (g,x,y)= xgcd(a,m)
/// ```
///
/// # Examples
///
/// ```rust
/// use algebra::arith;
///
/// fn main() {
///     let x = arith::mod_inv(14, 37);
///     println!("{:?}", x);
/// }
/// ```
pub fn mod_inv(a: i8, m: i8) -> i8 {
    let (g, x, _) = gcd::xgcd(a, m);
    assert!(g == 1);
    (x + m) % m
}

/// Computes exponention in modulo arithmetic.
///
/// ```text
/// a ^ b (mod m)
/// ```
///
/// # Examples
///
/// ```rust
/// use algebra::arith;
///
/// fn main() {
///     let x = arith::mod_exp(3, 4, 37);
///     println!("{:?}", x);
/// }
/// ```
pub fn mod_exp(base: i8, exponent: i8, m: i8) -> i8 {
    let mut res = base;
    for _ in 1..exponent {
        res = mod_mul(res, base, m);
    }
    res
}
