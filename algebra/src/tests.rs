// Copyright (c) BohuTANG
// Code is licensed with BSD

#[cfg(test)]
pub mod tests {
    use crate::arith;
    use crate::gcd;

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

    #[test]
    fn arith_modadd_test() {
        {
            let x = arith::mod_add(14, 38, 37);
            assert_eq!(x, 15);
        }

        {
            let x = arith::mod_add(126, 126, 127);
            assert_eq!(x, 125);
        }
    }

    #[test]
    fn arith_modsub_test() {
        {
            let x = arith::mod_sub(14, 15, 37);
            assert_eq!(x, 36);
        }

        {
            let x = arith::mod_sub(15, 13, 37);
            assert_eq!(x, 2);
        }
    }

    #[test]
    fn arith_modmul_test() {
        {
            let x = arith::mod_mul(4, 16, 37);
            assert_eq!(x, 27);
        }

        {
            let x = arith::mod_mul(25, 30, 37);
            assert_eq!(x, 10);
        }
    }

    #[test]
    fn arith_moddiv_test() {
        {
            let x = arith::mod_div(4, 14, 37);
            assert_eq!(x, 32);
        }
    }

    #[test]
    fn arith_modinv_test() {
        {
            let x = arith::mod_inv(14, 37);
            assert_eq!(x, 8);
        }

        {
            let x = arith::mod_inv(2, 3);
            assert_eq!(x, 2);
        }

        {
            let x = arith::mod_inv(2, 127);
            assert_eq!(x, 64);
        }
    }

    #[test]
    fn arith_modexp_test() {
        {
            let x = arith::mod_exp(2, 3, 37);
            assert_eq!(x, 8);
        }

        {
            let x = arith::mod_exp(15, 2, 37);
            assert_eq!(x, 3);
        }
    }
}
