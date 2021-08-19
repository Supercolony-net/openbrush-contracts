use secp256k1::{SecretKey, PublicKey as AffinePublicKey, Signature as ECDSASignature, RecoveryId, Message, recover, sign};
use crate::{hash_keccak_256, hash_blake2b_256};
use ::ink_env::{
    DefaultEnvironment,
    Environment,
};

pub type AccountId = <DefaultEnvironment as Environment>::AccountId;

pub type ETHAddress = [u8; 20];

pub trait PublicKeyExt {
    fn to_eth_address(&self) -> ETHAddress;
    fn to_account_id(&self) -> AccountId;
}

/// The ECDSA compressed public key.
pub type PublicKey = AffinePublicKey;

impl PublicKeyExt for PublicKey {
    fn to_eth_address(&self) -> ETHAddress {
        let uncompressed = self.serialize();
        let hash = hash_keccak_256(&uncompressed[1..]);
        let mut result = ETHAddress::default();
        result.iter_mut().enumerate().for_each(|(i, byte)| *byte = hash[12 + i]);

        result
    }

    fn to_account_id(&self) -> AccountId {
        hash_blake2b_256(&self.serialize_compressed()[..]).into()
    }
}

/// The ECDSA secret key.
pub type PrivateKey = SecretKey;

/// A signature (a 512-bit value, plus 8 bits for recovery ID).
#[derive(Debug)]
pub struct Signature([u8; 65]);

impl Default for Signature {
    fn default() -> Self {
        Self {
            0: [0; 65]
        }
    }
}

impl core::ops::Deref for Signature {
    type Target = [u8; 65];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Signature {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8; 65] {
        &mut self.0
    }
}

pub fn ecsign(message: &[u8; 32], private_key: &PrivateKey, chain_id: Option<u8>) -> Signature {
    let message = Message::parse(message);
    let (signature, recovery_id) = sign(&message, private_key);
    let mut recovery_byte = recovery_id.serialize();
    let mut result = Signature::default();
    signature.r.b32().iter().enumerate().for_each(|(i, byte)| result[i] = byte.clone());
    signature.s.b32().iter().enumerate().for_each(|(i, byte)| result[32 + i] = byte.clone());

    if let Some(chain_id) = chain_id {
        recovery_byte = recovery_byte + (chain_id * 2 + 35);
    } else {
        recovery_byte += 27;
    }
    result[64] = recovery_byte;

    result
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RecoveryError {
    InvalidSignature,
    InvalidRecoveryId,
    UnableRecovery,
}

pub fn ecrecover(message: &[u8; 32], signature: &Signature, chain_id: Option<u8>) -> core::result::Result<PublicKey, RecoveryError> {
    let mut recovery_byte = signature[64];
    let message = Message::parse(message);
    let signature = ECDSASignature::parse_slice(&signature[0..64]).map_err(|_| RecoveryError::InvalidSignature)?;
    if recovery_byte < 27 {
        return Err(RecoveryError::InvalidRecoveryId)
    }

    if let Some(chain_id) = chain_id {
        recovery_byte -= 2 * chain_id + 35;
    } else {
        recovery_byte -= 27;
    }

    let recovery_id = RecoveryId::parse(recovery_byte).map_err(|_| RecoveryError::InvalidRecoveryId)?;

    recover(&message, &signature, &recovery_id).map_err(|_| RecoveryError::UnableRecovery)
}

#[cfg(test)]
mod tests {
    use crate::ecdsa::*;
    use crate::{hash_blake2b_256, hash_keccak_256};

    #[ink_lang::test]
    fn correct_sign_and_recovery_blake2b() {
        let message = hash_blake2b_256("Hello world".as_bytes());
        let private_key = PrivateKey::default();
        let public_key = PublicKey::from_secret_key(&private_key);

        let signature = ecsign(&message, &private_key, None);
        let recovery_result = ecrecover(&message, &signature, None);
        assert!(recovery_result.is_ok());
        assert_eq!(recovery_result.unwrap().to_eth_address(), public_key.to_eth_address());
    }

    #[ink_lang::test]
    fn correct_sign_and_recovery_keccak() {
        let message = hash_keccak_256("Hello world".as_bytes());
        let private_key = PrivateKey::default();
        let public_key = PublicKey::from_secret_key(&private_key);

        let signature = ecsign(&message, &private_key, None);
        let recovery_result = ecrecover(&message, &signature, None);
        assert!(recovery_result.is_ok());
        assert_eq!(recovery_result.unwrap().to_eth_address(), public_key.to_eth_address());
    }

    #[ink_lang::test]
    fn correct_sign_and_recovery_chain_id_not_zero() {
        let message = hash_keccak_256("Hello world".as_bytes());
        let private_key = PrivateKey::default();
        let public_key = PublicKey::from_secret_key(&private_key);

        let signature = ecsign(&message, &private_key, Some(13));
        let recovery_result = ecrecover(&message, &signature, Some(13));
        assert!(recovery_result.is_ok());
        assert_eq!(recovery_result.unwrap().to_eth_address(), public_key.to_eth_address());
    }

    #[ink_lang::test]
    fn correct_convert_to_eth_address() {
        let raw_private_key = hex::decode("fcfd8d467daa2a1d55d16e72c3db5a4c10a9e7e345f9a37d30aac4770f931fae").unwrap();
        let private_key = PrivateKey::parse_slice(raw_private_key.as_slice()).expect("Unable parse private key");
        let public_key = PublicKey::from_secret_key(&private_key);

        let raw_eth_address = hex::decode("381Ed05543D4f73d8b5f5a85Cbce947b33a7B47F").unwrap();
        assert_eq!(raw_eth_address, public_key.to_eth_address());
    }
}