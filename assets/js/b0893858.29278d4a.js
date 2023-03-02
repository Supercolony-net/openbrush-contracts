"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[90049],{3905:(e,t,n)=>{n.d(t,{Zo:()=>u,kt:()=>f});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function l(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var i=a.createContext({}),p=function(e){var t=a.useContext(i),n=t;return e&&(n="function"==typeof e?e(t):l(l({},t),e)),n},u=function(e){var t=p(e.components);return a.createElement(i.Provider,{value:t},e.children)},c="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,i=e.parentName,u=s(e,["components","mdxType","originalType","parentName"]),c=p(n),m=r,f=c["".concat(i,".").concat(m)]||c[m]||d[m]||o;return n?a.createElement(f,l(l({ref:t},u),{},{components:n})):a.createElement(f,l({ref:t},u))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,l=new Array(o);l[0]=m;var s={};for(var i in t)hasOwnProperty.call(t,i)&&(s[i]=t[i]);s.originalType=e,s[c]="string"==typeof e?e:r,l[1]=s;for(var p=2;p<o;p++)l[p]=n[p];return a.createElement.apply(null,l)}return a.createElement.apply(null,n)}m.displayName="MDXCreateElement"},63283:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>i,contentTitle:()=>l,default:()=>d,frontMatter:()=>o,metadata:()=>s,toc:()=>p});var a=n(87462),r=(n(67294),n(3905));const o={sidebar_position:5,title:"Pausable"},l=void 0,s={unversionedId:"smart-contracts/pausable",id:"version-1.4.0/smart-contracts/pausable",title:"Pausable",description:"This example shows how you can reuse the implementation of",source:"@site/versioned_docs/version-1.4.0/smart-contracts/pausable.md",sourceDirName:"smart-contracts",slug:"/smart-contracts/pausable",permalink:"/1.4.0/smart-contracts/pausable",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/versioned_docs/version-1.4.0/smart-contracts/pausable.md",tags:[],version:"1.4.0",sidebarPosition:5,frontMatter:{sidebar_position:5,title:"Pausable"},sidebar:"tutorialSidebar",previous:{title:"Reentrancy Guard",permalink:"/1.4.0/smart-contracts/reentrancy-guard"},next:{title:"Payment Splitter",permalink:"/1.4.0/smart-contracts/payment-splitter"}},i={},p=[{value:"Step 1: Include dependencies",id:"step-1-include-dependencies",level:2},{value:"Step 2: Add imports and enable unstable feature",id:"step-2-add-imports-and-enable-unstable-feature",level:2},{value:"Step 3: Define storage",id:"step-3-define-storage",level:2},{value:"Step 4: Inherit logic",id:"step-4-inherit-logic",level:2},{value:"Step 5: Define constructor",id:"step-5-define-constructor",level:2},{value:"Step 6: Customize your contract",id:"step-6-customize-your-contract",level:2}],u={toc:p},c="wrapper";function d(e){let{components:t,...n}=e;return(0,r.kt)(c,(0,a.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"This example shows how you can reuse the implementation of\n",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/security/pausable"},"pausable")," in ",(0,r.kt)("inlineCode",{parentName:"p"},"Flipper")," contract to ",(0,r.kt)("inlineCode",{parentName:"p"},"flip")," only if the contract is not paused."),(0,r.kt)("h2",{id:"step-1-include-dependencies"},"Step 1: Include dependencies"),(0,r.kt)("p",null,"Include ",(0,r.kt)("inlineCode",{parentName:"p"},"brush")," as dependency in the cargo file or you can use ",(0,r.kt)("a",{parentName:"p",href:"/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush"},"default ",(0,r.kt)("inlineCode",{parentName:"a"},"Cargo.toml"))," template.\nAfter you need to enable default implementation of Pausable via ",(0,r.kt)("inlineCode",{parentName:"p"},"brush")," features."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-toml"},'brush = { tag = "v1.4.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["pausable"] }\n')),(0,r.kt)("h2",{id:"step-2-add-imports-and-enable-unstable-feature"},"Step 2: Add imports and enable unstable feature"),(0,r.kt)("p",null,"Use ",(0,r.kt)("inlineCode",{parentName:"p"},"brush::contract")," macro instead of ",(0,r.kt)("inlineCode",{parentName:"p"},"ink::contract"),". Import ",(0,r.kt)("strong",{parentName:"p"},"everything")," from ",(0,r.kt)("inlineCode",{parentName:"p"},"brush::contracts::pausable"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[brush::contract]\npub mod my_pausable {\n    use brush::contracts::pausable::*;\n...\n')),(0,r.kt)("h2",{id:"step-3-define-storage"},"Step 3: Define storage"),(0,r.kt)("p",null,"Declare storage struct and declare the field related to ",(0,r.kt)("inlineCode",{parentName:"p"},"PausableStorage"),".\nThen you need to derive ",(0,r.kt)("inlineCode",{parentName:"p"},"PausableStorage")," trait and mark corresponding field\nwith ",(0,r.kt)("inlineCode",{parentName:"p"},"#[PausableStorageField]")," attribute. Deriving this trait allows you to reuse\nthe default implementation of ",(0,r.kt)("inlineCode",{parentName:"p"},"Pausable"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, PausableStorage)]\npub struct MyFlipper {\n   #[PausableStorageField]\n   pause: PausableData,\n   flipped: bool,\n}\n")),(0,r.kt)("h2",{id:"step-4-inherit-logic"},"Step 4: Inherit logic"),(0,r.kt)("p",null,"Inherit the implementation of ",(0,r.kt)("inlineCode",{parentName:"p"},"Pausable"),". You can customize (override) methods in this ",(0,r.kt)("inlineCode",{parentName:"p"},"impl")," block."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl Pausable for MyFlipper {}\n")),(0,r.kt)("h2",{id:"step-5-define-constructor"},"Step 5: Define constructor"),(0,r.kt)("p",null,"Define constructor. Your basic version of ",(0,r.kt)("inlineCode",{parentName:"p"},"Pausable")," contract is ready!"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl MyFlipper {\n   #[ink(constructor)]\n   pub fn new() -> Self {\n      Self::default()\n   }\n}\n")),(0,r.kt)("h2",{id:"step-6-customize-your-contract"},"Step 6: Customize your contract"),(0,r.kt)("p",null,"Customize it by adding flipper logic. We will implement ",(0,r.kt)("inlineCode",{parentName:"p"},"flip")," method marked with ",(0,r.kt)("inlineCode",{parentName:"p"},"when_not_paused")," modifier."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[brush::contract]\npub mod my_pausable {\n    use brush::contracts::pausable::*;\n\n    #[ink(storage)]\n    #[derive(Default, PausableStorage)]\n    pub struct MyFlipper {\n        #[PausableStorageField]\n        pause: PausableData,\n        flipped: bool,\n    }\n\n    impl MyFlipper {\n        #[ink(constructor)]\n        pub fn new() -> Self {\n            Self::default()\n        }\n\n        #[ink(message)]\n        #[brush::modifiers(when_not_paused)]\n        pub fn flip(&mut self) -> Result<(), PausableError> {\n            self.flipped = !self.flipped;\n            Ok(())\n        }\n\n        #[ink(message)]\n        pub fn pause(&mut self) -> Result<(), PausableError> {\n            self._pause()\n        }\n\n        #[ink(message)]\n        pub fn unpause(&mut self) -> Result<(), PausableError> {\n            self._unpause()\n        }\n    }\n\n    impl Pausable for MyFlipper {}\n}\n')),(0,r.kt)("p",null,"You can check an example of the usage of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/pausable"},"Pausable"),"."))}d.isMDXComponent=!0}}]);