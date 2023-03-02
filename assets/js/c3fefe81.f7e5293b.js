"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[61072],{3905:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>f});var r=n(67294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function l(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function i(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var c=r.createContext({}),s=function(e){var t=r.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):l(l({},t),e)),n},p=function(e){var t=s(e.components);return r.createElement(c.Provider,{value:t},e.children)},m="mdxType",u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,c=e.parentName,p=i(e,["components","mdxType","originalType","parentName"]),m=s(n),d=o,f=m["".concat(c,".").concat(d)]||m[d]||u[d]||a;return n?r.createElement(f,l(l({ref:t},p),{},{components:n})):r.createElement(f,l({ref:t},p))}));function f(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,l=new Array(a);l[0]=d;var i={};for(var c in t)hasOwnProperty.call(t,c)&&(i[c]=t[c]);i.originalType=e,i[m]="string"==typeof e?e:o,l[1]=i;for(var s=2;s<a;s++)l[s]=n[s];return r.createElement.apply(null,l)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},69021:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>c,contentTitle:()=>l,default:()=>u,frontMatter:()=>a,metadata:()=>i,toc:()=>s});var r=n(87462),o=(n(67294),n(3905));const a={sidebar_position:7,title:"Timelock Controller"},l=void 0,i={unversionedId:"smart-contracts/timelock-controller",id:"version-1.5.0/smart-contracts/timelock-controller",title:"Timelock Controller",description:"This example shows how you can reuse the implementation of",source:"@site/versioned_docs/version-1.5.0/smart-contracts/timelock-controller.md",sourceDirName:"smart-contracts",slug:"/smart-contracts/timelock-controller",permalink:"/1.5.0/smart-contracts/timelock-controller",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/versioned_docs/version-1.5.0/smart-contracts/timelock-controller.md",tags:[],version:"1.5.0",sidebarPosition:7,frontMatter:{sidebar_position:7,title:"Timelock Controller"},sidebar:"tutorialSidebar",previous:{title:"Payment Splitter",permalink:"/1.5.0/smart-contracts/payment-splitter"},next:{title:"PSP22",permalink:"/1.5.0/smart-contracts/PSP22/"}},c={},s=[{value:"Step 1: Include dependencies",id:"step-1-include-dependencies",level:2},{value:"Step 2: Add imports and enable unstable feature",id:"step-2-add-imports-and-enable-unstable-feature",level:2},{value:"Step 3: Define storage",id:"step-3-define-storage",level:2},{value:"Step 4: Inherit logic",id:"step-4-inherit-logic",level:2},{value:"Step 5: Define constructor",id:"step-5-define-constructor",level:2}],p={toc:s},m="wrapper";function u(e){let{components:t,...n}=e;return(0,o.kt)(m,(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"This example shows how you can reuse the implementation of\n",(0,o.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/governance/timelock_controller"},"timelock-controller"),"."),(0,o.kt)("h2",{id:"step-1-include-dependencies"},"Step 1: Include dependencies"),(0,o.kt)("p",null,"Include ",(0,o.kt)("inlineCode",{parentName:"p"},"brush")," as dependency in the cargo file or you can use ",(0,o.kt)("a",{parentName:"p",href:"/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush"},"default ",(0,o.kt)("inlineCode",{parentName:"a"},"Cargo.toml"))," template.\nAfter you need to enable default implementation of Timelock Controller via ",(0,o.kt)("inlineCode",{parentName:"p"},"brush")," features."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-toml"},'brush = { tag = "v1.5.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["timelock_controller"] }\n')),(0,o.kt)("h2",{id:"step-2-add-imports-and-enable-unstable-feature"},"Step 2: Add imports and enable unstable feature"),(0,o.kt)("p",null,"Use ",(0,o.kt)("inlineCode",{parentName:"p"},"brush::contract")," macro instead of ",(0,o.kt)("inlineCode",{parentName:"p"},"ink::contract"),". Import ",(0,o.kt)("strong",{parentName:"p"},"everything")," from ",(0,o.kt)("inlineCode",{parentName:"p"},"brush::contracts::psp22::utils::token_timelock"),"."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[brush::contract]\npub mod my_psp22_token_timelock {\n    use brush::contracts::psp22::utils::token_timelock::*;\n')),(0,o.kt)("h2",{id:"step-3-define-storage"},"Step 3: Define storage"),(0,o.kt)("p",null,(0,o.kt)("inlineCode",{parentName:"p"},"TimelockController")," is an extension for ",(0,o.kt)("inlineCode",{parentName:"p"},"AccessControl"),", so you need to impl stuff related to both modules.\nDeclare storage struct and declare the fields related to ",(0,o.kt)("inlineCode",{parentName:"p"},"TimelockControllerStorage")," and ",(0,o.kt)("inlineCode",{parentName:"p"},"AccessControlStorage"),".\nThen you need to derive ",(0,o.kt)("inlineCode",{parentName:"p"},"TimelockControllerStorage")," and ",(0,o.kt)("inlineCode",{parentName:"p"},"AccessControlStorage")," traits and mark corresponding fields\nwith ",(0,o.kt)("inlineCode",{parentName:"p"},"#[TimelockControllerStorageField]")," and ",(0,o.kt)("inlineCode",{parentName:"p"},"#[AccessControlStorageField]")," attributes.\nDeriving these traits allows you to reuse the default implementation of ",(0,o.kt)("inlineCode",{parentName:"p"},"TimelockController"),"(and ",(0,o.kt)("inlineCode",{parentName:"p"},"AccessControl"),")."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, AccessControlStorage, TimelockControllerStorage)]\npub struct TimelockStruct {\n   #[AccessControlStorageField]\n   access: AccessControlData,\n   #[TimelockControllerStorageField]\n   timelock: TimelockControllerData,\n}\n")),(0,o.kt)("h2",{id:"step-4-inherit-logic"},"Step 4: Inherit logic"),(0,o.kt)("p",null,"Inherit implementations of ",(0,o.kt)("inlineCode",{parentName:"p"},"TimelockController")," and ",(0,o.kt)("inlineCode",{parentName:"p"},"AccessControl")," traits. You can customize (override) methods in this ",(0,o.kt)("inlineCode",{parentName:"p"},"impl")," block."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"// `TimelockController` is an extension for `AccessControl`, so you need to impl stuff related to both modules.\nimpl AccessControl for TimelockStruct {}\nimpl TimelockController for TimelockStruct {}\n")),(0,o.kt)("h2",{id:"step-5-define-constructor"},"Step 5: Define constructor"),(0,o.kt)("p",null,"Define constructor. Your basic version of ",(0,o.kt)("inlineCode",{parentName:"p"},"TimelockController")," contract is ready!"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"impl TimelockStruct {\n   #[ink(constructor)]\n   pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {\n      let mut instance = Self::default();\n      let caller = instance.env().caller();\n      // `TimelockController` and `AccessControl` have `_init_with_admin` methods.\n      // You need to call it for each trait separately, to initialize everything for these traits.\n      AccessControl::_init_with_admin(&mut instance, caller);\n      TimelockController::_init_with_admin(&mut instance, caller, min_delay, proposers, executors);\n      instance\n   }\n}\n")),(0,o.kt)("p",null,"You can check an example of the usage of ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/timelock_controller"},"TimelockController"),"."))}u.isMDXComponent=!0}}]);