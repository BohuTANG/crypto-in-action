// Copyright (c) BohuTANG
// Code is licensed with BSD

#[allow(non_snake_case)]
#[cfg(test)]
pub mod tests {
    use curves::clockcurve;
    use subgroups::subgroup;

    #[test]
    fn subgroups_subgroup_default_test() {
        let sub = subgroup::SubGroup::default();
        let points = sub.points();
        for point in points.iter() {
            println!("{:?}", point);
        }
        /*
        Point { x: 2, y: 20 }
        Point { x: 18, y: 24 }
        Point { x: 5, y: 10 }
        Point { x: 27, y: 4 }
        Point { x: 21, y: 26 }
        Point { x: 7, y: 13 }
        Point { x: 11, y: 29 }
        Point { x: 30, y: 0 }
        Point { x: 11, y: 2 }
        Point { x: 7, y: 18 }
        Point { x: 21, y: 5 }
        Point { x: 27, y: 27 }
        Point { x: 5, y: 21 }
        Point { x: 18, y: 7 }
        Point { x: 2, y: 11 }
        Point { x: 0, y: 30 }
        Point { x: 29, y: 11 }
        Point { x: 13, y: 7 }
        Point { x: 26, y: 21 }
        Point { x: 4, y: 27 }
        Point { x: 10, y: 5 }
        Point { x: 24, y: 18 }
        Point { x: 20, y: 2 }
        Point { x: 1, y: 0 }
        Point { x: 20, y: 29 }
        Point { x: 24, y: 13 }
        Point { x: 10, y: 26 }
        Point { x: 4, y: 4 }
        Point { x: 26, y: 10 }
        Point { x: 13, y: 24 }
        Point { x: 29, y: 20 }
        Point { x: 0, y: 1 }
                */

        assert_eq!(sub.order(), 32);
    }

    #[test]
    fn subgroups_subgroup_order4_test() {
        let g = clockcurve::Point { x: 1, y: 0 };
        let sub = subgroup::SubGroup::new(g);
        let points = sub.points();
        for point in points.iter() {
            println!("{:?}", point);
        }
        /*
        Point { x: 1, y: 0 }
        Point { x: 0, y: 30 }
        Point { x: 30, y: 0 }
        Point { x: 0, y: 1 }
        */

        assert_eq!(sub.order(), 4);

        // L=32/4=8
        {
            let zero = sub.scalar_basemul(8);
            assert_eq!(zero, sub.infinity);
        }
    }

    #[test]
    fn subgroups_subgroup_order_test() {
        let g = clockcurve::Point { x: 2, y: 20 };
        let sub = subgroup::SubGroup::new(g);
        let points = sub.points();
        let orders = [1, 2, 4, 8, 32];

        for odr in orders.iter() {
            for point in points.iter() {
                if sub.curve.scalar_mul(*point, *odr as i8) == sub.infinity {
                    println!("order:{:?}, {:?}", *odr, *point);
                }
            }
        }
        /*
        order:1, Point { x: 0, y: 1 }
        order:2, Point { x: 0, y: 30 }
        order:2, Point { x: 0, y: 1 }
        order:4, Point { x: 30, y: 0 }
        order:4, Point { x: 0, y: 30 }
        order:4, Point { x: 1, y: 0 }
        order:4, Point { x: 0, y: 1 }
        order:8, Point { x: 27, y: 4 }
        order:8, Point { x: 30, y: 0 }
        order:8, Point { x: 27, y: 27 }
        order:8, Point { x: 0, y: 30 }
        order:8, Point { x: 4, y: 27 }
        order:8, Point { x: 1, y: 0 }
        order:8, Point { x: 4, y: 4 }
        order:8, Point { x: 0, y: 1 }
        order:32, Point { x: 2, y: 20 }
        order:32, Point { x: 18, y: 24 }
        order:32, Point { x: 5, y: 10 }
        order:32, Point { x: 27, y: 4 }
        order:32, Point { x: 21, y: 26 }
        order:32, Point { x: 7, y: 13 }
        order:32, Point { x: 11, y: 29 }
        order:32, Point { x: 30, y: 0 }
        order:32, Point { x: 11, y: 2 }
        order:32, Point { x: 7, y: 18 }
        order:32, Point { x: 21, y: 5 }
        order:32, Point { x: 27, y: 27 }
        order:32, Point { x: 5, y: 21 }
        order:32, Point { x: 18, y: 7 }
        order:32, Point { x: 2, y: 11 }
        order:32, Point { x: 0, y: 30 }
        order:32, Point { x: 29, y: 11 }
        order:32, Point { x: 13, y: 7 }
        order:32, Point { x: 26, y: 21 }
        order:32, Point { x: 4, y: 27 }
        order:32, Point { x: 10, y: 5 }
        order:32, Point { x: 24, y: 18 }
        order:32, Point { x: 20, y: 2 }
        order:32, Point { x: 1, y: 0 }
        order:32, Point { x: 20, y: 29 }
        order:32, Point { x: 24, y: 13 }
        order:32, Point { x: 10, y: 26 }
        order:32, Point { x: 4, y: 4 }
        order:32, Point { x: 26, y: 10 }
        order:32, Point { x: 13, y: 24 }
        order:32, Point { x: 29, y: 20 }
        order:32, Point { x: 0, y: 1 }
        */

        // Order 2.
        {
            let p1 = clockcurve::Point { x: 0, y: 30 };
            let p2 = sub.curve.scalar_mul(p1, 2);
            assert_eq!(p2, sub.infinity);
        }

        // Order 4.
        {
            let p1 = clockcurve::Point { x: 1, y: 0 };
            let p2 = sub.curve.scalar_mul(p1, 4);
            assert_eq!(p2, sub.infinity);
        }

        // Order 8.
        {
            let p1 = clockcurve::Point { x: 27, y: 27 };
            let p2 = sub.curve.scalar_mul(p1, 8);
            assert_eq!(p2, sub.infinity);
        }

        {
            // https://monero.stackexchange.com/questions/4241/how-does-the-recent-patched-key-image-exploit-work-in-practice
            // Curve order is 32
            //
            let c = 20;
            // I(7,13) in the group order 32.
            let I = clockcurve::Point { x: 7, y: 13 };
            let cI = sub.curve.scalar_mul(I, c);
            println!("{}*{:?}(in group order [32]) = {:?}", c, I, cI);
            // 20*Point { x: 7, y: 13 }(in group order [32]) = Point { x: 1, y: 0 }

            // L(1,0) in the order (4,8,32).
            // c = 5 times order 4.
            let L = clockcurve::Point { x: 1, y: 0 };
            let cL = sub.curve.scalar_mul(L, c);
            println!("{}*{:?}(in group order [4,8,32]) = {:?}", c, L, cL);
            assert_eq!(sub.infinity, cL);
            // 20*Point { x: 1, y: 0 }(in group order [4,8,32]) = Point { x: 0, y: 1 }

            // c*U = c*L + c*I = 0 + c*I = c*I
            let U = sub.curve.scalar_add(L, I);
            let cU = sub.curve.scalar_mul(U, c);
            println!("U=(I+L): {:?}", U);
            // U = (I+L): Point { x: 13, y: 24 }
            println!("c*U = c*(I+L) = (0 + c*I) = c*I= {:?}", cU);
            // c*U = c*(I+L) = (0 + c*I) = c*I= Point { x: 1, y: 0 }
            assert_ne!(I, U);
            assert_eq!(cI, cU);
        }
    }
}
