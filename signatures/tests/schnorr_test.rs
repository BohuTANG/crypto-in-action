// Copyright (c) BohuTANG
// Code is licensed with BSD

#[allow(non_snake_case)]
#[cfg(test)]

pub mod tests {
    use algebra::arith;
    use signatures::schnorr;

    #[test]
    fn signatures_schnorr_test() {
        let message = 10;
        let private = 5;
        let randomk = 7;

        let signature = schnorr::Schnorr::new();
        let (r, s) = signature.sign(message, private, randomk);
        println!("signature: r:{:?},s:{}", r, s);
        // signature: r:Point { x: 11, y: 29 },s:22

        let pubkey = signature.pubkey(private);
        let verify = signature.verify(message, pubkey, r, s);
        assert_eq!(verify, true);
    }

    #[test]
    fn signatures_schnorr_batch_verify_test() {
        let (message1, message2) = (10, 11);
        let (private1, private2) = (8, 9);
        let (randomk1, randomk2) = (9, 10);

        let signature = schnorr::Schnorr::new();
        let (r1, s1) = signature.sign(message1, private1, randomk1);
        println!(
            "private:{}, randomk:{}, message:{}, signature: <r:{:?},s:{}>",
            private1, randomk1, message1, r1, s1
        );
        // private:8, randomk:9, message:10, signature: <r:Point { x: 11, y: 2 },s:1>

        let pubkey1 = signature.pubkey(private1);
        let verify1 = signature.verify(message1, pubkey1, r1, s1);
        assert_eq!(verify1, true);

        let (r2, s2) = signature.sign(message2, private2, randomk2);
        println!(
            "private:{}, randomk:{}, message:{}, signature: <r:{:?},s:{}>",
            private2, randomk2, message2, r2, s2
        );
        // private:9, randomk:10, message:11, signature: <r:Point { x: 7, y: 18 },s:19>

        let pubkey2 = signature.pubkey(private2);
        let verify2 = signature.verify(message2, pubkey2, r2, s2);
        assert_eq!(verify2, true);

        let verify = signature.batch_verify(message1, pubkey1, r1, s1, message2, pubkey2, r2, s2);
        assert_eq!(verify, true);
    }

    #[test]
    /// s1 = k+hash(r|m1)*pk
    /// s2 = k+hash(r|m2)*pk
    /// pk = (s1 - s2)/(hash(r|m1) - hash(r|m2))
    /// https://en.wikipedia.org/wiki/Schnorr_signature#Key_leakage_from_nonce_reuse
    fn signatures_schnorr_key_leakage_from_nonce_reuse_test() {
        let randomk = 9;
        let private = 8;
        let (message1, message2) = (10, 11);

        let signature = schnorr::Schnorr::new();
        let (r1, s1) = signature.sign(message1, private, randomk);
        println!(
            "private:{}, randomk:{}, message:{}, signature: <r:{:?},s:{}>",
            private, randomk, message1, r1, s1
        );
        // private:8, randomk:9, message:10, signature: <r:Point { x: 11, y: 2 },s:1>

        let (r2, s2) = signature.sign(message2, private, randomk);
        println!(
            "private:{}, randomk:{}, message:{}, signature: <r:{:?},s:{}>",
            private, randomk, message2, r2, s2
        );
        // private:8, randomk:9, message:11, signature: <r:Point { x: 11, y: 2 },s:25>

        // pk = (s1 - s2)/(hash(r|m1) - hash(r|m2))
        let m = signature.group.order();
        let h1h2 = arith::mod_sub(
            signature.hash(message1, r1.x),
            signature.hash(message2, r2.x),
            m,
        );
        let s1s2 = arith::mod_sub(s1, s2, m);
        let x = arith::mod_div(s1s2, h1h2, m);

        assert_eq!(private, x);
    }
}
