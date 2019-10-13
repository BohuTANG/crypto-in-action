// Copyright (c) BohuTANG
// Code is licensed with BSD

use fields::field;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

/// Clock curve with Fp31.
/// Equation:
/// x^2 + y^2 = 1 over Fp31.
#[derive(Debug, Copy, Clone)]
pub struct ClockCurve {
    pub b: i8,
    pub base: Point,
    pub neutral: Point,
    pub primer: i8,
    pub field: field::Field,
}

impl Default for ClockCurve {
    fn default() -> Self {
        ClockCurve {
            b: 1,
            primer: 31,
            base: Point { x: 2, y: 20 },
            neutral: Point { x: 0, y: 1 },
            field: field::Field::new(31),
        }
    }
}

impl ClockCurve {
    /// Returns the sum of (x1,y1) and (x2,y2).
    ///
    /// ```text
    /// x^2 + y^2 = 1 addtion formual:
    /// (x3,y3) = (x1,y1) + (x2,y2) = (x1*y2 + x2*y1,y2*y1 - x1*x2)
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::clockcurve;
    ///
    /// fn main() {
    ///     let curve = clockcurve::ClockCurve::default();
    ///     let p1 = clockcurve::Point { x: 1, y: 0 };
    ///     let p2 = clockcurve::Point { x: 0, y: 1 };
    ///     let p3 = curve.scalar_add(p1, p2);
    ///     println!("{:?}", p3);
    /// }
    /// ```
    pub fn scalar_add(self, p1: Point, p2: Point) -> Point {
        let x1y2 = self.field.mul(p1.x, p2.y);
        let x2y1 = self.field.mul(p2.x, p1.y);
        let x3 = self.field.add(x1y2, x2y1);

        let y2y1 = self.field.mul(p2.y, p1.y);
        let x1x2 = self.field.mul(p1.x, p2.x);
        let y3 = self.field.sub(y2y1, x1x2);
        Point { x: x3, y: y3 }
    }

    /// Returns the sum of (x1,y1) and (x2,-y2).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::clockcurve;
    ///
    /// fn main() {
    ///     let curve = clockcurve::ClockCurve::default();
    ///     let p1 = clockcurve::Point { x: 1, y: 0 };
    ///     let p2 = clockcurve::Point { x: 0, y: 1 };
    ///     let p3 = curve.scalar_sub(p1, p2);
    ///     println!("{:?}", p3);
    /// }
    /// ```
    pub fn scalar_sub(self, p1: Point, p2: Point) -> Point {
        self.scalar_add(p1, self.point_neg(p2))
    }

    /// Returns the neg(x1,y1) = (-x1,y1).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::clockcurve;
    ///
    /// fn main() {
    ///     let curve = clockcurve::ClockCurve::default();
    ///     let p1 = clockcurve::Point { x: 2, y: 20 };
    ///     let pneg = curve.point_neg(p1);
    ///     println!("{:?}", pneg);
    /// }
    /// ```
    pub fn point_neg(self, p1: Point) -> Point {
        Point {
            x: self.field.sub(self.primer, p1.x),
            y: p1.y,
        }
    }

    /// Returns the sum of (x1,y1) and (x1,y1).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::clockcurve;
    ///
    /// fn main() {
    ///     let curve = clockcurve::ClockCurve::default();
    ///     let p1 = clockcurve::Point { x: 1, y: 0 };
    ///     let pp = curve.scalar_double(p1);
    ///     println!("{:?}", pp);
    /// }
    /// ```
    pub fn scalar_double(self, p: Point) -> Point {
        self.scalar_add(p, p)
    }

    /// Returns k*(x1,y1) where k is interge using Montgomery ladder for constant time.
    /// https://en.wikipedia.org/wiki/Elliptic_curve_point_multiplication#Montgomery_ladder
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::clockcurve;
    ///
    /// fn main() {
    ///     let curve = clockcurve::ClockCurve::default();
    ///     let p1 = clockcurve::Point { x: 2, y: 20 };
    ///     let p2 = curve.scalar_mul(p1, 3);
    ///     println!("{:?}", p2);
    /// }
    /// ```
    pub fn scalar_mul(self, p: Point, k: i8) -> Point {
        assert!(k != 0);
        let mut r0 = self.neutral;
        let mut r1 = Point { x: p.x, y: p.y };

        let mut bits = 0;
        let mut k1 = k;
        while k1 > 0 {
            bits += 1;
            k1 >>= 1;
        }

        for i in (0..bits).rev() {
            if (k >> i) & 0x01 == 0 {
                r1 = self.scalar_add(r0, r1);
                r0 = self.scalar_double(r0);
            } else {
                r0 = self.scalar_add(r0, r1);
                r1 = self.scalar_double(r1);
            }
        }
        r0
    }

    /// Returns k*(base point) where k is integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::clockcurve;
    ///
    /// fn main() {
    ///     let curve = clockcurve::ClockCurve::default();
    ///     let p1 = clockcurve::Point { x: 2, y: 20 };
    ///     let p2 = curve.scalar_mul(p1, 3);
    ///     println!("{:?}", p2);
    /// }
    /// ```
    pub fn scalar_basemul(self, k: i8) -> Point {
        self.scalar_mul(self.base, k)
    }

    ///  Checks the point p is on the curve or not.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::clockcurve;
    ///
    /// fn main() {
    ///     let curve = clockcurve::ClockCurve::default();
    ///     let p1 = clockcurve::Point { x: 2, y: 20 };
    ///     let res = curve.is_on_curve(p1);
    ///     println!("{:?}", res);
    /// }
    /// ```
    pub fn is_on_curve(self, p: Point) -> bool {
        let xx = self.field.mul(p.x, p.x);
        let yy = self.field.mul(p.y, p.y);
        self.field.add(xx, yy) == self.b
    }

    ///  Returns y coordinate if exists, otherwise None.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::clockcurve;
    ///
    /// fn main() {
    ///     let curve = clockcurve::ClockCurve::default();
    ///     let y = curve.y(2);
    ///     println!("{:?}", y);
    /// }
    /// ```
    pub fn y(self, x: i8) -> Option<i8> {
        let xx = self.field.mul(x, x);
        let yy = self.primer - xx + self.b;
        self.field.sqrt(yy)
    }
}
