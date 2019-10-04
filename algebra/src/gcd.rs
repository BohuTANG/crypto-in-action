use std::mem;

/// gcd(a,b)
/// Euclidean algorithm with non-recursive
///
/// case[1]
/// gcd(25,10)
/// 25 = 2*10 + 5
/// 10 = 2*5  + 0
///
/// case[2]
/// gcd(37,14)
/// 37 = 14(2) + 9
/// 14 = 9(1)  + 5
/// 9  = 5(1)  + 4
/// 5  = 4(1)  + 1
pub fn gcd(mut a: i8, mut b: i8) -> i8 {
    while b != 0 {
        a %= b;
        let tmp = a;
        a = b;
        b = tmp;
    }
    return a;
}

/// Finds the greatest common denominator of two integers *a* and *b*, and two
/// integers *x* and *y* such that *ax* + *by* is the greatest common
/// denominator of *a* and *b* (Bézout coefficients).
///
/// This function is an implementation of the [extended Euclidean
/// algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm).
///
///
/// ```
/// let (g, x, y) = xgcd(37, 14);
///
/// assert_eq!(g, 1);
/// assert_eq!(x, -3);
/// assert_eq!(y, 8);
/// assert_eq!((a * x) + (b * y), g);
/// ```
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
