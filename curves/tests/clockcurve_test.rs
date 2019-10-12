// Copyright (c) BohuTANG
// Code is licensed with BSD

#[cfg(test)]
pub mod tests {
    use curves::clockcurve;

    #[test]
    fn curves_clockcurve_scalar_add_test() {
        let clockcurve = clockcurve::ClockCurve::default();

        let p1 = clockcurve::Point { x: 2, y: 20 };
        let p2 = clockcurve::Point { x: 18, y: 24 };
        let p3 = clockcurve.scalar_add(p1, p2);

        assert_eq!(p3, clockcurve::Point { x: 5, y: 10 });
    }

    #[test]
    fn curves_clockcurve_scalar_double_test() {
        let clockcurve = clockcurve::ClockCurve::default();

        let p1 = clockcurve::Point { x: 2, y: 20 };
        let p2 = clockcurve.scalar_double(p1);

        assert_eq!(p2, clockcurve::Point { x: 18, y: 24 });
    }

    #[test]
    fn curves_clockcurve_scalar_mul_test() {
        let curve = clockcurve::ClockCurve::default();

        let p1 = clockcurve::Point { x: 7, y: 13 };
        for i in 1..34 {
            let p3 = curve.scalar_mul(p1, i);
            println!("{}P:\t({},{})", i, p3.x, p3.y);
        }

        // Identity.
        {
            let p3 = curve.scalar_mul(p1, 32);
            assert_eq!(p3, clockcurve::Point { x: 0, y: 1 });
        }

        // Cyclic.
        {
            let p3 = curve.scalar_mul(p1, 33);
            assert_eq!(p3, p1);
        }

        // Base mul.
        {
            let p3 = curve.scalar_basemul(33);
            assert_eq!(p3, clockcurve::Point { x: 2, y: 20 });
        }
    }

    #[test]
    fn curves_clockcurve_is_on_curve_test() {
        let clockcurve = clockcurve::ClockCurve::default();

        {
            let p1 = clockcurve::Point { x: 2, y: 20 };
            let res = clockcurve.is_on_curve(p1);
            assert_eq!(res, true);
        }
        {
            let p1 = clockcurve::Point { x: 3, y: 20 };
            let res = clockcurve.is_on_curve(p1);
            assert_eq!(res, false);
        }
    }

    #[test]
    fn curves_clockcurve_y_test() {
        let clockcurve = clockcurve::ClockCurve::default();

        let y = clockcurve.y(30);
        assert_eq!(y.unwrap(), 0);
    }

    #[test]
    fn curves_clockcurve_element_test() {
        let clockcurve = clockcurve::ClockCurve::default();

        for i in 0..31 {
            let p3 = clockcurve.y(i);
            if p3 != None {
                println!("({},{})", i, p3.unwrap());
                println!("({},{})", i, clockcurve.primer - p3.unwrap());
            }
        }
    }
}
