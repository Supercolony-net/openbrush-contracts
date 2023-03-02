"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[93098],{3905:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>f});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},i=Object.keys(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var p=a.createContext({}),s=function(e){var t=a.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},c=function(e){var t=s(e.components);return a.createElement(p.Provider,{value:t},e.children)},u="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},d=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,i=e.originalType,p=e.parentName,c=l(e,["components","mdxType","originalType","parentName"]),u=s(n),d=r,f=u["".concat(p,".").concat(d)]||u[d]||m[d]||i;return n?a.createElement(f,o(o({ref:t},c),{},{components:n})):a.createElement(f,o({ref:t},c))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var i=n.length,o=new Array(i);o[0]=d;var l={};for(var p in t)hasOwnProperty.call(t,p)&&(l[p]=t[p]);l.originalType=e,l[u]="string"==typeof e?e:r,o[1]=l;for(var s=2;s<i;s++)o[s]=n[s];return a.createElement.apply(null,o)}return a.createElement.apply(null,n)}d.displayName="MDXCreateElement"},76283:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>o,default:()=>m,frontMatter:()=>i,metadata:()=>l,toc:()=>s});var a=n(87462),r=(n(67294),n(3905));const i={sidebar_position:7,title:"Payment Splitter"},o=void 0,l={unversionedId:"smart-contracts/payment-splitter",id:"version-2.0.0/smart-contracts/payment-splitter",title:"Payment Splitter",description:"This example shows how you can reuse the implementation of",source:"@site/versioned_docs/version-2.0.0/smart-contracts/payment-splitter.md",sourceDirName:"smart-contracts",slug:"/smart-contracts/payment-splitter",permalink:"/2.0.0/smart-contracts/payment-splitter",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/versioned_docs/version-2.0.0/smart-contracts/payment-splitter.md",tags:[],version:"2.0.0",sidebarPosition:7,frontMatter:{sidebar_position:7,title:"Payment Splitter"},sidebar:"tutorialSidebar",previous:{title:"Pausable",permalink:"/2.0.0/smart-contracts/pausable"},next:{title:"PSP22",permalink:"/2.0.0/smart-contracts/PSP22/"}},p={},s=[{value:"Step 1: Include dependencies",id:"step-1-include-dependencies",level:2},{value:"Step 2: Add imports and enable unstable feature",id:"step-2-add-imports-and-enable-unstable-feature",level:2},{value:"Step 3: Define storage",id:"step-3-define-storage",level:2},{value:"Step 4: Inherit logic",id:"step-4-inherit-logic",level:2},{value:"Step 5: Define constructor",id:"step-5-define-constructor",level:2},{value:"Step 6 (Optional): Customize your contract",id:"step-6-optional-customize-your-contract",level:2}],c={toc:s},u="wrapper";function m(e){let{components:t,...n}=e;return(0,r.kt)(u,(0,a.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"This example shows how you can reuse the implementation of\n",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/finance/payment_splitter"},"payment-splitter"),"."),(0,r.kt)("h2",{id:"step-1-include-dependencies"},"Step 1: Include dependencies"),(0,r.kt)("p",null,"Include ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush")," as dependency in the cargo file or you can use ",(0,r.kt)("a",{parentName:"p",href:"/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush"},"default ",(0,r.kt)("inlineCode",{parentName:"a"},"Cargo.toml"))," template.\nAfter you need to enable default implementation of Payment Splitter via ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush")," features."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-toml"},'openbrush = { version = "~2.0.0", default-features = false, features = ["payment_splitter"] }\n\n# payment-splitter uses dividing inside, so your version of rust can require you to disable check overflow.\n[profile.dev]\n')),(0,r.kt)("h2",{id:"step-2-add-imports-and-enable-unstable-feature"},"Step 2: Add imports and enable unstable feature"),(0,r.kt)("p",null,"Use ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::contract")," macro instead of ",(0,r.kt)("inlineCode",{parentName:"p"},"ink::contract"),". Import ",(0,r.kt)("strong",{parentName:"p"},"everything")," from ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::contracts::payment_splitter"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[openbrush::contract]\npub mod my_payment_splitter {\n    use openbrush::contracts::payment_splitter::*;\n    use ink_prelude::vec::Vec;\n    use ink_storage::traits::SpreadAllocate;\n')),(0,r.kt)("h2",{id:"step-3-define-storage"},"Step 3: Define storage"),(0,r.kt)("p",null,"Declare storage struct and declare the field related to ",(0,r.kt)("inlineCode",{parentName:"p"},"PaymentSplitterStorage"),"\nThen you need to derive ",(0,r.kt)("inlineCode",{parentName:"p"},"PaymentSplitterStorage")," trait and mark corresponding field\nwith ",(0,r.kt)("inlineCode",{parentName:"p"},"#[PaymentSplitterStorageField]")," attribute. Deriving this trait allows you to reuse\nthe default implementation of ",(0,r.kt)("inlineCode",{parentName:"p"},"PaymentSplitter"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, SpreadAllocate, PaymentSplitterStorage)]\npub struct SplitterStruct {\n   #[PaymentSplitterStorageField]\n   splitter: PaymentSplitterData,\n}\n")),(0,r.kt)("h2",{id:"step-4-inherit-logic"},"Step 4: Inherit logic"),(0,r.kt)("p",null,"Inherit the implementation of ",(0,r.kt)("inlineCode",{parentName:"p"},"PaymentSplitter"),". You can customize (override) methods in this ",(0,r.kt)("inlineCode",{parentName:"p"},"impl")," block."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl PaymentSplitter for SplitterStruct {}\n")),(0,r.kt)("h2",{id:"step-5-define-constructor"},"Step 5: Define constructor"),(0,r.kt)("p",null,"Define constructor. Your basic version of ",(0,r.kt)("inlineCode",{parentName:"p"},"PaymentSplitter")," contract is ready!"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'impl SplitterStruct {\n   #[ink(constructor)]\n   pub fn new(payees_and_shares: Vec<(AccountId, Balance)>) -> Self {\n      ink_lang::codegen::initialize_contract(|instance: &mut Self| {\n         instance._init(payees_and_shares).expect("Should init");\n      })\n   }\n}\n')),(0,r.kt)("p",null,"You can check an example of the usage of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/payment_splitter"},"PaymentSplitter"),"."),(0,r.kt)("h2",{id:"step-6-optional-customize-your-contract"},"Step 6 (Optional): Customize your contract"),(0,r.kt)("p",null,"The ",(0,r.kt)("inlineCode",{parentName:"p"},"PaymentSplitter")," trait defines and has default implementations for the core payment splitter functions. Additional functionality with ",(0,r.kt)("em",{parentName:"p"},"some")," predefined functions is available through the ",(0,r.kt)("inlineCode",{parentName:"p"},"PaymentSplitterInternal")," trait (",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush-contracts/contracts/finance/payment_splitter/mod.rs"),"). Likely the most common function to use from this internal trait will be ",(0,r.kt)("inlineCode",{parentName:"p"},"_release_all"),". This allows you to payout all ",(0,r.kt)("inlineCode",{parentName:"p"},"payees")," stored in the contract at once. To add this function to your contract, simply define a new publicly dispatchable function (i.e. ",(0,r.kt)("inlineCode",{parentName:"p"},"#[ink(message)]"),") called ",(0,r.kt)("inlineCode",{parentName:"p"},"release_all")," and have it call the internal ",(0,r.kt)("inlineCode",{parentName:"p"},"_release_all")," function using ",(0,r.kt)("inlineCode",{parentName:"p"},"self"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'impl SplitterStruct {\n        #[ink(constructor)]\n        pub fn new(payees_and_shares: Vec<(AccountId, Balance)>) -> Self {\n            ink_lang::codegen::initialize_contract(|instance: &mut Self| {\n                instance._init(payees_and_shares).expect("Should init");\n            })\n        }\n\n        /// Payout all payees at once.\n        #[ink(message)]\n        pub fn release_all(&mut self) -> Result<(), PaymentSplitterError> {\n            // `_release_all()` is an internal method defined by the `PaymentSplitterInternal` trait\n            self._release_all()\n        }\n    }\n')),(0,r.kt)("p",null,"The ",(0,r.kt)("inlineCode",{parentName:"p"},"_add_payee")," function is also available in the ",(0,r.kt)("inlineCode",{parentName:"p"},"PaymentSplitterInternal")," trait and can be added to your contract in the same way as ",(0,r.kt)("inlineCode",{parentName:"p"},"_release_all"),"."))}m.isMDXComponent=!0}}]);