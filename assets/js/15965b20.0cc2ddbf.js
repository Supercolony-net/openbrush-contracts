"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[61300],{3905:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>f});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function l(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var i=a.createContext({}),p=function(e){var t=a.useContext(i),n=t;return e&&(n="function"==typeof e?e(t):l(l({},t),e)),n},c=function(e){var t=p(e.components);return a.createElement(i.Provider,{value:t},e.children)},u="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},d=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,i=e.parentName,c=s(e,["components","mdxType","originalType","parentName"]),u=p(n),d=r,f=u["".concat(i,".").concat(d)]||u[d]||m[d]||o;return n?a.createElement(f,l(l({ref:t},c),{},{components:n})):a.createElement(f,l({ref:t},c))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,l=new Array(o);l[0]=d;var s={};for(var i in t)hasOwnProperty.call(t,i)&&(s[i]=t[i]);s.originalType=e,s[u]="string"==typeof e?e:r,l[1]=s;for(var p=2;p<o;p++)l[p]=n[p];return a.createElement.apply(null,l)}return a.createElement.apply(null,n)}d.displayName="MDXCreateElement"},30790:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>i,contentTitle:()=>l,default:()=>m,frontMatter:()=>o,metadata:()=>s,toc:()=>p});var a=n(87462),r=(n(67294),n(3905));const o={sidebar_position:9,title:"PSP22 Pallet"},l=void 0,s={unversionedId:"smart-contracts/PSP22-Pallet/psp22-pallet",id:"smart-contracts/PSP22-Pallet/psp22-pallet",title:"PSP22 Pallet",description:"This example shows how you can reuse the implementation of PSP22 Pallet via pallet-assets chain extension. Also, this example shows how you can customize the logic, for example, to get current assetid.",source:"@site/docs/smart-contracts/PSP22-Pallet/psp22-pallet.md",sourceDirName:"smart-contracts/PSP22-Pallet",slug:"/smart-contracts/PSP22-Pallet/",permalink:"/next/smart-contracts/PSP22-Pallet/",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/docs/smart-contracts/PSP22-Pallet/psp22-pallet.md",tags:[],version:"current",sidebarPosition:9,frontMatter:{sidebar_position:9,title:"PSP22 Pallet"},sidebar:"tutorialSidebar",previous:{title:"Timelock Controller",permalink:"/next/smart-contracts/timelock-controller"},next:{title:"PSP22 Metadata",permalink:"/next/smart-contracts/PSP22-Pallet/Extensions/metadata"}},i={},p=[{value:"Step 1: Import default implementation",id:"step-1-import-default-implementation",level:2},{value:"Step 2: Define storage and implement default PSP22 trait",id:"step-2-define-storage-and-implement-default-psp22-trait",level:2},{value:"Step 3: Add constructor",id:"step-3-add-constructor",level:2}],c={toc:p},u="wrapper";function m(e){let{components:t,...n}=e;return(0,r.kt)(u,(0,a.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"This example shows how you can reuse the implementation of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet"},"PSP22 Pallet")," via ",(0,r.kt)("inlineCode",{parentName:"p"},"pallet-assets")," chain extension. Also, this example shows how you can customize the logic, for example, to get current ",(0,r.kt)("inlineCode",{parentName:"p"},"asset_id"),"."),(0,r.kt)("h2",{id:"step-1-import-default-implementation"},"Step 1: Import default implementation"),(0,r.kt)("p",null,"With ",(0,r.kt)("a",{parentName:"p",href:"/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush"},"default ",(0,r.kt)("inlineCode",{parentName:"a"},"Cargo.toml")),",\nyou need to import the ",(0,r.kt)("inlineCode",{parentName:"p"},"psp22_pallet")," module, enable the corresponding feature, and embed the module data structure\nas described in ",(0,r.kt)("a",{parentName:"p",href:"/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush"},"that section"),"."),(0,r.kt)("h2",{id:"step-2-define-storage-and-implement-default-psp22-trait"},"Step 2: Define storage and implement default PSP22 trait"),(0,r.kt)("p",null,"Use ",(0,r.kt)("inlineCode",{parentName:"p"},"psp22_pallet")," storage and implement ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22")," trait for your contract."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[openbrush::contract]\npub mod my_psp22_pallet {\n    use ink_storage::traits::SpreadAllocate;\n    use openbrush::{\n        contracts::psp22_pallet::*,\n        traits::Storage,\n    };\n\n    #[ink(storage)]\n    #[derive(Default, SpreadAllocate, Storage)]\n    pub struct Contract {\n        #[storage_field]\n        pallet: psp22_pallet::Data,\n    }\n\n    impl PSP22 for Contract {}\n}\n')),(0,r.kt)("h2",{id:"step-3-add-constructor"},"Step 3: Add constructor"),(0,r.kt)("p",null,"Add constructor for your contract, create asset and mint tokens to caller."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[openbrush::contract]\npub mod my_psp22_pallet {\n    use ink_storage::traits::SpreadAllocate;\n    use openbrush::{\n        contracts::psp22_pallet::*,\n        traits::Storage,\n    };\n\n    #[ink(storage)]\n    #[derive(Default, SpreadAllocate, Storage)]\n    pub struct Contract {\n        #[storage_field]\n        pallet: psp22_pallet::Data,\n    }\n\n    impl PSP22 for Contract {}\n\n    impl Contract {\n        /// During instantiation of the contract, you need to pass native tokens as a deposit\n        /// for asset creation.\n        #[ink(constructor)]\n        #[ink(payable)]\n        pub fn new(asset_id: u32, min_balance: Balance, total_supply: Balance) -> Self {\n            ink_lang::codegen::initialize_contract(|instance: &mut Contract| {\n                // The contract is admin of the asset\n                instance\n                    ._create(asset_id, Self::env().account_id(), min_balance)\n                    .expect("Should create an asset");\n                instance.pallet.asset_id = asset_id;\n                instance.pallet.origin = Origin::Caller;\n                instance\n                    ._mint_to(instance.env().caller(), total_supply)\n                    .expect("Should mint");\n            })\n        }\n\n        /// Asset id of the asset in the `pallet-assets`\n        #[ink(message)]\n        pub fn asset_id(&self) -> u32 {\n            self.pallet.asset_id\n        }\n    }\n}\n\n')),(0,r.kt)("p",null,"You can check an example of the usage of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_pallet"},"PSP22 Pallet"),"."),(0,r.kt)("p",null,"Also you can use extensions for PSP22 token:"),(0,r.kt)("p",null,(0,r.kt)("a",{parentName:"p",href:"Extensions/metadata"},"PSP22 Pallet Metadata"),": metadata for PSP22 Pallet."),(0,r.kt)("p",null,(0,r.kt)("a",{parentName:"p",href:"Extensions/mintable"},"PSP22 Pallet Mintable"),": creation of new tokens."),(0,r.kt)("p",null,(0,r.kt)("a",{parentName:"p",href:"Extensions/burnable"},"PSP22 Pallet Burnable"),": destruction of own tokens."))}m.isMDXComponent=!0}}]);