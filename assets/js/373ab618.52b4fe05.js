"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[55454],{3905:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>P});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var p=a.createContext({}),l=function(e){var t=a.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},c=function(e){var t=l(e.components);return a.createElement(p.Provider,{value:t},e.children)},u="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,p=e.parentName,c=s(e,["components","mdxType","originalType","parentName"]),u=l(n),m=r,P=u["".concat(p,".").concat(m)]||u[m]||d[m]||o;return n?a.createElement(P,i(i({ref:t},c),{},{components:n})):a.createElement(P,i({ref:t},c))}));function P(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,i=new Array(o);i[0]=m;var s={};for(var p in t)hasOwnProperty.call(t,p)&&(s[p]=t[p]);s.originalType=e,s[u]="string"==typeof e?e:r,i[1]=s;for(var l=2;l<o;l++)i[l]=n[l];return a.createElement.apply(null,i)}return a.createElement.apply(null,n)}m.displayName="MDXCreateElement"},57675:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>i,default:()=>d,frontMatter:()=>o,metadata:()=>s,toc:()=>l});var a=n(87462),r=(n(67294),n(3905));const o={sidebar_position:1,title:"PSP34 Metadata"},i=void 0,s={unversionedId:"smart-contracts/PSP34/Extensions/metadata",id:"version-1.7.0/smart-contracts/PSP34/Extensions/metadata",title:"PSP34 Metadata",description:"This example shows how you can reuse the implementation of PSP34 token with PSP34Metadata extension.",source:"@site/versioned_docs/version-1.7.0/smart-contracts/PSP34/Extensions/metadata.md",sourceDirName:"smart-contracts/PSP34/Extensions",slug:"/smart-contracts/PSP34/Extensions/metadata",permalink:"/1.7.0/smart-contracts/PSP34/Extensions/metadata",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/versioned_docs/version-1.7.0/smart-contracts/PSP34/Extensions/metadata.md",tags:[],version:"1.7.0",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"PSP34 Metadata"},sidebar:"tutorialSidebar",previous:{title:"PSP34",permalink:"/1.7.0/smart-contracts/PSP34/"},next:{title:"PSP34 Mintable",permalink:"/1.7.0/smart-contracts/PSP34/Extensions/mintable"}},p={},l=[{value:"Step 1: Include dependencies",id:"step-1-include-dependencies",level:2},{value:"Step 2: Add imports and enable unstable feature",id:"step-2-add-imports-and-enable-unstable-feature",level:2},{value:"Step 3: Define storage",id:"step-3-define-storage",level:2},{value:"Step 4: Inherit logic",id:"step-4-inherit-logic",level:2},{value:"Step 5: Define constructor",id:"step-5-define-constructor",level:2}],c={toc:l},u="wrapper";function d(e){let{components:t,...n}=e;return(0,r.kt)(u,(0,a.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"This example shows how you can reuse the implementation of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp34"},"PSP34")," token with ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp34/src/extensions/metadata.rs"},"PSP34Metadata")," extension."),(0,r.kt)("h2",{id:"step-1-include-dependencies"},"Step 1: Include dependencies"),(0,r.kt)("p",null,"Include ",(0,r.kt)("inlineCode",{parentName:"p"},"brush")," as dependency in the cargo file or you can use ",(0,r.kt)("a",{parentName:"p",href:"../../overview#the-default-toml-of-your-project-with-openbrush"},"default ",(0,r.kt)("inlineCode",{parentName:"a"},"Cargo.toml"))," template.\nAfter you need to enable default implementation of PSP34 via ",(0,r.kt)("inlineCode",{parentName:"p"},"brush")," features."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-toml"},'brush = { tag = "v1.7.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["psp34"] }\n')),(0,r.kt)("h2",{id:"step-2-add-imports-and-enable-unstable-feature"},"Step 2: Add imports and enable unstable feature"),(0,r.kt)("p",null,"Use ",(0,r.kt)("inlineCode",{parentName:"p"},"brush::contract")," macro instead of ",(0,r.kt)("inlineCode",{parentName:"p"},"ink::contract"),". Import ",(0,r.kt)("strong",{parentName:"p"},"everything")," from ",(0,r.kt)("inlineCode",{parentName:"p"},"brush::contracts::psp34::extensions::metadata"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[brush::contract]\npub mod my_psp34_metadata {\n    use brush::contracts::psp34::extensions::metadata::*;\n    use ink_prelude::string::String;\n    use ink_storage::traits::SpreadAllocate;\n...\n')),(0,r.kt)("h2",{id:"step-3-define-storage"},"Step 3: Define storage"),(0,r.kt)("p",null,"Declare storage struct and declare the field related to the ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP34MetadataStorage")," trait in addition to your ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP34Storage")," field. Then you need to derive the ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP34MetadataStorage")," trait and mark the corresponding field with the ",(0,r.kt)("inlineCode",{parentName:"p"},"#[PSP34MetadataStorageField]")," attribute. Deriving this trait allows you to reuse the ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP34Metadata")," extension in your ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP34")," implementation."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage)]\npub struct MyPSP34 {\n    #[PSP34StorageField]\n    psp34: PSP34Data,\n    #[PSP34MetadataStorageField]\n    metadata: PSP34MetadataData,\n}\n")),(0,r.kt)("h2",{id:"step-4-inherit-logic"},"Step 4: Inherit logic"),(0,r.kt)("p",null,"Inherit implementation of the ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP34Metadata")," trait. You can customize (override) methods in this ",(0,r.kt)("inlineCode",{parentName:"p"},"impl")," block."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl PSP34 for MyPSP34 {}\nimpl PSP34Metadata for MyPSP34 {}\n// Optionally you can add more default implementations\nimpl PSP34Internal for MyPSP34 {}\nimpl PSP34MetadataInternal for MYPSP34 {}\n")),(0,r.kt)("h2",{id:"step-5-define-constructor"},"Step 5: Define constructor"),(0,r.kt)("p",null,"Define constructor. Your ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP34Metadata")," contract is ready!"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'impl MyPSP34 {\n    #[ink(constructor)]\n    pub fn new(id: Id, name: String, symbol: String) -> Self {\n        ink_lang::codegen::initialize_contract(|instance: &mut Self| {\n            instance._set_attribute(id.clone(), String::from("name").into_bytes(), name.into_bytes());\n            instance._set_attribute(id, String::from("symbol").into_bytes(), symbol.into_bytes());\n        }\n    }\n}\n')),(0,r.kt)("p",null,"You can check an example of the usage of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp34_extensions/metadata"},"PSP34 Metadata"),"."),(0,r.kt)("p",null,"You can also check the documentation for the basic implementation of ",(0,r.kt)("a",{parentName:"p",href:"/1.7.0/smart-contracts/PSP34/"},"PSP34"),"."))}d.isMDXComponent=!0}}]);