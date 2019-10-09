// Copyright (c) BohuTANG
// Code is licensed with BSD

use fields::field;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

/// Clock curve with Fp31.
/// x^2 + y^2 = 1
#[derive(Debug)]
pub struct Curve31 {
    b: i8,
    primer: i8,
    base: Point,
    field: field::Field,
}

impl Default for Curve31 {
    fn default() -> Self {
        Curve31 {
            b: 1,
            primer: 31,
            base: Point { x: 2, y: 20 },
            field: field::Field::new(31),
        }
    }
}

impl Curve31 {
    ///  Returns the sum of (x1,y1) and (x2,y2).
    ///
    /// ```text
    /// x^2 + y^2 = 1 addtion formual:
    /// (x3,y3) = (x1,y1) + (x2 + y2) = (x1*y2 + x2*y1,y2*y1 - x1*x2)
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::curve31;
    ///
    /// fn main() {
    ///     let curve = curve31::Curve31::default();
    ///     let p1 = curve31::Point { x: 1, y: 0 };
    ///     let p2 = curve31::Point { x: 0, y: 1 };
    ///     let p3 = curve.scalar_add(p1, p2);
    ///     println!("{:?}", p3);
    /// }
    /// ```
    ///
    pub fn scalar_add(&self, p1: Point, p2: Point) -> Point {
        let x1y2 = self.field.mul(p1.x, p2.y);
        let x2y1 = self.field.mul(p2.x, p1.y);
        let x3 = self.field.add(x1y2, x2y1);

        let y2y1 = self.field.mul(p2.y, p1.y);
        let x1x2 = self.field.mul(p1.x, p2.x);
        let y3 = self.field.sub(y2y1, x1x2);
        Point { x: x3, y: y3 }
    }

    ///  Returns k*(x1,y1) where k is integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::curve31;
    ///
    /// fn main() {
    ///     let curve = curve31::Curve31::default();
    ///     let p1 = curve31::Point { x: 2, y: 20 };
    ///     let p2 = curve.scalar_mul(p1, 3);
    ///     println!("{:?}", p2);
    /// }
    /// ```
    ///
    pub fn scalar_mul(&self, p: Point, k: i8) -> Point {
        assert!(k != 0);
        let mut r = Point { x: p.x, y: p.y };
        if k == 1 {
            r
        } else {
            for _ in 1..k {
                r = self.scalar_add(r, p);
            }
            r
        }
    }

    ///  Returns k*(base point) where k is integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::curve31;
    ///
    /// fn main() {
    ///     let curve = curve31::Curve31::default();
    ///     let p1 = curve31::Point { x: 2, y: 20 };
    ///     let p2 = curve.scalar_mul(p1, 3);
    ///     println!("{:?}", p2);
    /// }
    /// ```
    ///
    pub fn scalar_basemul(&self, k: i8) -> Point {
        self.scalar_mul(self.base, k)
    }

    ///  Returns y coordinate if exists, otherwise None.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::curve31;
    ///
    /// fn main() {
    ///     let curve = curve31::Curve31::default();
    ///     let y = curve.y(2);
    ///     println!("{:?}", y);
    /// }
    /// ```
    ///
    pub fn y(&self, x: i8) -> Option<i8> {
        let xx = self.field.mul(x, x);
        let yy = self.primer - xx + self.b;
        self.field.sqrt(yy)
    }
}
