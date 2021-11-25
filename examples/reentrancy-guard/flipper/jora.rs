#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
/// This is a stub implementation of contract with method `flip_on_me`.
/// We need this implementation to create wrapper for contract's account id.
/// With this wrapper, we can easily call methods of some contract.
/// Example:
/// ```
/// let mut flipper: CallerOfFlip = FromAccountId::from_account_id(callee);
/// flipper.flip_on_me();
/// ```
pub mod flip_on_me {
    impl ::ink_lang::ContractEnv for CallerOfFlip {
        type Env = ::ink_env::DefaultEnvironment;
    }
    type Environment = <CallerOfFlip as ::ink_lang::ContractEnv>::Env;
    type AccountId =
        <<CallerOfFlip as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::AccountId;
    type Balance =
        <<CallerOfFlip as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::Balance;
    type Hash = <<CallerOfFlip as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::Hash;
    type Timestamp =
        <<CallerOfFlip as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::Timestamp;
    type BlockNumber =
        <<CallerOfFlip as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::BlockNumber;
    pub struct CallerOfFlip {
        account_id: AccountId,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for CallerOfFlip {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new(
                        "CallerOfFlip",
                        "my_flipper_guard::flip_on_me",
                    ))
                    .type_params(::alloc::vec::Vec::new())
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field_of::<AccountId>("account_id", "AccountId"),
                    )
            }
        };
    };
    const _: () = {
        impl ::ink_storage::traits::StorageLayout for CallerOfFlip {
            fn layout(
                __key_ptr: &mut ::ink_storage::traits::KeyPtr,
            ) -> ::ink_metadata::layout::Layout {
                ::ink_metadata::layout::Layout::Struct(::ink_metadata::layout::StructLayout::new(
                    <[_]>::into_vec(box [::ink_metadata::layout::FieldLayout::new(
                        Some("account_id"),
                        <AccountId as ::ink_storage::traits::StorageLayout>::layout(__key_ptr),
                    )]),
                ))
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for CallerOfFlip {
        #[inline]
        fn clone(&self) -> CallerOfFlip {
            match *self {
                CallerOfFlip {
                    account_id: ref __self_0_0,
                } => CallerOfFlip {
                    account_id: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for CallerOfFlip {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                CallerOfFlip {
                    account_id: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "CallerOfFlip");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "account_id",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    const _: () = {
        impl ::scale::Encode for CallerOfFlip {
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::scale::Encode::encode_to(&&self.account_id, __codec_dest_edqy)
            }
            fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                ::scale::Encode::encode(&&self.account_id)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::scale::Encode::using_encoded(&&self.account_id, f)
            }
        }
        impl ::scale::EncodeLike for CallerOfFlip {}
    };
    const _: () = {
        impl ::scale::Decode for CallerOfFlip {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(CallerOfFlip {
                    account_id: {
                        let __codec_res_edqy =
                            <AccountId as ::scale::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `CallerOfFlip::account_id`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                })
            }
        }
    };
    const _: () = {
        impl ::ink_storage::traits::SpreadLayout for CallerOfFlip {
            #[allow(unused_comparisons)]
            const FOOTPRINT: u64 = [
                (0u64 + <AccountId as ::ink_storage::traits::SpreadLayout>::FOOTPRINT),
                0u64,
            ][((0u64 + <AccountId as ::ink_storage::traits::SpreadLayout>::FOOTPRINT)
                < 0u64) as usize];
            const REQUIRES_DEEP_CLEAN_UP: bool = (false
                || (false
                    || <AccountId as ::ink_storage::traits::SpreadLayout>::REQUIRES_DEEP_CLEAN_UP));
            fn pull_spread(__key_ptr: &mut ::ink_storage::traits::KeyPtr) -> Self {
                CallerOfFlip {
                    account_id: <AccountId as ::ink_storage::traits::SpreadLayout>::pull_spread(
                        __key_ptr,
                    ),
                }
            }
            fn push_spread(&self, __key_ptr: &mut ::ink_storage::traits::KeyPtr) {
                match self {
                    CallerOfFlip {
                        account_id: __binding_0,
                    } => {
                        ::ink_storage::traits::SpreadLayout::push_spread(__binding_0, __key_ptr);
                    }
                }
            }
            fn clear_spread(&self, __key_ptr: &mut ::ink_storage::traits::KeyPtr) {
                match self {
                    CallerOfFlip {
                        account_id: __binding_0,
                    } => {
                        ::ink_storage::traits::SpreadLayout::clear_spread(__binding_0, __key_ptr);
                    }
                }
            }
        }
    };
    const _: () = {
        impl ::ink_storage::traits::PackedLayout for CallerOfFlip {
            fn pull_packed(&mut self, __key: &::ink_primitives::Key) {
                match self {
                    CallerOfFlip {
                        account_id: __binding_0,
                    } => {
                        ::ink_storage::traits::PackedLayout::pull_packed(__binding_0, __key);
                    }
                }
            }
            fn push_packed(&self, __key: &::ink_primitives::Key) {
                match self {
                    CallerOfFlip {
                        account_id: __binding_0,
                    } => {
                        ::ink_storage::traits::PackedLayout::push_packed(__binding_0, __key);
                    }
                }
            }
            fn clear_packed(&self, __key: &::ink_primitives::Key) {
                match self {
                    CallerOfFlip {
                        account_id: __binding_0,
                    } => {
                        ::ink_storage::traits::PackedLayout::clear_packed(__binding_0, __key);
                    }
                }
            }
        }
    };
    const _: () = {
        impl ::ink_env::call::FromAccountId<Environment> for CallerOfFlip {
            #[inline]
            fn from_account_id(account_id: AccountId) -> Self {
                Self { account_id }
            }
        }
        impl ::ink_lang::ToAccountId<Environment> for CallerOfFlip {
            #[inline]
            fn to_account_id(&self) -> AccountId {
                self.account_id
            }
        }
    };
    const _: () = {
        impl<'a> ::ink_lang::ForwardCall for &'a CallerOfFlip {
            type Forwarder = __ink_CallForwarder<&'a CallerOfFlip>;
            #[inline]
            fn call(self) -> Self::Forwarder {
                __ink_CallForwarder { contract: self }
            }
        }
        impl<'a> ::ink_lang::ForwardCallMut for &'a mut CallerOfFlip {
            type Forwarder = __ink_CallForwarder<&'a mut CallerOfFlip>;
            #[inline]
            fn call_mut(self) -> Self::Forwarder {
                __ink_CallForwarder { contract: self }
            }
        }
        #[doc(hidden)]
        pub struct __ink_CallForwarder<T> {
            contract: T,
        }
        impl<'a> __ink_CallForwarder<&'a CallerOfFlip> {}
        impl<'a> __ink_CallForwarder<&'a CallerOfFlip> {}
        impl<'a> __ink_CallForwarder<&'a mut CallerOfFlip> {}
        impl<'a> __ink_CallForwarder<&'a mut CallerOfFlip> {
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn flip_on_me(
                self,
            ) -> ::ink_env::call::CallBuilder<
                Environment,
                ::ink_env::call::utils::Set<AccountId>,
                ::ink_env::call::utils::Unset<u64>,
                ::ink_env::call::utils::Unset<Balance>,
                ::ink_env::call::utils::Set<
                    ::ink_env::call::ExecutionInput<::ink_env::call::utils::EmptyArgumentList>,
                >,
                ::ink_env::call::utils::Set<
                    ::ink_env::call::utils::ReturnType<Result<(), ReentrancyGuardError>>,
                >,
            > {
                :: ink_env :: call :: build_call :: < Environment > () . callee (:: ink_lang :: ToAccountId :: to_account_id (self . contract)) . exec_input (:: ink_env :: call :: ExecutionInput :: new (:: ink_env :: call :: Selector :: new ([166u8 , 151u8 , 92u8 , 246u8]))) . returns :: < :: ink_env :: call :: utils :: ReturnType < Result < () , ReentrancyGuardError > > > ()
            }
        }
    };
    impl CallerOfFlip {
        #[inline]
        #[allow(clippy::type_complexity)]
        pub fn new() -> ::ink_env::call::CreateBuilder<
            Environment,
            ::ink_env::call::utils::Unset<Hash>,
            ::ink_env::call::utils::Unset<u64>,
            ::ink_env::call::utils::Unset<Balance>,
            ::ink_env::call::utils::Set<
                ::ink_env::call::ExecutionInput<::ink_env::call::utils::EmptyArgumentList>,
            >,
            ::ink_env::call::utils::Unset<::ink_env::call::state::Salt>,
            Self,
        > {
            ::ink_env::call::build_create::<Environment, Self>().exec_input(
                ::ink_env::call::ExecutionInput::new(::ink_env::call::Selector::new([
                    155u8, 174u8, 157u8, 94u8,
                ])),
            )
        }
    }
    impl CallerOfFlip {
        #[inline]
        pub fn flip_on_me(&mut self) -> Result<(), ReentrancyGuardError> {
            <&mut Self as ::ink_lang::ForwardCallMut>::call_mut(self)
                .flip_on_me()
                .fire()
                .expect("encountered error while calling CallerOfFlip::flip_on_me")
        }
    }
    use reentrancy_guard::traits::*;
}
#[cfg(feature = "ink-as-dependency")]
pub mod my_flipper_guard {
    impl ::ink_lang::ContractEnv for MyFlipper {
        type Env = ::ink_env::DefaultEnvironment;
    }
    type Environment = <MyFlipper as ::ink_lang::ContractEnv>::Env;
    type AccountId =
        <<MyFlipper as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::AccountId;
    type Balance = <<MyFlipper as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::Balance;
    type Hash = <<MyFlipper as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::Hash;
    type Timestamp =
        <<MyFlipper as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::Timestamp;
    type BlockNumber =
        <<MyFlipper as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::BlockNumber;
    #[cfg(feature = "ink-as-dependency")]
    pub struct MyFlipper {
        account_id: AccountId,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for MyFlipper {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new(
                        "MyFlipper",
                        "my_flipper_guard::my_flipper_guard",
                    ))
                    .type_params(::alloc::vec::Vec::new())
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field_of::<AccountId>("account_id", "AccountId"),
                    )
            }
        };
    };
    const _: () = {
        impl ::ink_storage::traits::StorageLayout for MyFlipper {
            fn layout(
                __key_ptr: &mut ::ink_storage::traits::KeyPtr,
            ) -> ::ink_metadata::layout::Layout {
                ::ink_metadata::layout::Layout::Struct(::ink_metadata::layout::StructLayout::new(
                    <[_]>::into_vec(box [::ink_metadata::layout::FieldLayout::new(
                        Some("account_id"),
                        <AccountId as ::ink_storage::traits::StorageLayout>::layout(__key_ptr),
                    )]),
                ))
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for MyFlipper {
        #[inline]
        fn clone(&self) -> MyFlipper {
            match *self {
                MyFlipper {
                    account_id: ref __self_0_0,
                } => MyFlipper {
                    account_id: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for MyFlipper {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                MyFlipper {
                    account_id: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "MyFlipper");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "account_id",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    const _: () = {
        impl ::scale::Encode for MyFlipper {
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::scale::Encode::encode_to(&&self.account_id, __codec_dest_edqy)
            }
            fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                ::scale::Encode::encode(&&self.account_id)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::scale::Encode::using_encoded(&&self.account_id, f)
            }
        }
        impl ::scale::EncodeLike for MyFlipper {}
    };
    const _: () = {
        impl ::scale::Decode for MyFlipper {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(MyFlipper {
                    account_id: {
                        let __codec_res_edqy =
                            <AccountId as ::scale::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `MyFlipper::account_id`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                })
            }
        }
    };
    const _: () = {
        impl ::ink_storage::traits::SpreadLayout for MyFlipper {
            #[allow(unused_comparisons)]
            const FOOTPRINT: u64 = [
                (0u64 + <AccountId as ::ink_storage::traits::SpreadLayout>::FOOTPRINT),
                0u64,
            ][((0u64 + <AccountId as ::ink_storage::traits::SpreadLayout>::FOOTPRINT)
                < 0u64) as usize];
            const REQUIRES_DEEP_CLEAN_UP: bool = (false
                || (false
                    || <AccountId as ::ink_storage::traits::SpreadLayout>::REQUIRES_DEEP_CLEAN_UP));
            fn pull_spread(__key_ptr: &mut ::ink_storage::traits::KeyPtr) -> Self {
                MyFlipper {
                    account_id: <AccountId as ::ink_storage::traits::SpreadLayout>::pull_spread(
                        __key_ptr,
                    ),
                }
            }
            fn push_spread(&self, __key_ptr: &mut ::ink_storage::traits::KeyPtr) {
                match self {
                    MyFlipper {
                        account_id: __binding_0,
                    } => {
                        ::ink_storage::traits::SpreadLayout::push_spread(__binding_0, __key_ptr);
                    }
                }
            }
            fn clear_spread(&self, __key_ptr: &mut ::ink_storage::traits::KeyPtr) {
                match self {
                    MyFlipper {
                        account_id: __binding_0,
                    } => {
                        ::ink_storage::traits::SpreadLayout::clear_spread(__binding_0, __key_ptr);
                    }
                }
            }
        }
    };
    const _: () = {
        impl ::ink_storage::traits::PackedLayout for MyFlipper {
            fn pull_packed(&mut self, __key: &::ink_primitives::Key) {
                match self {
                    MyFlipper {
                        account_id: __binding_0,
                    } => {
                        ::ink_storage::traits::PackedLayout::pull_packed(__binding_0, __key);
                    }
                }
            }
            fn push_packed(&self, __key: &::ink_primitives::Key) {
                match self {
                    MyFlipper {
                        account_id: __binding_0,
                    } => {
                        ::ink_storage::traits::PackedLayout::push_packed(__binding_0, __key);
                    }
                }
            }
            fn clear_packed(&self, __key: &::ink_primitives::Key) {
                match self {
                    MyFlipper {
                        account_id: __binding_0,
                    } => {
                        ::ink_storage::traits::PackedLayout::clear_packed(__binding_0, __key);
                    }
                }
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for MyFlipper {
        #[inline]
        fn default() -> MyFlipper {
            MyFlipper {
                account_id: ::core::default::Default::default(),
            }
        }
    }
    #[cfg(feature = "ink-as-dependency")]
    const _: () = {
        impl ::ink_env::call::FromAccountId<Environment> for MyFlipper {
            #[inline]
            fn from_account_id(account_id: AccountId) -> Self {
                Self { account_id }
            }
        }
        impl ::ink_lang::ToAccountId<Environment> for MyFlipper {
            #[inline]
            fn to_account_id(&self) -> AccountId {
                self.account_id
            }
        }
    };
    #[cfg(feature = "ink-as-dependency")]
    const _: () = {
        impl<'a> ::ink_lang::ForwardCall for &'a MyFlipper {
            type Forwarder = __ink_CallForwarder<&'a MyFlipper>;
            #[inline]
            fn call(self) -> Self::Forwarder {
                __ink_CallForwarder { contract: self }
            }
        }
        impl<'a> ::ink_lang::ForwardCallMut for &'a mut MyFlipper {
            type Forwarder = __ink_CallForwarder<&'a mut MyFlipper>;
            #[inline]
            fn call_mut(self) -> Self::Forwarder {
                __ink_CallForwarder { contract: self }
            }
        }
        #[doc(hidden)]
        pub struct __ink_CallForwarder<T> {
            contract: T,
        }
        unsafe impl<'a> ::ink_lang::CheckedInkTrait<[(); 1022313125usize]>
            for __ink_CallForwarder<&'a MyFlipper>
        {
        }
        impl<'a> __brush_PSP22ReceiverWrapper for __ink_CallForwarder<&'a MyFlipper> {
            type __ink_Checksum = [(); 1022313125usize];
            type BeforeReceivedOut = Result<(), PSP22ReceiverError>;
            #[cold]
            #[doc(hidden)]
            fn before_received(
                &mut self,
                __ink_binding_0: AccountId,
                __ink_binding_1: AccountId,
                __ink_binding_2: Balance,
                __ink_binding_3: Vec<u8>,
            ) -> Self::BeforeReceivedOut {
                extern "C" {
                    fn __ink_enforce_error_0x01705f5f62727573685f50535032325265636569766572577261707065723c6265666f72655f7265636569766564fda6f1a901(
                    ) -> !;
                }
                unsafe {
                    __ink_enforce_error_0x01705f5f62727573685f50535032325265636569766572577261707065723c6265666f72655f7265636569766564fda6f1a901 ()
                }
            }
        }
        unsafe impl<'a> ::ink_lang::CheckedInkTrait<[(); 46033994usize]>
            for __ink_CallForwarder<&'a MyFlipper>
        {
        }
        impl<'a> __brush_PSP721ReceiverWrapper for __ink_CallForwarder<&'a MyFlipper> {
            type __ink_Checksum = [(); 46033994usize];
            type BeforeReceivedOut = Result<(), PSP721ReceiverError>;
            #[cold]
            #[doc(hidden)]
            fn before_received(
                &mut self,
                __ink_binding_0: AccountId,
                __ink_binding_1: AccountId,
                __ink_binding_2: Id,
                __ink_binding_3: Vec<u8>,
            ) -> Self::BeforeReceivedOut {
                extern "C" {
                    fn __ink_enforce_error_0x01745f5f62727573685f5053503732315265636569766572577261707065723c6265666f72655f72656365697665643852dda801(
                    ) -> !;
                }
                unsafe {
                    __ink_enforce_error_0x01745f5f62727573685f5053503732315265636569766572577261707065723c6265666f72655f72656365697665643852dda801 ()
                }
            }
        }
        impl<'a> __ink_CallForwarder<&'a MyFlipper> {
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn get_value(
                self,
            ) -> ::ink_env::call::CallBuilder<
                Environment,
                ::ink_env::call::utils::Set<AccountId>,
                ::ink_env::call::utils::Unset<u64>,
                ::ink_env::call::utils::Unset<Balance>,
                ::ink_env::call::utils::Set<
                    ::ink_env::call::ExecutionInput<::ink_env::call::utils::EmptyArgumentList>,
                >,
                ::ink_env::call::utils::Set<::ink_env::call::utils::ReturnType<bool>>,
            > {
                ::ink_env::call::build_call::<Environment>()
                    .callee(::ink_lang::ToAccountId::to_account_id(self.contract))
                    .exec_input(::ink_env::call::ExecutionInput::new(
                        ::ink_env::call::Selector::new([202u8, 111u8, 33u8, 112u8]),
                    ))
                    .returns::<::ink_env::call::utils::ReturnType<bool>>()
            }
        }
        unsafe impl<'a> ::ink_lang::CheckedInkTrait<[(); 1022313125usize]>
            for __ink_CallForwarder<&'a mut MyFlipper>
        {
        }
        impl<'a> __brush_PSP22ReceiverWrapper for __ink_CallForwarder<&'a mut MyFlipper> {
            type __ink_Checksum = [(); 1022313125usize];
            #[allow(clippy::type_complexity)]
            type BeforeReceivedOut = ::ink_env::call::CallBuilder<
                Environment,
                ::ink_env::call::utils::Set<AccountId>,
                ::ink_env::call::utils::Unset<u64>,
                ::ink_env::call::utils::Unset<Balance>,
                ::ink_env::call::utils::Set<
                    ::ink_env::call::ExecutionInput<
                        ::ink_env::call::utils::ArgumentList<
                            ::ink_env::call::utils::Argument<Vec<u8>>,
                            ::ink_env::call::utils::ArgumentList<
                                ::ink_env::call::utils::Argument<Balance>,
                                ::ink_env::call::utils::ArgumentList<
                                    ::ink_env::call::utils::Argument<AccountId>,
                                    ::ink_env::call::utils::ArgumentList<
                                        ::ink_env::call::utils::Argument<AccountId>,
                                        ::ink_env::call::utils::EmptyArgumentList,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
                ::ink_env::call::utils::Set<
                    ::ink_env::call::utils::ReturnType<Result<(), PSP22ReceiverError>>,
                >,
            >;
            #[inline]
            fn before_received(
                &mut self,
                __ink_binding_0: AccountId,
                __ink_binding_1: AccountId,
                __ink_binding_2: Balance,
                __ink_binding_3: Vec<u8>,
            ) -> Self::BeforeReceivedOut {
                ::ink_env::call::build_call::<Environment>()
                    .callee(::ink_lang::ToAccountId::to_account_id(self.contract))
                    .exec_input(
                        ::ink_env::call::ExecutionInput::new(::ink_env::call::Selector::new([
                            253u8, 166u8, 241u8, 169u8,
                        ]))
                        .push_arg(__ink_binding_0)
                        .push_arg(__ink_binding_1)
                        .push_arg(__ink_binding_2)
                        .push_arg(__ink_binding_3),
                    )
                    .returns::<::ink_env::call::utils::ReturnType<Result<(), PSP22ReceiverError>>>()
            }
        }
        unsafe impl<'a> ::ink_lang::CheckedInkTrait<[(); 46033994usize]>
            for __ink_CallForwarder<&'a mut MyFlipper>
        {
        }
        impl<'a> __brush_PSP721ReceiverWrapper for __ink_CallForwarder<&'a mut MyFlipper> {
            type __ink_Checksum = [(); 46033994usize];
            #[allow(clippy::type_complexity)]
            type BeforeReceivedOut = ::ink_env::call::CallBuilder<
                Environment,
                ::ink_env::call::utils::Set<AccountId>,
                ::ink_env::call::utils::Unset<u64>,
                ::ink_env::call::utils::Unset<Balance>,
                ::ink_env::call::utils::Set<
                    ::ink_env::call::ExecutionInput<
                        ::ink_env::call::utils::ArgumentList<
                            ::ink_env::call::utils::Argument<Vec<u8>>,
                            ::ink_env::call::utils::ArgumentList<
                                ::ink_env::call::utils::Argument<Id>,
                                ::ink_env::call::utils::ArgumentList<
                                    ::ink_env::call::utils::Argument<AccountId>,
                                    ::ink_env::call::utils::ArgumentList<
                                        ::ink_env::call::utils::Argument<AccountId>,
                                        ::ink_env::call::utils::EmptyArgumentList,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
                ::ink_env::call::utils::Set<
                    ::ink_env::call::utils::ReturnType<Result<(), PSP721ReceiverError>>,
                >,
            >;
            #[inline]
            fn before_received(
                &mut self,
                __ink_binding_0: AccountId,
                __ink_binding_1: AccountId,
                __ink_binding_2: Id,
                __ink_binding_3: Vec<u8>,
            ) -> Self::BeforeReceivedOut {
                ::ink_env::call::build_call::<Environment>()
                    .callee(::ink_lang::ToAccountId::to_account_id(self.contract))
                    .exec_input(
                        ::ink_env::call::ExecutionInput::new(::ink_env::call::Selector::new([
                            56u8, 82u8, 221u8, 168u8,
                        ]))
                        .push_arg(__ink_binding_0)
                        .push_arg(__ink_binding_1)
                        .push_arg(__ink_binding_2)
                        .push_arg(__ink_binding_3),
                    )
                    .returns::<::ink_env::call::utils::ReturnType<Result<(), PSP721ReceiverError>>>(
                    )
            }
        }
        impl<'a> __ink_CallForwarder<&'a mut MyFlipper> {
            #[cfg(feature = "ink-as-dependency")]
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn flip(
                self,
            ) -> ::ink_env::call::CallBuilder<
                Environment,
                ::ink_env::call::utils::Set<AccountId>,
                ::ink_env::call::utils::Unset<u64>,
                ::ink_env::call::utils::Unset<Balance>,
                ::ink_env::call::utils::Set<
                    ::ink_env::call::ExecutionInput<::ink_env::call::utils::EmptyArgumentList>,
                >,
                ::ink_env::call::utils::Set<
                    ::ink_env::call::utils::ReturnType<Result<(), ReentrancyGuardError>>,
                >,
            > {
                :: ink_env :: call :: build_call :: < Environment > () . callee (:: ink_lang :: ToAccountId :: to_account_id (self . contract)) . exec_input (:: ink_env :: call :: ExecutionInput :: new (:: ink_env :: call :: Selector :: new ([99u8 , 58u8 , 165u8 , 81u8]))) . returns :: < :: ink_env :: call :: utils :: ReturnType < Result < () , ReentrancyGuardError > > > ()
            }
            #[cfg(feature = "ink-as-dependency")]
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn call_flip_on_me(
                self,
                __ink_binding_0: AccountId,
            ) -> ::ink_env::call::CallBuilder<
                Environment,
                ::ink_env::call::utils::Set<AccountId>,
                ::ink_env::call::utils::Unset<u64>,
                ::ink_env::call::utils::Unset<Balance>,
                ::ink_env::call::utils::Set<
                    ::ink_env::call::ExecutionInput<
                        ::ink_env::call::utils::ArgumentList<
                            ::ink_env::call::utils::Argument<AccountId>,
                            ::ink_env::call::utils::EmptyArgumentList,
                        >,
                    >,
                >,
                ::ink_env::call::utils::Set<
                    ::ink_env::call::utils::ReturnType<Result<(), ReentrancyGuardError>>,
                >,
            > {
                :: ink_env :: call :: build_call :: < Environment > () . callee (:: ink_lang :: ToAccountId :: to_account_id (self . contract)) . exec_input (:: ink_env :: call :: ExecutionInput :: new (:: ink_env :: call :: Selector :: new ([134u8 , 238u8 , 141u8 , 195u8])) . push_arg (__ink_binding_0)) . returns :: < :: ink_env :: call :: utils :: ReturnType < Result < () , ReentrancyGuardError > > > ()
            }
        }
    };
    #[cfg(feature = "ink-as-dependency")]
    unsafe impl ::ink_lang::CheckedInkTrait<[(); 1022313125usize]> for MyFlipper {}
    #[cfg(feature = "ink-as-dependency")]
    impl __brush_PSP22ReceiverWrapper for MyFlipper {
        type __ink_Checksum = [(); 1022313125usize];
        type BeforeReceivedOut = Result<(), PSP22ReceiverError>;
        #[inline]
        fn before_received(
            &mut self,
            operator: AccountId,
            from: AccountId,
            value: Balance,
            data: Vec<u8>,
        ) -> Self::BeforeReceivedOut {
            < & mut Self as :: ink_lang :: ForwardCallMut > :: call_mut (self) . before_received (operator , from , value , data) . fire () . expect ("encountered error while calling <MyFlipper as __brush_PSP22ReceiverWrapper>::before_received")
        }
    }
    #[cfg(feature = "ink-as-dependency")]
    unsafe impl ::ink_lang::CheckedInkTrait<[(); 46033994usize]> for MyFlipper {}
    #[cfg(feature = "ink-as-dependency")]
    impl __brush_PSP721ReceiverWrapper for MyFlipper {
        type __ink_Checksum = [(); 46033994usize];
        type BeforeReceivedOut = Result<(), PSP721ReceiverError>;
        #[inline]
        fn before_received(
            &mut self,
            operator: AccountId,
            from: AccountId,
            id: Id,
            data: Vec<u8>,
        ) -> Self::BeforeReceivedOut {
            < & mut Self as :: ink_lang :: ForwardCallMut > :: call_mut (self) . before_received (operator , from , id , data) . fire () . expect ("encountered error while calling <MyFlipper as __brush_PSP721ReceiverWrapper>::before_received")
        }
    }
    #[cfg(feature = "ink-as-dependency")]
    impl MyFlipper {
        #[inline]
        pub fn get_value(&self) -> bool {
            <&Self as ::ink_lang::ForwardCall>::call(self)
                .get_value()
                .fire()
                .expect("encountered error while calling MyFlipper::get_value")
        }
        #[inline]
        pub fn flip(&mut self) -> Result<(), ReentrancyGuardError> {
            <&mut Self as ::ink_lang::ForwardCallMut>::call_mut(self)
                .flip()
                .fire()
                .expect("encountered error while calling MyFlipper::flip")
        }
        #[inline]
        pub fn call_flip_on_me(&mut self, callee: AccountId) -> Result<(), ReentrancyGuardError> {
            <&mut Self as ::ink_lang::ForwardCallMut>::call_mut(self)
                .call_flip_on_me(callee)
                .fire()
                .expect("encountered error while calling MyFlipper::call_flip_on_me")
        }
        #[inline]
        #[allow(clippy::type_complexity)]
        pub fn new() -> ::ink_env::call::CreateBuilder<
            Environment,
            ::ink_env::call::utils::Unset<Hash>,
            ::ink_env::call::utils::Unset<u64>,
            ::ink_env::call::utils::Unset<Balance>,
            ::ink_env::call::utils::Set<
                ::ink_env::call::ExecutionInput<::ink_env::call::utils::EmptyArgumentList>,
            >,
            ::ink_env::call::utils::Unset<::ink_env::call::state::Salt>,
            Self,
        > {
            ::ink_env::call::build_create::<Environment, Self>().exec_input(
                ::ink_env::call::ExecutionInput::new(::ink_env::call::Selector::new([
                    155u8, 174u8, 157u8, 94u8,
                ])),
            )
        }
    }
    use brush::modifiers;
    use ink_prelude::vec::Vec;
    use psp22::traits::*;
    use psp721::traits::*;
    use reentrancy_guard::traits::*;
}
