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

use ::ink_env::{
    DefaultEnvironment,
    Environment,
};
use core::mem::ManuallyDrop;

pub type AccountId = <DefaultEnvironment as Environment>::AccountId;
pub type Balance = <DefaultEnvironment as Environment>::Balance;
pub type Hash = <DefaultEnvironment as Environment>::Hash;
pub type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
pub type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
pub type ChainExtension = <DefaultEnvironment as Environment>::ChainExtension;
pub type EnvAccess = ::ink_lang::EnvAccess<'static, DefaultEnvironment>;

pub trait InkStorage: Sized {
    fn env() -> EnvAccess {
        Default::default()
    }
}

impl<T> InkStorage for T {}

pub const ZERO_ADDRESS: [u8; 32] = [0; 32];

pub trait AccountIdExt {
    fn is_zero(&self) -> bool;
}

impl AccountIdExt for AccountId {
    fn is_zero(&self) -> bool {
        self == &ZERO_ADDRESS.into()
    }
}

/// This trait is automatically implemented for storage.
pub trait Flush: ::ink_storage::traits::SpreadLayout + InkStorage {
    /// Method flushes the current state of `Self` into storage.
    /// ink! recursively calculate a key of each field.
    /// So if you want to flush the correct state of the contract,
    /// you have to this method on storage struct.
    fn flush(&self) {
        let root_key = ::ink_primitives::Key::from([0x00; 32]);
        ::ink_storage::traits::push_spread_root::<Self>(self, &root_key);
    }

    /// Method loads the current state of `Self` from storage.
    /// ink! recursively calculate a key of each field.
    /// So if you want to load the correct state of the contract,
    /// you have to this method on storage struct.
    fn load(&mut self) {
        let root_key = ::ink_primitives::Key::from([0x00; 32]);
        let mut state = ::ink_storage::traits::pull_spread_root::<Self>(&root_key);
        core::mem::swap(self, &mut state);
        let _ = ManuallyDrop::new(state);
    }
}

impl<T: ::ink_storage::traits::SpreadLayout + InkStorage> Flush for T {}

/// Types for managing mock cross-contract calls in unit tests
#[cfg(feature = "mockable")]
pub mod mock {
    use super::AccountId;

    use alloc::{
        rc::Rc,
        vec::Vec,
    };
    use core::{
        cell::{
            Ref,
            RefCell,
            RefMut,
        },
        ops::{
            Deref,
            DerefMut,
        },
    };

    /// A frame in the call stack
    #[derive(Clone, Debug)]
    pub struct MockCallContext {
        pub level: u32,
        pub caller: Option<AccountId>,
        pub callee: AccountId,
    }

    /// A managed call stack for mocking cross-contract call in test environment
    #[derive(Clone)]
    pub struct SharedCallStack {
        stack: Rc<RefCell<Vec<MockCallContext>>>,
    }

    impl SharedCallStack {
        /// Crates a call stack with the default `account`
        pub fn new(account: AccountId) -> Self {
            SharedCallStack {
                stack: Rc::new(RefCell::new(alloc::vec![MockCallContext {
                    level: 0,
                    caller: None,
                    callee: account,
                }])),
            }
        }

        /// Changes the caller account
        ///
        /// Only allowed outside any contract call (when the stack is empty).
        pub fn switch_account(&self, account: AccountId) -> Result<(), ()> {
            let mut stack = self.stack.borrow_mut();
            if stack.len() != 1 {
                return Err(())
            }
            let ctx = stack.get_mut(0).ok_or(())?;
            ctx.callee = account;
            Ok(())
        }

        /// Pushes a new call frame
        pub fn push(&self, callee: &AccountId) {
            let parent_ctx = self.peek();
            self.stack.borrow_mut().push(MockCallContext {
                level: parent_ctx.level + 1,
                caller: Some(parent_ctx.callee),
                callee: callee.clone(),
            });
            self.sync_to_ink();
        }

        /// Pops the call frame and returns the frame
        pub fn pop(&self) -> Option<MockCallContext> {
            if self.stack.borrow().len() > 1 {
                let ctx = self.stack.borrow_mut().pop();
                self.sync_to_ink();
                ctx
            } else {
                None
            }
        }

