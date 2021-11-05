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
        let mut _inst = MyFlipper::new();

        assert_eq!(Ok(false), _inst.flip());
        assert_eq!(Ok(true), _inst.flip());
        assert_eq!(Ok(false), _inst.flip());
    }

    #[ink::test]
    fn call_flip_after_lock_fails() {
        let mut _inst = MyFlipper::new();

        assert_eq!(Err(ReentrancyGuardError::ReentrantCall), _inst.call_flip_after_lock());
    }
}
