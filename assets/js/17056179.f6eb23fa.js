"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[772],{3905:function(e,n,t){t.d(n,{Zo:function(){return u},kt:function(){return m}});var r=t(7294);function a(e,n,t){return n in e?Object.defineProperty(e,n,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[n]=t,e}function o(e,n){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);n&&(r=r.filter((function(n){return Object.getOwnPropertyDescriptor(e,n).enumerable}))),t.push.apply(t,r)}return t}function i(e){for(var n=1;n<arguments.length;n++){var t=null!=arguments[n]?arguments[n]:{};n%2?o(Object(t),!0).forEach((function(n){a(e,n,t[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):o(Object(t)).forEach((function(n){Object.defineProperty(e,n,Object.getOwnPropertyDescriptor(t,n))}))}return e}function l(e,n){if(null==e)return{};var t,r,a=function(e,n){if(null==e)return{};var t,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)t=o[r],n.indexOf(t)>=0||(a[t]=e[t]);return a}(e,n);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)t=o[r],n.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(a[t]=e[t])}return a}var c=r.createContext({}),s=function(e){var n=r.useContext(c),t=n;return e&&(t="function"==typeof e?e(n):i(i({},n),e)),t},u=function(e){var n=s(e.components);return r.createElement(c.Provider,{value:n},e.children)},p={inlineCode:"code",wrapper:function(e){var n=e.children;return r.createElement(r.Fragment,{},n)}},d=r.forwardRef((function(e,n){var t=e.components,a=e.mdxType,o=e.originalType,c=e.parentName,u=l(e,["components","mdxType","originalType","parentName"]),d=s(t),m=a,f=d["".concat(c,".").concat(m)]||d[m]||p[m]||o;return t?r.createElement(f,i(i({ref:n},u),{},{components:t})):r.createElement(f,i({ref:n},u))}));function m(e,n){var t=arguments,a=n&&n.mdxType;if("string"==typeof e||a){var o=t.length,i=new Array(o);i[0]=d;var l={};for(var c in n)hasOwnProperty.call(n,c)&&(l[c]=n[c]);l.originalType=e,l.mdxType="string"==typeof e?e:a,i[1]=l;for(var s=2;s<o;s++)i[s]=t[s];return r.createElement.apply(null,i)}return r.createElement.apply(null,t)}d.displayName="MDXCreateElement"},9053:function(e,n,t){t.r(n),t.d(n,{contentTitle:function(){return c},default:function(){return d},frontMatter:function(){return l},metadata:function(){return s},toc:function(){return u}});var r=t(7462),a=t(3366),o=(t(7294),t(3905)),i=["components"],l={sidebar_position:3,title:"Ownable"},c=void 0,s={unversionedId:"smart-contracts/ownable",id:"smart-contracts/ownable",isDocsHomePage:!1,title:"Ownable",description:"This example shows how you can use the implementation of ownable to provide only owner rights for contract's functions.",source:"@site/docs/smart-contracts/ownable.md",sourceDirName:"smart-contracts",slug:"/smart-contracts/ownable",permalink:"/smart-contracts/ownable",editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/docs/smart-contracts/ownable.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{sidebar_position:3,title:"Ownable"},sidebar:"tutorialSidebar",previous:{title:"Access Control",permalink:"/smart-contracts/access-control"},next:{title:"Proxy",permalink:"/smart-contracts/proxy"}},u=[{value:"Step 1: Include dependencies",id:"step-1-include-dependencies",children:[]},{value:"Step 2: Add imports and enable unstable feature",id:"step-2-add-imports-and-enable-unstable-feature",children:[]},{value:"Step 3: Define storage",id:"step-3-define-storage",children:[]},{value:"Step 4: Inherit logic",id:"step-4-inherit-logic",children:[]},{value:"Step 5: Define constructor",id:"step-5-define-constructor",children:[]},{value:"Step 6: Customize your contract",id:"step-6-customize-your-contract",children:[]}],p={toc:u};function d(e){var n=e.components,t=(0,a.Z)(e,i);return(0,o.kt)("wrapper",(0,r.Z)({},p,t,{components:n,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"This example shows how you can use the implementation of ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/access/ownable"},"ownable")," to provide ",(0,o.kt)("inlineCode",{parentName:"p"},"only owner")," rights for contract's functions."),(0,o.kt)("h2",{id:"step-1-include-dependencies"},"Step 1: Include dependencies"),(0,o.kt)("p",null,"Include ",(0,o.kt)("inlineCode",{parentName:"p"},"brush")," as dependency in the cargo file or you can use ",(0,o.kt)("a",{parentName:"p",href:"/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush"},"default ",(0,o.kt)("inlineCode",{parentName:"a"},"Cargo.toml"))," template.\nAfter you need to enable default implementation of Ownable via ",(0,o.kt)("inlineCode",{parentName:"p"},"brush")," features."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-toml"},'brush = { tag = "v1.4.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["ownable"] }\n')),(0,o.kt)("h2",{id:"step-2-add-imports-and-enable-unstable-feature"},"Step 2: Add imports and enable unstable feature"),(0,o.kt)("p",null,"Use ",(0,o.kt)("inlineCode",{parentName:"p"},"brush::contract")," macro instead of ",(0,o.kt)("inlineCode",{parentName:"p"},"ink::contract"),". Import ",(0,o.kt)("strong",{parentName:"p"},"everything")," from ",(0,o.kt)("inlineCode",{parentName:"p"},"brush::contracts::ownable"),"."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[brush::contract]\npub mod my_ownable {\n    use brush::{\n        contracts::ownable::*,\n        modifiers,\n    };\n...\n')),(0,o.kt)("h2",{id:"step-3-define-storage"},"Step 3: Define storage"),(0,o.kt)("p",null,"Declare storage struct and declare the field related to ",(0,o.kt)("inlineCode",{parentName:"p"},"OwnableStorage")," trait. Then you need to derive the ",(0,o.kt)("inlineCode",{parentName:"p"},"OwnableStorage")," trait and mark the corresponding field with the ",(0,o.kt)("inlineCode",{parentName:"p"},"#[OwnableStorageField]")," attribute. Deriving this trait allows you to reuse the default implementation of ",(0,o.kt)("inlineCode",{parentName:"p"},"Ownable"),"."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, OwnableStorage)]\npub struct MyOwnable {\n    #[OwnableStorageField]\n    ownable: OwnableData,\n}\n")),(0,o.kt)("h2",{id:"step-4-inherit-logic"},"Step 4: Inherit logic"),(0,o.kt)("p",null,"Inherit implementation of the ",(0,o.kt)("inlineCode",{parentName:"p"},"Ownable")," trait. You can customize (override) methods in this ",(0,o.kt)("inlineCode",{parentName:"p"},"impl")," block."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"impl Ownable for MyOwnable {}\n")),(0,o.kt)("h2",{id:"step-5-define-constructor"},"Step 5: Define constructor"),(0,o.kt)("p",null,"Define the constructor and initialize the owner with the contract initiator. Your basic version of ",(0,o.kt)("inlineCode",{parentName:"p"},"Ownable")," contract is ready!"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"impl MyOwnable {\n    #[ink(constructor)]\n    pub fn new() -> Self {\n        let mut instance = Self::default();\n        let caller = instance.env().caller();\n        instance._init_with_owner(caller);\n        instance\n    }\n}\n")),(0,o.kt)("h2",{id:"step-6-customize-your-contract"},"Step 6: Customize your contract"),(0,o.kt)("p",null,"Customize it by adding ownable logic. We will add a ",(0,o.kt)("inlineCode",{parentName:"p"},"owner_function")," to ",(0,o.kt)("inlineCode",{parentName:"p"},"MyOwnable")," implemenation and add the ",(0,o.kt)("inlineCode",{parentName:"p"},"only_owner")," modifier, which will verify that the caller of the function is the owner."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[brush::contract]\npub mod my_ownable {\n    use brush::{\n        contracts::ownable::*,\n        modifiers,\n    };\n\n   #[ink(storage)]\n   #[derive(Default, OwnableStorage)]\n   pub struct MyOwnable {\n      #[OwnableStorageField]\n      ownable: OwnableData,\n   }\n\n   impl Ownable for MyOwnable {}\n    \n   impl MyOwnable {\n      \n      #[ink(constructor)]\n      pub fn new() -> Self {\n         let mut instance = Self::default();\n         let caller = instance.env().caller();\n         instance._init_with_owner(caller);\n         instance\n      }\n\n      #[ink(message)]\n      #[modifiers(only_owner)]\n      pub fn owner_function(&mut self) -> Result<(), OwnableError> {\n         todo!()\n      }\n   }\n}\n\n')),(0,o.kt)("p",null,"You can check an example of the usage of ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/ownable"},"Ownable"),"."))}d.isMDXComponent=!0}}]);