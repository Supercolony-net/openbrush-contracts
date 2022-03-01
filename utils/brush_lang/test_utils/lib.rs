#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::{
    hash::{
        Blake2x256,
        CryptoHash,
        HashOutput,
    },
    Clear,
    Hash,
};

#[cfg(feature = "std")]
use ink_env::{
    test::DefaultAccounts,
    DefaultEnvironment,
    Environment,
};

pub fn encoded_into_hash<T>(entity: &T) -> Hash
where
    T: scale::Encode,
{
    let mut result = Hash::clear();
    let len_result = result.as_ref().len();
    let encoded = entity.encode();
    let len_encoded = encoded.len();
    if len_encoded <= len_result {
        result.as_mut()[..len_encoded].copy_from_slice(&encoded);
        return result
    }
    let mut hash_output = <<Blake2x256 as HashOutput>::Type as Default>::default();
    <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
    let copy_len = core::cmp::min(hash_output.len(), len_result);
    result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
    result
}

/// For calculating the event topic hash.
pub struct PrefixedValue<'a, 'b, T> {
    pub prefix: &'a [u8],
    pub value: &'b T,
}

impl<X> scale::Encode for PrefixedValue<'_, '_, X>
where
    X: scale::Encode,
{
    #[inline]
    fn size_hint(&self) -> usize {
        self.prefix.size_hint() + self.value.size_hint()
    }

    #[inline]
    fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
        self.prefix.encode_to(dest);
        self.value.encode_to(dest);
    }
}

#[cfg(feature = "std")]
pub fn accounts() -> DefaultAccounts<DefaultEnvironment> {
    ink_env::test::default_accounts::<DefaultEnvironment>().expect("Cannot get accounts")
}

#[cfg(feature = "std")]
pub fn change_caller(new_caller: <DefaultEnvironment as Environment>::AccountId) {
    let callee = ink_env::account_id::<DefaultEnvironment>();
    let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4]));
    data.push_arg(&new_caller);
    ink_env::test::push_execution_context::<DefaultEnvironment>(new_caller, callee, 1000000, 1000000, data);
}
