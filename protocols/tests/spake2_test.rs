// Copyright (c) BohuTANG
// Code is licensed with BSD

#[allow(non_snake_case)]
#[cfg(test)]

pub mod tests {
    use curves::clockcurve;

    #[test]
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
    /// B4. Bob checks 𝑆 == 𝑇
    fn spake2_test() {
        // password.
        let w = 7;
        let clockcurve = clockcurve::ClockCurve::default();

        let m = 11;
        // 𝑀 = 𝑔^𝑚 𝑚𝑜𝑑 𝑝
        let M = clockcurve.scalar_basemul(m);

        let x = 66;
        // 𝑋 = 𝑔^𝑥 𝑚𝑜𝑑 𝑝
        let X = clockcurve.scalar_basemul(x);

        // 𝑇 = (𝑀^𝑤 + 𝑋)
        let wM = clockcurve.scalar_mul(M, w);
        let T = clockcurve.scalar_add(wM, X);

        let n = 23;
        // 𝑁 = 𝑔^𝑛 𝑚𝑜𝑑 𝑝
        let N = clockcurve.scalar_basemul(n);
        let y = 88;
        // 𝑌 = 𝑔^𝑦 𝑚𝑜𝑑 𝑝
        let Y = clockcurve.scalar_basemul(y);

        // 𝑆 = 𝑁^𝑤 + 𝑌
        let wN = clockcurve.scalar_mul(N, w);
        let S = clockcurve.scalar_add(wN, Y);

        // 𝐾(𝐵𝑜𝑏) = (𝑆 − 𝑁^𝑤)^𝑥 = (𝑁^𝑤 + 𝑌 − 𝑁^𝑤)^𝑥 = 𝑌^𝑥 = 𝑔^𝑥𝑦
        let SwN = clockcurve.scalar_sub(S, wN);
        let KBob = clockcurve.scalar_mul(SwN, x);

        // 𝐾(𝐴𝑙𝑖𝑐𝑒) = (𝑇 − 𝑀^𝑤)^𝑦 = (𝑀^𝑤 + 𝑋 − 𝑀^𝑤)^𝑦 = 𝑋^𝑦 = 𝑔^𝑥𝑦
        let TwM = clockcurve.scalar_sub(T, wM);
        let KAlice = clockcurve.scalar_mul(TwM, y);
        assert_eq!(KBob, KAlice);
    }
}
