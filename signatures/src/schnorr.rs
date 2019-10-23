// Copyright (c) BohuTANG
// Code is licensed with BSD

use algebra::arith;
use curves::clockcurve;
use subgroups::subgroup;

#[derive(Debug, Clone, Copy)]
pub struct Schnorr {
    pub group: subgroup::SubGroup,
}

impl Default for Schnorr {
    fn default() -> Self {
        Schnorr::new()
    }
}

impl Schnorr {
    pub fn new() -> Self {
        Schnorr {
            group: subgroup::SubGroup::default(),
        }
    }

    pub fn pubkey(&self, pk: i8) -> clockcurve::Point {
        self.group.curve.scalar_basemul(pk)
    }

    pub fn hash(&self, message: i8, x: i8) -> i8 {
        let prime = self.group.curve.prime;
        let mut y = self.group.curve.y(x).unwrap();
        if y > prime >> 1 {
            y = prime - y;
        }
        x ^ y ^ message
    }

    /// Returns signature with the param(message, private, random nonce).
    ///
    /// r = k*G
    /// s = k + hash(r,m)*pk
    /// # Examples
    ///
    /// ```rust
    /// use signatures::schnorr;
    ///
    /// fn main() {
    ///    let message = 10;
    ///    let private = 5;
    ///    let randomk = 7;
    ///    let signature = schnorr::Schnorr::new();
    ///    let (r, s) = signature.sign(message, private, randomk);
    ///    println!("signature: r:{:?},s:{}", r, s);
    ///    let pubkey = signature.pubkey(private);
    ///    let verify = signature.verify(message, pubkey, r, s);
    ///    assert_eq!(verify, true);
    /// }
    pub fn sign(&self, message: i8, private: i8, randomk: i8) -> (clockcurve::Point, i8) {
        let m = self.group.order();

        // r = (k*G)
        let r = self.group.scalar_basemul(randomk);
        assert!(r != self.group.curve.infinity);

        // e = hash(r|m)
        let e = self.hash(message, r.x);

        // s = k + e*pk
        let s = arith::mod_add(randomk, arith::mod_mul(e, private, m), m);
        (r, s)
    }

    /// Returns verify result.
    ///
    /// check s*G = r + hash(r,m)*P
    ///
    /// # Examples
    ///
    /// ```rust
    /// use signatures::schnorr;
    ///
    /// fn main() {
    ///    let message = 10;
    ///    let private = 5;
    ///    let randomk = 7;
    ///    let signature = schnorr::Schnorr::new();
    ///    let (r, s) = signature.sign(message, private, randomk);
    ///    println!("signature: r:{:?},s:{}", r, s);
    ///    let pubkey = signature.pubkey(private);
    ///    let verify = signature.verify(message, pubkey, r, s);
    ///    assert_eq!(verify, true);
    /// }
    pub fn verify(
        &self,
        message: i8,
        pubkey: clockcurve::Point,
        r: clockcurve::Point,
        s: i8,
    ) -> bool {
        let e = self.hash(message, r.x);

        // check s×G = R + hash(e)×P.
        let s1 = self.group.scalar_basemul(s);
        assert!(s1 != self.group.curve.infinity);
        let s2 = self
            .group
            .curve
            .scalar_add(r, self.group.curve.scalar_mul(pubkey, e));
        (s1 == s2)
    }

    #[allow(clippy::too_many_arguments)]
    /// Returns batch verify result.
    ///
    /// (s1+s2+…+s1000)×G=(r1+…+r1000)+(hash(r1,m1)×P1+ hash(r2,m2)×P2+…+hash(r1000,m1000)×P1000)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use signatures::schnorr;
    ///
    /// fn main() {
    ///    let (message1, message2) = (10, 11);
    ///    let (private1, private2) = (8, 9);
    ///    let (randomk1, randomk2) = (9, 10);
    ///    let signature = schnorr::Schnorr::new();
    ///    let (r1, s1) = signature.sign(message1, private1, randomk1);
    ///    let pubkey1 = signature.pubkey(private1);
    ///    let (r2, s2) = signature.sign(message2, private2, randomk2);
    ///    let pubkey2 = signature.pubkey(private2);
    ///    let verify = signature.batch_verify(message1, pubkey1, r1, s1, message2, pubkey2, r2, s2);
    ///    assert_eq!(verify, true);
    /// }
    pub fn batch_verify(
        &self,
        message1: i8,
        pubkey1: clockcurve::Point,
        r1: clockcurve::Point,
        s1: i8,
        message2: i8,
        pubkey2: clockcurve::Point,
        r2: clockcurve::Point,
        s2: i8,
    ) -> bool {
        let m = self.group.order();

        // (s1+s2+…+s1000)×G
        let ssum = arith::mod_add(s1, s2, m);
        let s = self.group.curve.scalar_basemul(ssum);

        // (r1+…+r1000)
        let r = self.group.curve.scalar_add(r1, r2);

        let s1 = self
            .group
            .curve
            .scalar_mul(pubkey1, self.hash(message1, r1.x));
        let s2 = self
            .group
            .curve
            .scalar_mul(pubkey2, self.hash(message2, r2.x));

        // (r1+…+r1000)+(hash(r,m1)×P1+ hash(r2,m2)×P2+…+hash(r1000,m1000)×P1000)
        let final_s = self
            .group
            .curve
            .scalar_add(r, self.group.curve.scalar_add(s1, s2));
        s == final_s
    }
}
