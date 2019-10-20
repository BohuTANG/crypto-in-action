// Copyright (c) BohuTANG
// Code is licensed with BSD

#[allow(non_snake_case)]
#[cfg(test)]

pub mod tests {
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
}
