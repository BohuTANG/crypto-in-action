// Copyright (c) BohuTANG
// Code is licensed with BSD

#[cfg(test)]
pub mod tests {
    use crate::curve31;
    use crate::keys;

    #[test]
    fn curves_curve31_scalar_add_test() {
        let curve31 = curve31::Curve31::default();

        let p1 = curve31::Point { x: 2, y: 20 };
        let p2 = curve31::Point { x: 18, y: 24 };
        let p3 = curve31.scalar_add(p1, p2);

        assert_eq!(p3, curve31::Point { x: 5, y: 10 });
    }

    #[test]
    fn curves_curve31_scalar_double_test() {
        let curve31 = curve31::Curve31::default();

        let p1 = curve31::Point { x: 2, y: 20 };
        let p2 = curve31.scalar_double(p1);

        assert_eq!(p2, curve31::Point { x: 18, y: 24 });
    }

    #[test]
    fn curves_curve31_scalar_mul_test() {
        let curve = curve31::Curve31::default();

        let p1 = curve31::Point { x: 7, y: 13 };
        for i in 1..34 {
            let p3 = curve.scalar_mul(p1, i);
            println!("{}P:\t({},{})", i, p3.x, p3.y);
        }

        // Identity.
        {
            let p3 = curve.scalar_mul(p1, 32);
            assert_eq!(p3, curve31::Point { x: 0, y: 1 });
        }

        // Cyclic.
        {
            let p3 = curve.scalar_mul(p1, 33);
            assert_eq!(p3, p1);
        }

        // Base mul.
        {
            let p3 = curve.scalar_basemul(33);
            assert_eq!(p3, curve31::Point { x: 2, y: 20 });
        }
    }

    #[test]
    fn curves_curve31_is_on_curve_test() {
        let curve31 = curve31::Curve31::default();

        {
            let p1 = curve31::Point { x: 2, y: 20 };
            let res = curve31.is_on_curve(p1);
            assert_eq!(res, true);
        }
        {
            let p1 = curve31::Point { x: 3, y: 20 };
            let res = curve31.is_on_curve(p1);
            assert_eq!(res, false);
        }
    }

    #[test]
    fn curves_curve31_y_test() {
        let curve31 = curve31::Curve31::default();

        let y = curve31.y(30);
        assert_eq!(y.unwrap(), 0);
    }

    #[test]
    fn curves_curve31_element_test() {
        let curve31 = curve31::Curve31::default();

        for i in 0..31 {
            let p3 = curve31.y(i);
            if p3 != None {
                println!("({},{})", i, p3.unwrap());
                println!("({},{})", i, curve31.primer - p3.unwrap());
            }
        }
    }

    #[test]
    fn curves_keys_test() {
        let privatekey = keys::PrivateKey::new(2);
        let publickey = privatekey.publickey();
        println!("({:?},{:?})", privatekey.serialize(), publickey.serialize());
    }
}
