// Copyright (c) BohuTANG
// Code is licensed with BSD

use algebra::arith;
use curves::clockcurve;
use subgroups::subgroup;

#[derive(Debug, Clone, Copy)]
pub struct ECDSA {
    pub group: subgroup::SubGroup,
}

impl Default for ECDSA {
    fn default() -> Self {
        ECDSA::new()
    }
}

impl ECDSA {
    pub fn new() -> Self {
        ECDSA {
            group: subgroup::SubGroup::default(),
        }
    }

    pub fn pubkey(self, pk: i8) -> clockcurve::Point {
        self.group.curve.scalar_basemul(pk)
    }

    pub fn sign(self, message: i8, private: i8, randomk: i8) -> (i8, i8) {
        let m = self.group.order();

        // z = hash(message), we use the raw message as hash.
        let z = message;

        // r = (k*G).x
        let r = self.group.scalar_basemul(randomk).x;

        // kinverse = 1/randomk
        let kinverse = arith::mod_div(1, randomk, m);
        // s = (z + r*pk)/k
        let s = arith::mod_mul(
            arith::mod_add(z, arith::mod_mul(r, private, m), m),
            kinverse,
            m,
        );
        (r, s)
    }

    pub fn verify(self, message: i8, pubkey: clockcurve::Point, r: i8, s: i8) -> bool {
        let m = self.group.order();

        // z = hash(message), we use the raw message as hash.
        let z = message;

        // sinverse = 1/s
        let sinverse = arith::mod_div(1, s, m);

        // (z/s)*G
        let p1 = self.group.scalar_basemul(arith::mod_mul(z, sinverse, m));
        // (r/s)*P
        let p2 = self
            .group
            .curve
            .scalar_mul(pubkey, arith::mod_mul(r, sinverse, m));

        // check r == ((z/s)*G + (r/s)*P).x
        self.group.curve.scalar_add(p1, p2).x == r
    }
}
