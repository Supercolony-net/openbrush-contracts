#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_crypto {
    use ink_lang::EthereumAddress;

    #[ink(storage)]
    #[derive(Default)]
    pub struct MyCrypto {}

    impl MyCrypto {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn ecdsa_recovery(&self) {
            let signature: [u8; 65] = [
                161, 234, 203,  74, 147, 96,  51, 212,   5, 174, 231,   9, 142,  48, 137, 201,
                162, 118, 192,  67, 239, 16,  71, 216, 125,  86, 167, 139,  70,   7,  86, 241,
                 33,  87, 154, 251,  81, 29, 160,   4, 176, 239,  88, 211, 244, 232, 232,  52,
                211, 234, 100, 115, 230, 47,  80,  44, 152, 166,  62,  50,   8,  13,  86, 175,
                 33,
            ];
            let message_hash: [u8; 32] = [
                162, 28, 244, 179, 96, 76, 244, 178, 188,  83, 230, 248, 143, 106,  77, 117,
                239, 95, 244, 171, 65, 95,  62, 153, 174, 166, 182,  28, 130,  73, 196, 208
            ];
            const EXPECTED_COMPRESSED_PUBLIC_KEY: [u8; 33] = [
                  2, 121, 190, 102, 126, 249, 220, 187, 172, 85, 160,  98, 149, 206, 135, 11,
                  7,   2, 155, 252, 219,  45, 206,  40, 217, 89, 242, 129,  91,  22, 248, 23,
                152,
            ];
            let pub_key = self.env().ecdsa_recover(&signature, &message_hash);
            ink_env::debug_print!("{:?}", pub_key);
            assert!(pub_key.is_ok());
            let pub_key = pub_key.unwrap();
            assert_eq!(*pub_key, EXPECTED_COMPRESSED_PUBLIC_KEY);

            const EXPECTED_ETH_ADDRESS: EthereumAddress = [
                126, 95, 69, 82, 9, 26, 105, 18, 93, 93, 252, 183, 184, 194, 101, 144, 41, 57, 91, 223
            ];

            assert_eq!(pub_key.to_eth_address(), EXPECTED_ETH_ADDRESS);
        }
    }
}
