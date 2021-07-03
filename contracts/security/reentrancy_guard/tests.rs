#[cfg(test)]
#[brush::contract]
mod tests {
    use crate::traits::*;
    use ink_lang as ink;

    #[ink(storage)]
    #[derive(Default, ReentrancyGuardStorage)]
    pub struct MyFlipper {
        flipped: bool,
    }

    impl MyFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        #[brush::modifiers(non_reentrant)]
        pub fn flip(&mut self) -> bool {
            let previous = self.flipped;
            if self.flipped {
                self.flipped = false;
            } else {
                self.flipped = true;
            }

            previous
        }

        #[ink(message)]
        #[brush::modifiers(non_reentrant)]
        pub fn call_flip_after_lock(&mut self) -> bool {
            self.flip()
        }
    }

    #[ink::test]
    fn flip_works() {
        let mut _inst = MyFlipper::new();

        assert_eq!(false, _inst.flip());
        assert_eq!(true, _inst.flip());
        assert_eq!(false, _inst.flip());
    }

    #[ink::test]
    #[should_panic(expected = "ReentrantCall")]
    fn call_flip_after_lock_fails() {
        let mut _inst = MyFlipper::new();

        _inst.call_flip_after_lock();
    }
}