// Copyright (c) BohuTANG
// Code is licensed with BSD

#[cfg(test)]
pub mod tests {
    use curves::clockcurve;
    use curves::keys;

    #[test]
    fn curves_keys_test() {
        let privatekey = keys::PrivateKey::new(2);
        let publickey = privatekey.publickey();
        assert_eq!(publickey.point, clockcurve::Point { x: 18, y: 24 });
        println!("({:?},{:?})", privatekey.serialize(), publickey.serialize());
        // ([0, 2],[18, 24])
    }
}
