#[cfg(test)]
pub mod tests {
    use crate::gcd;
    use crate::modinv;

    #[test]
    fn gcd_test() {
        assert_eq!(5, gcd::gcd(10, 25));
        assert_eq!(1, gcd::gcd(37, 14));
    }

    #[test]
    fn xgcd_test() {
        let (g, x, y) = gcd::xgcd(37, 14);
        assert_eq!(x * 37 + y * 14, g);
        assert_eq!(g, 1);
        assert_eq!(x, -3);
        assert_eq!(y, 8);

        let (g, x, y) = gcd::xgcd(10, 25);
        assert_eq!(x * 10 + y * 25, g);
    }

    #[test]
    fn modinv_test() {
        {
            let x = modinv::mod_inv(14, 37);
            assert_eq!(x, Some(8));
        }

        {
            let x = modinv::mod_inv(14, 38);
            assert_eq!(x, None);
        }
    }
}
