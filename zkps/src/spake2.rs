// Copyright (c) BohuTANG
// Code is licensed with BSD

use curves::clockcurve;

#[derive(Debug)]
pub struct SPAKE2 {
    password: i8,
    secret: i8,
    random: i8,
    secret_pubkey: clockcurve::Point,
    pub password_pubkey: clockcurve::Point,
    pub curve: clockcurve::ClockCurve,
}

/// SPAKE2 Protocol
/// Simple Password-Authenticated Key Exchange
/// A1. Alice selects a random scalar 𝑀 = 𝑔^𝑚 𝑚𝑜𝑑 𝑝
/// A2. Alice selects 𝑥 as secert key and calculates 𝑔^𝑦 𝑚𝑜𝑑 𝑝
/// A3. Alice then computes 𝑇 = (𝑀^𝑤 + 𝑋)
/// A4. Alice send 𝑇 to Bob.
///
/// B1. Bob selects a random scalar 𝑁 = 𝑔^𝑛 𝑚𝑜𝑑 𝑝
/// B2. Bob selects 𝑦 as secert key and calculates 𝑌 = 𝑔^𝑦 𝑚𝑜𝑑 𝑝
/// B3. Bob then computes 𝑆 = 𝑁^𝑤 + 𝑌
///
/// Alice calculates 𝐾(𝐴𝑙𝑖𝑐𝑒) = (𝑆 − 𝑁^𝑤)^𝑥
/// Bob calculates   𝐾(𝐵𝑜𝑏) = (𝑇 − 𝑀^𝑤)^𝑦
/// Check 𝐾(𝐴𝑙𝑖𝑐𝑒) == 𝐾(𝐵𝑜𝑏)
/// 𝐾(𝐴𝑙𝑖𝑐𝑒) = (𝑆 − 𝑁^𝑤)^𝑥 = (𝑁^𝑤 + 𝑌 − 𝑁^𝑤) = 𝑌^𝑥 = 𝑔^𝑥𝑦 𝑚𝑜𝑑 𝑝
/// 𝐾(𝐵𝑜𝑏) = (𝑇 − 𝑀^𝑤)^𝑦 = (𝑀^𝑤 + 𝑋 − 𝑁^𝑤)^𝑦 = 𝑋^𝑦 = 𝑔^𝑥𝑦
impl SPAKE2 {
    pub fn new(pwd: i8, rnd: i8, sec: i8) -> Self {
        let curv = clockcurve::ClockCurve::default();
        let sec_pubkey = curv.scalar_basemul(sec);
        let rand_pubkey = curv.scalar_basemul(rnd);
        let pwd_pubkey = curv.scalar_mul(rand_pubkey, pwd);
        SPAKE2 {
            password: pwd,
            secret: sec,
            random: rnd,
            secret_pubkey: sec_pubkey,
            password_pubkey: pwd_pubkey,
            curve: curv,
        }
    }

    /// Returns the first round pake key.
    /// 𝐾(𝐴𝑙𝑖𝑐𝑒) = (𝑆 − 𝑁^𝑤)^𝑥
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zkps::spake2;
    ///
    /// fn main() {
    ///     let password = 7;
    ///     let alice_rand_number = 11;
    ///     let alice_secret_number = 66;
    ///     let alice = spake2::SPAKE2::new(password, alice_rand_number, alice_secret_number);
    ///     let alice_pake_key = alice.pake_key();
    ///     println!("{:?}", alice_pake_key);
    /// }
    /// ```
    pub fn pake_key(&self) -> clockcurve::Point {
        self.curve
            .scalar_add(self.password_pubkey, self.secret_pubkey)
    }

    /// Returns the final round authenticated key.
    /// 𝐾(𝐴𝑙𝑖𝑐𝑒) = (𝑆 − 𝑁^𝑤)^𝑥
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zkps::spake2;
    ///
    /// fn main() {
    ///     let password = 7;
    ///     let alice_rand_number = 11;
    ///     let alice_secret_number = 66;
    ///     let alice = spake2::SPAKE2::new(password, alice_rand_number, alice_secret_number);
    ///     let alice_pake_key = alice.pake_key();
    ///     let bob_rand_number = 23;
    ///     let bob_secret_number = 88;
    ///     let bob = spake2::SPAKE2::new(password, bob_rand_number, bob_secret_number);
    ///     let bob_pake_key = bob.pake_key();
    ///     let alice_final_key = alice.final_key(bob_pake_key, bob.password_pubkey);
    ///     println!("{:?}", alice_final_key);
    /// }
    /// ```
    pub fn final_key(
        &self,
        pakekey: clockcurve::Point,
        password_pubkey: clockcurve::Point,
    ) -> clockcurve::Point {
        self.curve
            .scalar_mul(self.curve.scalar_sub(pakekey, password_pubkey), self.secret)
    }
}
