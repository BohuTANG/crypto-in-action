// Copyright (c) BohuTANG
// Code is licensed with BSD

#[cfg(test)]
pub mod tests {
    use algebra::arith;

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

        {
            let x = arith::mod_div(1, 14, 37);
            assert_eq!(x, 8);
        }

        {
            let x = arith::mod_div(1, 23, 37);
            assert_eq!(x, 29);
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
