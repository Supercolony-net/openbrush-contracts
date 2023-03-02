"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[53717],{3905:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>d});var r=n(67294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var l=r.createContext({}),s=function(e){var t=r.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},p=function(e){var t=s(e.components);return r.createElement(l.Provider,{value:t},e.children)},u="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},f=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,l=e.parentName,p=c(e,["components","mdxType","originalType","parentName"]),u=s(n),f=o,d=u["".concat(l,".").concat(f)]||u[f]||m[f]||a;return n?r.createElement(d,i(i({ref:t},p),{},{components:n})):r.createElement(d,i({ref:t},p))}));function d(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,i=new Array(a);i[0]=f;var c={};for(var l in t)hasOwnProperty.call(t,l)&&(c[l]=t[l]);c.originalType=e,c[u]="string"==typeof e?e:o,i[1]=c;for(var s=2;s<a;s++)i[s]=n[s];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}f.displayName="MDXCreateElement"},39949:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>i,default:()=>m,frontMatter:()=>a,metadata:()=>c,toc:()=>s});var r=n(87462),o=(n(67294),n(3905));const a={sidebar_position:3,title:"Ownable"},i=void 0,c={unversionedId:"smart-contracts/ownable",id:"version-v2.3.0/smart-contracts/ownable",title:"Ownable",description:"This example shows how you can use the implementation of ownable to provide only owner rights for contract's functions.",source:"@site/versioned_docs/version-v2.3.0/smart-contracts/ownable.md",sourceDirName:"smart-contracts",slug:"/smart-contracts/ownable",permalink:"/smart-contracts/ownable",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/versioned_docs/version-v2.3.0/smart-contracts/ownable.md",tags:[],version:"v2.3.0",sidebarPosition:3,frontMatter:{sidebar_position:3,title:"Ownable"},sidebar:"tutorialSidebar",previous:{title:"AccessControl Enumerable",permalink:"/smart-contracts/access-control/Extensions/enumerable"},next:{title:"Proxy",permalink:"/smart-contracts/proxy"}},l={},s=[{value:"Step 1: Import default implementation",id:"step-1-import-default-implementation",level:2},{value:"Step 2: Define constructor",id:"step-2-define-constructor",level:2},{value:"Step 3: Customize your contract",id:"step-3-customize-your-contract",level:2}],p={toc:s},u="wrapper";function m(e){let{components:t,...n}=e;return(0,o.kt)(u,(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"This example shows how you can use the implementation of ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/access/ownable"},"ownable")," to provide ",(0,o.kt)("inlineCode",{parentName:"p"},"only owner")," rights for contract's functions."),(0,o.kt)("h2",{id:"step-1-import-default-implementation"},"Step 1: Import default implementation"),(0,o.kt)("p",null,"With ",(0,o.kt)("a",{parentName:"p",href:"/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush"},"default ",(0,o.kt)("inlineCode",{parentName:"a"},"Cargo.toml")),",\nyou need to import the ",(0,o.kt)("inlineCode",{parentName:"p"},"ownable")," module, enable the corresponding feature, and embed the module data structure\nas described in ",(0,o.kt)("a",{parentName:"p",href:"/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush"},"that section"),"."),(0,o.kt)("p",null,"The main trait is ",(0,o.kt)("inlineCode",{parentName:"p"},"Ownable"),"."),(0,o.kt)("h2",{id:"step-2-define-constructor"},"Step 2: Define constructor"),(0,o.kt)("p",null,"Define the constructor and initialize the owner with the contract initiator."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"impl Contract {\n    #[ink(constructor)]\n    pub fn new() -> Self {\n        ink_lang::codegen::initialize_contract(|instance: &mut Self| {\n            let caller = instance.env().caller();\n            instance._init_with_owner(caller);\n        })\n    }\n}\n")),(0,o.kt)("h2",{id:"step-3-customize-your-contract"},"Step 3: Customize your contract"),(0,o.kt)("p",null,"Customize it by adding ownable logic. We will add a ",(0,o.kt)("inlineCode",{parentName:"p"},"owner_function")," to ",(0,o.kt)("inlineCode",{parentName:"p"},"MyOwnable")," implemenation\nand add the ",(0,o.kt)("inlineCode",{parentName:"p"},"only_owner")," modifier, which will verify that the caller of the function is the owner."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[openbrush::contract]\npub mod my_ownable {\n    use openbrush::{\n        contracts::ownable::*,\n        modifiers,\n    };\n    use ink_storage::traits::SpreadAllocate;\n    use openbrush::traits::Storage;\n\n\n   #[ink(storage)]\n   #[derive(Default, SpreadAllocate, Storage)]\n   pub struct Contract {\n      #[storage_field]\n      ownable: ownable::Data,\n   }\n\n   impl Ownable for Contract {}\n    \n   impl Contract {\n      #[ink(constructor)]\n      pub fn new() -> Self {\n        ink_lang::codegen::initialize_contract(|instance: &mut Self| {\n            let caller = instance.env().caller();\n            instance._init_with_owner(caller);\n        })\n      }\n\n      #[ink(message)]\n      #[modifiers(only_owner)]\n      pub fn owner_function(&mut self) -> Result<(), OwnableError> {\n         todo!()\n      }\n   }\n}\n\n')),(0,o.kt)("p",null,"You can check an example of the usage of ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/ownable"},"Ownable"),"."))}m.isMDXComponent=!0}}]);