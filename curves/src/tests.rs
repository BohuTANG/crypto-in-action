// Copyright (c) BohuTANG
// Code is licensed with BSD

#[cfg(test)]
pub mod tests {
    use crate::curve31;

    #[test]
    fn curves_curve31_scalar_add_test() {
        let curve31 = curve31::Curve31::default();

        let p1 = curve31::Point { x: 2, y: 20 };
        let p2 = curve31::Point { x: 0, y: 1 };
        let p3 = curve31.scalar_add(p1, p2);
        println!("{:?}", p3);
    }

    #[test]
    fn curves_curve31_scalar_mul_test() {
        let curve = curve31::Curve31::default();

        let p1 = curve31::Point { x: 2, y: 20 };
        for i in 1..34 {
            let p3 = curve.scalar_mul(p1, i);
            println!("i:{},{:?}", i, p3);
        }

        // Identity.
        {
            let p3 = curve.scalar_mul(p1, 32);
            assert_eq!(p3, curve31::Point { x: 0, y: 1 });
        }

        // Cyclic.
        {
            let p3 = curve.scalar_mul(p1, 33);
            assert_eq!(p3, curve31::Point { x: 2, y: 20 });
        }

        // Base mul.
        {
            let p3 = curve.scalar_basemul(33);
            assert_eq!(p3, curve31::Point { x: 2, y: 20 });
        }
    }

    #[test]
    fn curves_curve31_element_test() {
        let curve31 = curve31::Curve31::default();

        for i in 0..40 {
            let p3 = curve31.y(i);
            if p3 != None {
                println!("({},{})", i, p3.unwrap());
            }
        }
    }
}
