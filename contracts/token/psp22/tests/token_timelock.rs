#[brush::contract]
mod tests {
    use ink_lang as ink;
    use psp22::{
        traits::*,
        utils::token_timelock::*,
    };

    #[ink(storage)]
    #[derive(Default, PSP22TokenTimelockStorage)]
    pub struct PSP22TokenTimelockStruct {
        #[PSP22TokenTimelockStorageField]
        timelock: PSP22TokenTimelockData,
    }

    /// We will override timelock functions so they are not using cross-contract call in tests
    /// We will just remove calls to transfer of underlying token
    /// The cross-contract interaction will be tested in integration tests
    impl PSP22TokenTimelock for PSP22TokenTimelockStruct {}

    impl PSP22Receiver for PSP22TokenTimelockStruct {
        #[ink(message)]
        fn before_received(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _value: Balance,
            _data: Vec<u8>,
        ) -> Result<(), PSP22ReceiverError> {
            Ok(())
        }
    }

    impl PSP22TokenTimelockStruct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    /// The default constructor does its job.
    #[ink::test]
    fn new_works() {
        let mut timelock = PSP22TokenTimelockStruct::new();
    }
}
