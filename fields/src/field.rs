// Copyright (c) BohuTANG
// Code is licensed with BSD

use algebra::arith;

/// Field with prime.
#[derive(Debug)]
pub struct Field {
    prime: i8,
}

impl Field {
    pub fn new(p: i8) -> Field {
        Field { prime: p }
    }

    ///  The function `add` computes two numbers sum in finite field.
    ///
    /// ```text
    /// a + b (mod prime)
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fields::field;
    ///
    /// fn main() {
    ///     let fp37 = field::Field::new(37);
    ///     println!("{:?}", fp37.add(15,33));
    /// }
    /// ```
    ///
    pub fn add(&self, a: i8, b: i8) -> i8 {
        return arith::mod_add(a, b, self.prime);
    }

    ///  The function `mul` computes two numbers product in finite field.
    ///
    /// ```text
    /// a * b (mod prime)
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fields::field;
    ///
    /// fn main() {
    ///     let fp37 = field::Field::new(37);
    ///     println!("{:?}", fp37.mul(15,33));
    /// }
    /// ```
    ///
    pub fn mul(&self, a: i8, b: i8) -> i8 {
        return arith::mod_mul(a, b, self.prime);
    }
}
