// Copyright (c) BohuTANG
// Code is licensed with BSD

use algebra::arith;

/// Field with prime.
#[derive(Debug, Copy, Clone)]
pub struct Field {
    primer: i8,
}

impl Field {
    pub fn new(p: i8) -> Self {
        Field { primer: p }
    }

    ///  Computes two numbers sum in finite field.
    ///
    /// ```text
    /// a + b (mod primer)
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fields::field;
    ///
    /// fn main() {
    ///     let fp = field::Field::new(37);
    ///     println!("{:?}", fp.add(15,33));
    /// }
    /// ```
    pub fn add(self, a: i8, b: i8) -> i8 {
        arith::mod_add(a, b, self.primer)
    }

    ///  Computes two numbers sum in finite field.
    ///
    /// ```text
    /// a - b (mod primer)
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fields::field;
    ///
    /// fn main() {
    ///     let fp = field::Field::new(37);
    ///     println!("{:?}", fp.sub(15,33));
    /// }
    /// ```
    pub fn sub(self, a: i8, b: i8) -> i8 {
        arith::mod_sub(a, b, self.primer)
    }

    ///  Computes two numbers product in finite field.
    ///
    /// ```text
    /// a * b (mod primer)
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fields::field;
    ///
    /// fn main() {
    ///     let fp = field::Field::new(37);
    ///     println!("{:?}", fp.mul(15,33));
    /// }
    /// ```
    pub fn mul(self, a: i8, b: i8) -> i8 {
        arith::mod_mul(a, b, self.primer)
    }

    ///  Computes the quadratic residue (mod primer) of 'a'.
    ///
    /// ```text
    /// Returns None if has no squre root.
    /// ```
    ///
    /// ```text
    /// Solve the congruence of the form:
    /// x^2 = a (mod p)
    /// Note that p - x is also a root.
    /// Here, we only handle the case that primer%4 ==3.
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fields::field;
    ///
    /// fn main() {
    ///     let fp = field::Field::new(31);
    ///     println!("{:?}", fp.sqrt(28));
    /// }
    /// ```
    pub fn sqrt(self, a: i8) -> Option<i8> {
        let a = a % self.primer;
        if a == 0 {
            return Some(0);
        } else if self.primer == 2 {
            return Some(self.primer);
        } else if self.primer % 4 != 3 {
            return None;
        }

        let ls = self.legendre_symbol(a);
        match ls {
            1 => Some(arith::mod_exp(a, (self.primer + 1) / 4, self.primer)),
            _ => None,
        }
    }

    ///  Computes the Legendre symbol a|p using Euler's criterion.
    ///
    /// ```text
    /// Returns 1 if a has a square root modulo p, -1 otherwise.
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fields::field;
    ///
    /// fn main() {
    ///     let fp = field::Field::new(31);
    ///     println!("{:?}", fp.legendre_symbol(28));
    /// }
    /// ```
    pub fn legendre_symbol(self, a: i8) -> i8 {
        let p1 = self.primer - 1;
        let ls = arith::mod_exp(a, p1 / 2, self.primer);
        if ls == p1 {
            -1
        } else {
            ls
        }
    }
}
