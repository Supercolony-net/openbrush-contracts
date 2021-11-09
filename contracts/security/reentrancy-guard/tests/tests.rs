#[cfg(test)]
#[brush::contract]
mod tests {
    use ink_lang as ink;
    use reentrancy_guard::traits::*;

    #[ink(storage)]
    #[derive(Default, ReentrancyGuardStorage)]
    pub struct MyFlipper {
        #[ReentrancyGuardStorageField]
        guard: ReentrancyGuardData,
        flipped: bool,
    }

    impl MyFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        #[brush::modifiers(non_reentrant)]
        pub fn flip(&mut self) -> Result<bool, ReentrancyGuardError> {
            let previous = self.flipped;
            self.flipped = !previous;

            Ok(previous)
        }

        #[ink(message)]
        #[brush::modifiers(non_reentrant)]
        pub fn call_flip_after_lock(&mut self) -> Result<bool, ReentrancyGuardError> {
            self.flip()
        }
    }

    #[ink::test]
    fn flip_works() {
        let mut instance = MyFlipper::new();

        assert_eq!(Ok(false), instance.flip());
        assert_eq!(Ok(true), instance.flip());
        assert_eq!(Ok(false), instance.flip());
    }

    #[ink::test]
    fn call_flip_after_lock_fails() {
        let mut instance = MyFlipper::new();

        assert_eq!(
            Err(ReentrancyGuardError::ReentrantCall),
            instance.call_flip_after_lock()
        );
    }
}
