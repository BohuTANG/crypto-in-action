// Copyright (c) BohuTANG
// Code is licensed with BSD

use curves::clockcurve;

#[derive(Debug)]
pub struct SubGroup {
    pub infinity: clockcurve::Point,
    pub basepoint: clockcurve::Point,
    pub points: Vec<clockcurve::Point>,
    pub curve: clockcurve::ClockCurve,
}

impl SubGroup {
    pub fn new(g: clockcurve::Point) -> Self {
        let curv = clockcurve::ClockCurve::default();
        assert!(curv.is_on_curve(g));
        let mut sub = SubGroup {
            infinity: clockcurve::Point { x: 0, y: 1 },
            basepoint: g,
            points: Vec::new(),
            curve: curv,
        };
        sub.generate();
        sub
    }

    fn generate(&mut self) {
        self.points.push(self.basepoint);
        for i in 2..=self.curve.primer + 1 {
            let p = self.curve.scalar_mul(self.basepoint, i);
            if p == self.basepoint {
                break;
            }
            self.points.push(p);
        }
    }

    pub fn scalar_basemul(&self, a: i8) -> clockcurve::Point {
        self.curve.scalar_mul(self.basepoint, a)
    }
    pub fn points(&self) -> Vec<clockcurve::Point> {
        self.points.clone()
    }

    pub fn order(&self) -> i8 {
        self.points.len() as i8
    }
}
