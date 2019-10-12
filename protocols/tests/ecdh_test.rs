// Copyright (c) BohuTANG
// Code is licensed with BSD

#[allow(non_snake_case)]
#[cfg(test)]

pub mod tests {
    use curves::clockcurve;

    #[test]
    /// Elliptic Curve Diffie-Hellman (ECDH)
    /// 1. Alice selects a as secert key and calculates 𝐴 = 𝑔^𝑎 mod 𝑝
    /// 2. Bob b selects b as secert key and calculates 𝐵 = 𝑔^𝑏 mod 𝑝
    /// 3. Alice sends Bob A
    /// 4. Bob sends Alice B
    /// 5. Now Alice calculates 𝑠 = 𝐵^𝑎 mod 𝑝
    /// 6. Now Bob calculates   𝑠 = 𝐴^𝑏 mod 𝑝
    fn ecdh_test() {
        let a = 8;
        let b = 66;
        let clockcurve = clockcurve::ClockCurve::default();

        let A = clockcurve.scalar_basemul(a);
        let B = clockcurve.scalar_basemul(b);
        println!("{:?}, {:?}", A, B);
        // Point { x: 30, y: 0 }, Point { x: 18, y: 24 }

        let sa = clockcurve.scalar_mul(A, b);
        let sb = clockcurve.scalar_mul(B, a);

        println!("{:?}, {:?}", sa, sb);
        // Point { x: 0, y: 30 }, Point { x: 0, y: 30 }

        assert_eq!(sa, sb);
    }
}
