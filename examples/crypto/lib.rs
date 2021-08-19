#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_crypto {
    use brush::crypto::ecdsa::*;
    use brush::crypto::hash_keccak_256;

    #[ink(storage)]
    #[derive(Default)]
    pub struct MyCrypto {}

    impl MyCrypto {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn verify(&self) {
            let message = "Hello my dear world";
            let message_hash = hash_keccak_256(message.as_bytes());
            let private_key = PrivateKey::default();
            let public_key = PublicKey::from_secret_key(&private_key);

            let signature = ecsign(&message_hash, &private_key, Some(13));
            let recovery_result = ecrecover(&message_hash, &signature, Some(13));
            // assert!(recovery_result.is_ok());
            // assert_eq!(recovery_result.unwrap().to_eth_address(), public_key.to_eth_address());
        }
    }
}
