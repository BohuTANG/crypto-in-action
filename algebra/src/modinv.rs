use crate::gcd;

///  The function `mod_inv` computes the modular inverse of x mod m
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
/// use algebra::modinv;
///
/// fn main() {
///     let x = modinv::mod_inv(14, 37);
///     println!("{:?}", x);
/// }
/// ```
///
pub fn mod_inv(a: i8, m: i8) -> Option<i8> {
    let (g, x, _) = gcd::xgcd(a, m);
    if g != 1 {
        return None;
    } else {
        if x >= 0 {
            return Some(x);
        } else {
            return Some(m + x);
        }
    }
}
