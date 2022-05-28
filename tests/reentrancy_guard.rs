// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![feature(min_specialization)]
#[cfg(feature = "reentrancy_guard")]
#[openbrush::contract]
mod reentrancy_guard {
    use ink_lang as ink;
    use openbrush::contracts::reentrancy_guard::*;

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
        #[openbrush::modifiers(non_reentrant)]
        pub fn flip(&mut self) -> Result<bool, ReentrancyGuardError> {
            let previous = self.flipped;
            self.flipped = !previous;

            Ok(previous)
        }

        #[ink(message)]
        #[openbrush::modifiers(non_reentrant)]
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
