// Copyright (c) BohuTANG
// Code is licensed with BSD

#[cfg(test)]
pub mod tests {
    use algebra::gcd;

    #[test]
    fn gcd_gcd_test() {
        assert_eq!(5, gcd::gcd(10, 25));
        assert_eq!(1, gcd::gcd(37, 14));
    }

    #[test]
    fn gcd_xgcd_test() {
        let (g, x, y) = gcd::xgcd(37, 14);
        assert_eq!(x * 37 + y * 14, g);
        assert_eq!(g, 1);
        assert_eq!(x, -3);
        assert_eq!(y, 8);

        let (g, x, y) = gcd::xgcd(10, 25);
        assert_eq!(x * 10 + y * 25, g);
    }
}
