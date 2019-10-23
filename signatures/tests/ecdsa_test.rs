// Copyright (c) BohuTANG
// Code is licensed with BSD

#[allow(non_snake_case)]
#[cfg(test)]

pub mod tests {
    use algebra::arith;
    use signatures::ecdsa;

    #[test]
    fn signatures_ecdsa_test() {
        let message = 10;
        let private = 5;
        let randomk = 7;

        let ecd = ecdsa::ECDSA::new();
        let (r, s) = ecd.sign(message, private, randomk);
        println!("signature: r:{},s:{}", r, s);
        // signature: r:11,s:23

        let pubkey = ecd.pubkey(private);
        let verify = ecd.verify(message, pubkey, r, s);
        assert_eq!(verify, true);
    }

    #[test]
    /// s1 = (H(m1) + r1*x1) / k
    /// s2 = (H(m2) + r2*x2) / k
    /// r1 = r2 and x1 = x2
    /// k = (H(m1) - H(m2)) / (s1 - s2)
    /// x1 = ((k * s1) - H(m1)) / r1
    ///
    /// https://crypto.stackexchange.com/questions/71764/is-it-safe-to-reuse-a-ecdsa-nonce-for-two-signatures-if-the-public-keys-are-diff
    fn signatures_ecdsa_key_leakage_from_nonce_reuse_test() {
        let randomk = 7;
        let private = 5;
        let (message1, message2) = (8, 9);

        let ecd = ecdsa::ECDSA::new();
        let (r1, s1) = ecd.sign(message1, private, randomk);
        println!(
            "private:{}, randomk:{}, message:{}, signature: <r:{},s:{}>",
            private, randomk, message1, r1, s1
        );
        // private:5, randomk:7, message:8, signature: <r:11,s:9>

        let (r2, s2) = ecd.sign(message2, private, randomk);
        println!(
            "private:{}, randomk:{}, message:{}, signature: <r:{},s:{}>",
            private, randomk, message2, r2, s2
        );
        // private:5, randomk:7, message:9, signature: <r:11,s:0>

        // k = (H(m1) – H(m2)) / (s1 – s2)
        let m = ecd.group.order();
        let h1h2 = arith::mod_sub(message1, message2, m);
        let s1s2 = arith::mod_sub(s1, s2, m);
        let k = arith::mod_div(h1h2, s1s2, m);

        assert_eq!(randomk, k);

        // x1 = ((k * s1) - H(m1)) / r1
        let x1 = arith::mod_div(arith::mod_sub(arith::mod_mul(s1, k, m), message1, m), r1, m);
        assert_eq!(private, x1);
    }
}
