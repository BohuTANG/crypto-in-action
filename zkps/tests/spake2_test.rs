// Copyright (c) BohuTANG
// Code is licensed with BSD

#[allow(non_snake_case)]
#[cfg(test)]

pub mod tests {
    use zkps::spake2;

    #[test]
    fn zkps_spake2_test() {
        let password = 7;

        // Alice.
        let alice_rand_number = 11;
        let alice_secret_number = 66;
        let alice = spake2::SPAKE2::new(password, alice_rand_number, alice_secret_number);
        let alice_pake_key = alice.pake_key();

        // Bob.
        let bob_rand_number = 23;
        let bob_secret_number = 88;
        let bob = spake2::SPAKE2::new(password, bob_rand_number, bob_secret_number);
        let bob_pake_key = bob.pake_key();

        // Check the final key.
        let alice_final_key = alice.final_key(bob_pake_key, bob.password_pubkey);
        let bob_final_key = bob.final_key(alice_pake_key, alice.password_pubkey);
        assert_eq!(alice_final_key, bob_final_key);
    }
}
