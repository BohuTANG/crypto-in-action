// Copyright (c) BohuTANG
// Code is licensed with BSD

use crate::clockcurve;

#[derive(Clone, Copy, Debug)]
pub struct PublicKey {
    pub point: clockcurve::Point,
    pub curve: clockcurve::ClockCurve,
}

impl PublicKey {
    /// Returns the serialize format of the public key.
    /// ```text
    /// [0] -- x
    /// [1] -- y
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::keys;
    ///
    /// fn main() {
    ///     let privatekey = keys::PrivateKey::new(2);
    ///     let publickey = privatekey.publickey();
    ///     println!("{:?}", publickey.serialize());
    /// }
    pub fn serialize(self) -> [i8; 2] {
        [self.point.x, self.point.y]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PrivateKey {
    pub key: i8,
    publickey: PublicKey,
    curve: clockcurve::ClockCurve,
}

impl PrivateKey {
    pub fn new(k: i8) -> Self {
        let cuv31 = clockcurve::ClockCurve::default();
        let p = cuv31.scalar_basemul(k);
        PrivateKey {
            key: k,
            curve: cuv31,
            publickey: PublicKey {
                point: p,
                curve: cuv31,
            },
        }
    }

    pub fn publickey(&self) -> PublicKey {
        self.publickey
    }

    /// Returns the serialize format of the private key.
    /// ```text
    /// [0] -- null byte
    /// [1] -- key byte
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use curves::keys;
    ///
    /// fn main() {
    ///     let privatekey = keys::PrivateKey::new(2);
    ///     println!("{:?}", privatekey.serialize());
    /// }
    pub fn serialize(&self) -> [i8; 2] {
        [0x00, self.key]
    }
}
