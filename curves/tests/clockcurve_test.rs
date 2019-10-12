// Copyright (c) BohuTANG
// Code is licensed with BSD

#[cfg(test)]
pub mod tests {
    use curves::clockcurve;

    #[test]
    fn curves_clockcurve_scalar_add_test() {
        let curv = clockcurve::ClockCurve::default();

        {
            let p1 = clockcurve::Point { x: 2, y: 20 };
            let p2 = clockcurve::Point { x: 18, y: 24 };
            let p3 = curv.scalar_add(p1, p2);

            assert_eq!(p3, clockcurve::Point { x: 5, y: 10 });
        }

        {
            let p1 = clockcurve::Point { x: 2, y: 20 };
            let p2 = curv.point_neg(p1);
            let p3 = curv.scalar_add(p1, p2);

            assert_eq!(p3, clockcurve::Point { x: 0, y: 1 });
        }
    }

    #[test]
    fn curves_clockcurve_scalar_double_test() {
        let clockcurve = clockcurve::ClockCurve::default();

        let p1 = clockcurve::Point { x: 2, y: 20 };
        let p2 = clockcurve.scalar_double(p1);
        assert_eq!(p2, clockcurve::Point { x: 18, y: 24 });

        let p4 = clockcurve.scalar_double(p2);
        assert_eq!(p4, clockcurve::Point { x: 27, y: 4 });

        let p8 = clockcurve.scalar_double(p4);
        assert_eq!(p8, clockcurve::Point { x: 30, y: 0 });

        let p16 = clockcurve.scalar_double(p8);
        assert_eq!(p16, clockcurve::Point { x: 0, y: 30 });

        let p32 = clockcurve.scalar_double(p16);
        assert_eq!(p32, clockcurve::Point { x: 0, y: 1 });
    }

    #[test]
    fn curves_clockcurve_scalar_mul_test() {
        let curve = clockcurve::ClockCurve::default();

        let p1 = clockcurve::Point { x: 7, y: 13 };
        for i in 1..34 {
            let p3 = curve.scalar_mul(p1, i);
            println!("{}P:\t({},{})", i, p3.x, p3.y);
        }
        /*
        1P:     (7,13)
        2P:     (27,27)
        3P:     (13,7)
        4P:     (1,0)
        5P:     (13,24)
        6P:     (27,4)
        7P:     (7,18)
        8P:     (0,30)
        9P:     (24,18)
        10P:    (4,4)
        11P:    (18,24)
        12P:    (30,0)
        13P:    (18,7)
        14P:    (4,27)
        15P:    (24,13)
        16P:    (0,1)
        17P:    (7,13)
        18P:    (27,27)
        19P:    (13,7)
        20P:    (1,0)
        21P:    (13,24)
        22P:    (27,4)
        23P:    (7,18)
        24P:    (0,30)
        25P:    (24,18)
        26P:    (4,4)
        27P:    (18,24)
        28P:    (30,0)
        29P:    (18,7)
        30P:    (4,27)
        31P:    (24,13)
        32P:    (0,1)
        33P:    (7,13)
        */

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
    fn curves_clockcurve_mul_test() {
        let clockcurve = clockcurve::ClockCurve::default();

        {
            let p1 = clockcurve::Point { x: 2, y: 20 };
            let mut points: Vec<clockcurve::Point> = Vec::new();
            for i in 1..40 {
                let res = clockcurve.scalar_mul(p1, i);
                points.push(res);
                println!("{}, {:?}", i, res);
            }

            /*
            1, Point { x: 2, y: 20 }
            2, Point { x: 18, y: 24 }
            3, Point { x: 5, y: 10 }
            4, Point { x: 27, y: 4 }
            5, Point { x: 21, y: 26 }
            6, Point { x: 7, y: 13 }
            7, Point { x: 11, y: 29 }
            8, Point { x: 30, y: 0 }
            9, Point { x: 11, y: 2 }
            10, Point { x: 7, y: 18 }
            11, Point { x: 21, y: 5 }
            12, Point { x: 27, y: 27 }
            13, Point { x: 5, y: 21 }
            14, Point { x: 18, y: 7 }
            15, Point { x: 2, y: 11 }
            16, Point { x: 0, y: 30 }
            17, Point { x: 29, y: 11 }
            18, Point { x: 13, y: 7 }
            19, Point { x: 26, y: 21 }
            20, Point { x: 4, y: 27 }
            21, Point { x: 10, y: 5 }
            22, Point { x: 24, y: 18 }
            23, Point { x: 20, y: 2 }
            24, Point { x: 1, y: 0 }
            25, Point { x: 20, y: 29 }
            26, Point { x: 24, y: 13 }
            27, Point { x: 10, y: 26 }
            28, Point { x: 4, y: 4 }
            29, Point { x: 26, y: 10 }
            30, Point { x: 13, y: 24 }
            31, Point { x: 29, y: 20 }
            32, Point { x: 0, y: 1 }
            33, Point { x: 2, y: 20 }
            34, Point { x: 18, y: 24 }
            35, Point { x: 5, y: 10 }
            36, Point { x: 27, y: 4 }
            37, Point { x: 21, y: 26 }
            38, Point { x: 7, y: 13 }
            39, Point { x: 11, y: 29 }
            */
            assert_eq!(points[0], clockcurve::Point { x: 2, y: 20 });
            assert_eq!(points[1], clockcurve::Point { x: 18, y: 24 });
            assert_eq!(points[2], clockcurve::Point { x: 5, y: 10 });
            assert_eq!(points[15], clockcurve::Point { x: 0, y: 30 });
            assert_eq!(points[16], clockcurve::Point { x: 29, y: 11 });
            assert_eq!(points[31], clockcurve::Point { x: 0, y: 1 });
            assert_eq!(points[32], clockcurve::Point { x: 2, y: 20 });
            assert_eq!(points[38], clockcurve::Point { x: 11, y: 29 });
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
        /*
        (0,1)
        (0,30)
        (1,0)
        (1,31)
        (2,20)
        (2,11)
        (4,4)
        (4,27)
        (5,10)
        (5,21)
        (7,18)
        (7,13)
        (10,5)
        (10,26)
        (11,2)
        (11,29)
        (13,7)
        (13,24)
        (18,7)
        (18,24)
        (20,2)
        (20,29)
        (21,5)
        (21,26)
        (24,18)
        (24,13)
        (26,10)
        (26,21)
        (27,4)
        (27,27)
        (29,20)
        (29,11)
        (30,0)
        (30,31)
        */
    }
}
