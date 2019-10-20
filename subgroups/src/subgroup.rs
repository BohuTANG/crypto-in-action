// Copyright (c) BohuTANG
// Code is licensed with BSD

use curves::clockcurve;

#[derive(Debug, Clone, Copy)]
pub struct SubGroup {
    pub infinity: clockcurve::Point,
    pub basepoint: clockcurve::Point,
    pub curve: clockcurve::ClockCurve,
}

impl Default for SubGroup {
    fn default() -> Self {
        SubGroup::new(clockcurve::Point { x: 2, y: 20 })
    }
}

impl SubGroup {
    pub fn new(g: clockcurve::Point) -> Self {
        let curv = clockcurve::ClockCurve::default();
        assert!(curv.is_on_curve(g));
        SubGroup {
            infinity: clockcurve::Point { x: 0, y: 1 },
            basepoint: g,
            curve: curv,
        }
    }

    /// Returns k*(subgroup base point) where k is integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::clockcurve;
    /// use subgroups::subgroup;
    ///
    /// fn main() {
    ///     let g = clockcurve::Point { x: 1, y: 0 };
    ///     let sub = subgroup::SubGroup::new(g);
    ///     let p = sub.scalar_basemul(8);
    ///     println!("{:?}", p);
    /// }
    /// ```
    pub fn scalar_basemul(&self, a: i8) -> clockcurve::Point {
        self.curve.scalar_mul(self.basepoint, a)
    }

    /// Returns all points of the subgroup.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::clockcurve;
    /// use subgroups::subgroup;
    ///
    /// fn main() {
    ///     let g = clockcurve::Point { x: 1, y: 0 };
    ///     let sub = subgroup::SubGroup::new(g);
    ///     let points = sub.points();
    ///     println!("{:?}", points);
    /// }
    /// ```
    pub fn points(&self) -> Vec<clockcurve::Point> {
        let mut points: Vec<clockcurve::Point> = Vec::new();
        points.push(self.basepoint);
        for i in 2..=self.curve.prime + 1 {
            let p = self.curve.scalar_mul(self.basepoint, i);
            if p == self.basepoint {
                break;
            }
            points.push(p);
        }
        points.clone()
    }

    /// Returns the order of the subgroup.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::clockcurve;
    /// use subgroups::subgroup;
    ///
    /// fn main() {
    ///     let g = clockcurve::Point { x: 1, y: 0 };
    ///     let sub = subgroup::SubGroup::new(g);
    ///     let order = sub.order();
    ///     println!("{:?}", order);
    /// }
    /// ```
    pub fn order(&self) -> i8 {
        let mut points: Vec<clockcurve::Point> = Vec::new();
        points.push(self.basepoint);
        for i in 2..=self.curve.prime + 1 {
            let p = self.curve.scalar_mul(self.basepoint, i);
            if p == self.basepoint {
                break;
            }
            points.push(p);
        }

        points.len() as i8
    }
}
