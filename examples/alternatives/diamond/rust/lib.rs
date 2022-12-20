#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![feature(core_panic)]
#![feature(fmt_internals)]

#[cfg(not(feature = "std"))]
mod ext;

#[cfg(not(feature = "std"))]
use ink::primitives::Key;
use ink::primitives::KeyComposer;
use openbrush::contracts::diamond::*;
#[cfg(not(feature = "std"))]
use openbrush::traits::{
    DefaultEnv,
    Flush,
};

#[cfg(not(feature = "std"))]
const ROOT_KEY: Key = KeyComposer::from_bytes(&[0; 32]);

#[cfg(not(feature = "std"))]
const _: () = {
    #[no_mangle]
    #[allow(clippy::nonminimal_bool)]
    fn deploy() {
        let (_, facet_cut) = ink::env::decode_input::<([u8; 4], FacetCut)>().unwrap();

        // Support of diamond
        let mut storage = ink::env::get_contract_storage::<Key, diamond::Data>(&ROOT_KEY)
            .unwrap()
            .unwrap();
        storage._diamond_cut_facet(&facet_cut).expect("Init diamond cut");

        // Support of ownable
        let mut ownable = ink::env::get_contract_storage::<Key, ownable::Data>(&ROOT_KEY)
            .unwrap()
            .unwrap();
        ownable._init_with_owner(<ownable::Data as DefaultEnv>::env().caller());
        ownable.flush();
    }

    #[no_mangle]
    #[allow(clippy::nonminimal_bool)]
    fn call() {
        let selector = ink::env::decode_input::<[u8; 4]>().unwrap();

        let storage = ink::env::get_contract_storage::<Key, diamond::Data>(&ROOT_KEY)
            .unwrap()
            .unwrap();
        let hash = storage.selector_to_hash.get(&selector).expect("Can't find code hash");

        // Better than usage of `CallBuilder`.
        let _ = ext::delegate_call(
            1 | (1 as u32) << 2, // forward input && tailing call
            hash.as_ref(),
            &[],
            &mut [].as_mut(),
        );
    }
};

// It is stub struct to impl `StorageLayout`. But it can contains all fields with
// zero cost abstraction
pub struct Contract;

impl Contract {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(feature = "std")]
const _: () = {
    impl ::ink::storage::traits::StorageLayout for Contract {
        fn layout(key: &::ink::primitives::Key) -> ::ink::metadata::layout::Layout {
            ::ink::metadata::layout::Layout::Struct(::ink::metadata::layout::StructLayout::new(
                "Contract",
                [::ink::metadata::layout::FieldLayout::new(
                    "diamond",
                    <diamond::Data as ::ink::storage::traits::StorageLayout>::layout(key),
                )],
            ))
        }
    }
};

const _: () = {
    impl ::ink::storage::traits::StorageKey for Contract {
        const KEY: ::ink::primitives::Key = <() as ::ink::storage::traits::StorageKey>::KEY;
    }
};

impl ::ink::reflect::DispatchableConstructorInfo<0x9BAE9D5E_u32> for Contract {
    type Input = ();
    type Output = Self;
    type Storage = Contract;
    type Error = <::ink::reflect::ConstructorOutputValue<Self> as ::ink::reflect::ConstructorOutput<Contract>>::Error;
    const IS_RESULT: ::core::primitive::bool =
        <::ink::reflect::ConstructorOutputValue<Self> as ::ink::reflect::ConstructorOutput<Contract>>::IS_RESULT;
    const CALLABLE: fn(Self::Input) -> Self::Output = |__ink_binding_0| Contract::new();
    const PAYABLE: ::core::primitive::bool = false;
    const SELECTOR: [::core::primitive::u8; 4usize] = [0x9B_u8, 0xAE_u8, 0x9D_u8, 0x5E_u8];
    const LABEL: &'static ::core::primitive::str = "new";
}

#[cfg(feature = "std")]
#[cfg(not(feature = "ink-as-dependency"))]
const _: () = {
    #[no_mangle]
    pub fn __ink_generate_metadata() -> ::ink::metadata::InkProject {
        let layout = ::ink::metadata::layout::Layout::Root(::ink::metadata::layout::RootLayout::new(
            <::ink::metadata::layout::LayoutKey as ::core::convert::From<::ink::primitives::Key>>::from(
                <Contract as ::ink::storage::traits::StorageKey>::KEY,
            ),
            <Contract as ::ink::storage::traits::StorageLayout>::layout(
                &<Contract as ::ink::storage::traits::StorageKey>::KEY,
            ),
        ));
        ::ink::metadata::layout::ValidateLayout::validate(&layout).unwrap_or_else(|error| {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["metadata ink! generation failed: "],
                &[::core::fmt::ArgumentV1::new_display(&error)],
            ))
        });
        ::ink::metadata::InkProject::new(
            layout,
            ::ink::metadata::ContractSpec::new()
                .constructors([::ink::metadata::ConstructorSpec::from_label("new")
                    .selector([0x9B_u8, 0xAE_u8, 0x9D_u8, 0x5E_u8])
                    .args([::ink::metadata::MessageParamSpec::new("diamond_facet")
                        .of_type(::ink::metadata::TypeSpec::with_name_segs::<FacetCut, _>(
                            ::core::iter::Iterator::map(
                                ::core::iter::IntoIterator::into_iter(["FacetCut"]),
                                ::core::convert::AsRef::as_ref,
                            ),
                        ))
                        .done()])
                    .payable(false)
                    .returns(::ink::metadata::ReturnTypeSpec::new(
                        if <Contract as ::ink::reflect::DispatchableConstructorInfo<2611912030u32>>::IS_RESULT {
                            ::core::option::Option::Some(::ink::metadata::TypeSpec::with_name_str::<
                                ::ink::ConstructorResult<
                                    ::core::result::Result<
                                        (),
                                        <Contract as ::ink::reflect::DispatchableConstructorInfo<2611912030u32>>::Error,
                                    >,
                                >,
                            >(
                                "ink_primitives::ConstructorResult"
                            ))
                        } else {
                            ::core::option::Option::Some(::ink::metadata::TypeSpec::with_name_str::<
                                ::ink::ConstructorResult<()>,
                            >(
                                "ink_primitives::ConstructorResult"
                            ))
                        },
                    ))
                    .docs([])
                    .done()])
                .messages([::ink::metadata::MessageSpec::from_label("forward")
                    .selector([0x45_u8, 0x75_u8, 0x3C_u8, 0x2B_u8])
                    .args([])
                    .returns(::ink::metadata::ReturnTypeSpec::new(
                        ::ink::metadata::TypeSpec::with_name_segs::<::ink::MessageResult<()>, _>(
                            ::core::iter::Iterator::map(
                                ::core::iter::IntoIterator::into_iter(["ink", "MessageResult"]),
                                ::core::convert::AsRef::as_ref,
                            ),
                        ),
                    ))
                    .mutates(false)
                    .payable(true)
                    .docs([])
                    .done()])
                .events([])
                .docs([])
                .lang_error(::ink::metadata::TypeSpec::with_name_segs::<::ink::LangError, _>(
                    ::core::iter::Iterator::map(
                        ::core::iter::IntoIterator::into_iter(["ink", "LangError"]),
                        ::core::convert::AsRef::as_ref,
                    ),
                ))
                .done(),
        )
    }
};
