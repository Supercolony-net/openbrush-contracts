"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[1252],{3905:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>m});var r=n(67294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function p(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function i(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var s=r.createContext({}),l=function(e){var t=r.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):p(p({},t),e)),n},c=function(e){var t=l(e.components);return r.createElement(s.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,s=e.parentName,c=i(e,["components","mdxType","originalType","parentName"]),d=l(n),m=a,P=d["".concat(s,".").concat(m)]||d[m]||u[m]||o;return n?r.createElement(P,p(p({ref:t},c),{},{components:n})):r.createElement(P,p({ref:t},c))}));function m(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,p=new Array(o);p[0]=d;var i={};for(var s in t)hasOwnProperty.call(t,s)&&(i[s]=t[s]);i.originalType=e,i.mdxType="string"==typeof e?e:a,p[1]=i;for(var l=2;l<o;l++)p[l]=n[l];return r.createElement.apply(null,p)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},10945:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>p,default:()=>u,frontMatter:()=>o,metadata:()=>i,toc:()=>l});var r=n(87462),a=(n(67294),n(3905));const o={sidebar_position:4,title:"PSP22 Wrapper"},p=void 0,i={unversionedId:"smart-contracts/PSP22/Extensions/wrapper",id:"version-1.3.0/smart-contracts/PSP22/Extensions/wrapper",title:"PSP22 Wrapper",description:"This example shows how you can reuse the implementation of PSP22 token with PSP22 Wrapper extension, which allows you to wrap your PSP22 token in a PSP22Wrapper token which can be used for example for governance.",source:"@site/versioned_docs/version-1.3.0/smart-contracts/PSP22/Extensions/wrapper.md",sourceDirName:"smart-contracts/PSP22/Extensions",slug:"/smart-contracts/PSP22/Extensions/wrapper",permalink:"/1.3.0/smart-contracts/PSP22/Extensions/wrapper",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/versioned_docs/version-1.3.0/smart-contracts/PSP22/Extensions/wrapper.md",tags:[],version:"1.3.0",sidebarPosition:4,frontMatter:{sidebar_position:4,title:"PSP22 Wrapper"},sidebar:"tutorialSidebar",previous:{title:"PSP22 Burnable",permalink:"/1.3.0/smart-contracts/PSP22/Extensions/burnable"},next:{title:"PSP22 FlashMint",permalink:"/1.3.0/smart-contracts/PSP22/Extensions/flashmint"}},s={},l=[{value:"Step 1: Include dependencies",id:"step-1-include-dependencies",level:2},{value:"Step 2: Add imports and enable unstable feature",id:"step-2-add-imports-and-enable-unstable-feature",level:2},{value:"Step 3: Define storage",id:"step-3-define-storage",level:2},{value:"Step 4: Inherit logic",id:"step-4-inherit-logic",level:2},{value:"Step 5: Define constructor",id:"step-5-define-constructor",level:2}],c={toc:l};function u(e){let{components:t,...n}=e;return(0,a.kt)("wrapper",(0,r.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("p",null,"This example shows how you can reuse the implementation of ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22"},"PSP22")," token with ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22/extensions/wrapper.rs"},"PSP22 Wrapper")," extension, which allows you to wrap your ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22")," token in a ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22Wrapper")," token which can be used for example for governance."),(0,a.kt)("h2",{id:"step-1-include-dependencies"},"Step 1: Include dependencies"),(0,a.kt)("p",null,"Include ",(0,a.kt)("inlineCode",{parentName:"p"},"brush")," as dependency in the cargo file or you can use ",(0,a.kt)("a",{parentName:"p",href:"/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush"},"default ",(0,a.kt)("inlineCode",{parentName:"a"},"Cargo.toml"))," template.\nAfter you need to enable default implementation of PSP22 via ",(0,a.kt)("inlineCode",{parentName:"p"},"brush")," features."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-toml"},'brush = { tag = "v1.3.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["psp22"] }\n')),(0,a.kt)("h2",{id:"step-2-add-imports-and-enable-unstable-feature"},"Step 2: Add imports and enable unstable feature"),(0,a.kt)("p",null,"Use ",(0,a.kt)("inlineCode",{parentName:"p"},"brush::contract")," macro instead of ",(0,a.kt)("inlineCode",{parentName:"p"},"ink::contract"),". Import ",(0,a.kt)("strong",{parentName:"p"},"everything")," from ",(0,a.kt)("inlineCode",{parentName:"p"},"brush::contracts::psp22::extensions::wrapper"),"."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[brush::contract]\npub mod my_psp22_wrapper {\n    use brush::contracts::psp22::extensions::wrapper::*;\n...\n')),(0,a.kt)("h2",{id:"step-3-define-storage"},"Step 3: Define storage"),(0,a.kt)("p",null,"Declare storage struct and declare the fields related to ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22Storage")," and ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22WrapperStorage")," traits. Then you need to derive ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22Storage")," and ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22WrapperStorage")," traits and mark corresponding fields with ",(0,a.kt)("inlineCode",{parentName:"p"},"#[PSP22StorageField]")," and ",(0,a.kt)("inlineCode",{parentName:"p"},"#[PSP22WrapperStorageField]")," attributes. Deriving these traits allows you to reuse the default implementation of ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22")," and ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22Wrapper"),"."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, PSP22WrapperStorage, PSP22Storage)]\npub struct MyPSP22Wrapper {\n    #[PSP22StorageField]\n    psp22: PSP22Data,\n    #[PSP22WrapperStorageField]\n    wrapper: PSP22WrapperData,\n}\n")),(0,a.kt)("h2",{id:"step-4-inherit-logic"},"Step 4: Inherit logic"),(0,a.kt)("p",null,"Inherit implementations of ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22")," and ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22Wrapper")," traits. You can customize (override) methods in this ",(0,a.kt)("inlineCode",{parentName:"p"},"impl")," block."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"impl PSP22 for MyPSP22Wrapper {}\n\nimpl PSP22Wrapper for MyPSP22Wrapper {}\n")),(0,a.kt)("h2",{id:"step-5-define-constructor"},"Step 5: Define constructor"),(0,a.kt)("p",null,"Define constructor. Your implementation of ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22Wrapper")," contract is ready!"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"impl MyPSP22 {\n   #[ink(constructor)]\n   pub fn new(token_address: AccountId) -> Self {\n        let mut instance = Self::default();\n        instance.init(token_address);\n        instance\n    }\n}\n")),(0,a.kt)("p",null,"You can check an example of the usage of ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/wrapper"},"PSP22 Wrapper"),"."),(0,a.kt)("p",null,"You can also check the documentation for the basic implementation of ",(0,a.kt)("a",{parentName:"p",href:"/1.3.0/smart-contracts/PSP22/"},"PSP22"),"."))}u.isMDXComponent=!0}}]);