        /// Peeks the current call frame
        pub fn peek(&self) -> MockCallContext {
            self.stack.borrow().last().cloned().expect("stack is never empty; qed.")
        }

        /// Syncs the top call frame to ink testing environment
        pub fn sync_to_ink(&self) {
            let ctx = self.peek();
            if let Some(caller) = ctx.caller {
                ink_env::test::set_caller::<ink_env::DefaultEnvironment>(caller);
            }
            ink_env::test::set_callee::<ink_env::DefaultEnvironment>(ctx.callee);
        }
    }

    /// A wrapper of a contract with an address for call stake auto-management
    #[derive(Clone)]
    pub struct Addressable<T> {
        inner: Rc<RefCell<T>>,
        id: AccountId,
        stack: SharedCallStack,
    }

    impl<T> Addressable<T> {
        /// Wraps a contract reference with id and a shared call stack
        pub fn new(id: AccountId, inner: Rc<RefCell<T>>, stack: SharedCallStack) -> Self {
            Addressable { inner, id, stack }
        }

        /// Wraps a native contract object with a simple id
        ///
        /// The account id of the contract will be the `id` with zero-padding.
        pub fn create_native(id: u8, inner: T, stack: SharedCallStack) -> Self {
            Addressable {
                inner: Rc::new(RefCell::new(inner)),
                id: naive_id(id),
                stack,
            }
        }

        /// Returns the account id of the inner contract
        pub fn id(&self) -> AccountId {
            self.id.clone()
        }

        /// Borrows the contract for _a_ call with the stack auto-managed
        ///
        /// Holding the ref for multiple calls or nested call is considered abuse.
        pub fn call(&self) -> ScopedRef<'_, T> {
            ScopedRef::new(self.inner.borrow(), &self.id, self.stack.clone())
        }

        /// Borrows the contract for _a_ mut call with the stack auto-managed
        ///
        /// Holding the mut ref for multiple calls or nested call is considered abuse.
        pub fn call_mut(&self) -> ScopedRefMut<'_, T> {
            ScopedRefMut::new(self.inner.borrow_mut(), &self.id, self.stack.clone())
        }
    }

    /// Push a call stack when the `Ref` in scope
    pub struct ScopedRef<'b, T: 'b> {
        inner: Ref<'b, T>,
        stack: SharedCallStack,
    }

    impl<'b, T> ScopedRef<'b, T> {
        fn new(inner: Ref<'b, T>, address: &AccountId, stack: SharedCallStack) -> Self {
            stack.push(address);
            Self { inner, stack }
        }
    }

    impl<'b, T> Deref for ScopedRef<'b, T> {
        type Target = T;
        fn deref(&self) -> &T {
            self.inner.deref()
        }
    }

    impl<'b, T> Drop for ScopedRef<'b, T> {
        fn drop(&mut self) {
            self.stack.pop().expect("pop never fails");
        }
    }

    /// Push a call stack when the `RefMut` in scope
    pub struct ScopedRefMut<'b, T: 'b> {
        inner: RefMut<'b, T>,
        stack: SharedCallStack,
    }

    impl<'b, T> ScopedRefMut<'b, T> {
        fn new(inner: RefMut<'b, T>, address: &AccountId, stack: SharedCallStack) -> Self {
            stack.push(address);
            Self { inner, stack }
        }
    }

    impl<'b, T> Deref for ScopedRefMut<'b, T> {
        type Target = T;
        fn deref(&self) -> &T {
            self.inner.deref()
        }
    }

    impl<'b, T> DerefMut for ScopedRefMut<'b, T> {
        fn deref_mut(&mut self) -> &mut T {
            self.inner.deref_mut()
        }
    }

    impl<'b, T> Drop for ScopedRefMut<'b, T> {
        fn drop(&mut self) {
            self.stack.pop().expect("pop never fails");
        }
    }

    /// Generates a naive zero-padding account id with a `u8` number
    pub fn naive_id(id: u8) -> AccountId {
        let mut address = [0u8; 32];
        address[31] = id;
        address.into()
    }
}
