"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[77382],{3905:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>f});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var s=a.createContext({}),p=function(e){var t=a.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},c=function(e){var t=p(e.components);return a.createElement(s.Provider,{value:t},e.children)},m="mdxType",u={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},d=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,s=e.parentName,c=l(e,["components","mdxType","originalType","parentName"]),m=p(n),d=r,f=m["".concat(s,".").concat(d)]||m[d]||u[d]||o;return n?a.createElement(f,i(i({ref:t},c),{},{components:n})):a.createElement(f,i({ref:t},c))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,i=new Array(o);i[0]=d;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l[m]="string"==typeof e?e:r,i[1]=l;for(var p=2;p<o;p++)i[p]=n[p];return a.createElement.apply(null,i)}return a.createElement.apply(null,n)}d.displayName="MDXCreateElement"},80444:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>i,default:()=>u,frontMatter:()=>o,metadata:()=>l,toc:()=>p});var a=n(87462),r=(n(67294),n(3905));const o={sidebar_position:6,title:"Data and derive macro"},i=void 0,l={unversionedId:"smart-contracts/example/data",id:"version-2.0.0/smart-contracts/example/data",title:"Data and derive macro",description:"Data segregation",source:"@site/versioned_docs/version-2.0.0/smart-contracts/example/data.md",sourceDirName:"smart-contracts/example",slug:"/smart-contracts/example/data",permalink:"/2.0.0/smart-contracts/example/data",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/versioned_docs/version-2.0.0/smart-contracts/example/data.md",tags:[],version:"2.0.0",sidebarPosition:6,frontMatter:{sidebar_position:6,title:"Data and derive macro"},sidebar:"tutorialSidebar",previous:{title:"Loan contract",permalink:"/2.0.0/smart-contracts/example/loan"},next:{title:"Lending impls",permalink:"/2.0.0/smart-contracts/example/impls"}},s={},p=[{value:"Data segregation",id:"data-segregation",level:2},{value:"Storage trait",id:"storage-trait",level:3},{value:"Data of the trait",id:"data-of-the-trait",level:3},{value:"Default implementation",id:"default-implementation",level:3},{value:"&quot;Inheritance&quot; of the implementation",id:"inheritance-of-the-implementation",level:3},{value:"Macros from OpenBrush",id:"macros-from-openbrush",level:2}],c={toc:p},m="wrapper";function u(e){let{components:t,...n}=e;return(0,r.kt)(m,(0,a.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("h2",{id:"data-segregation"},"Data segregation"),(0,r.kt)("p",null,'Rust doesn\'t have inheritance like OOP languages.\nIf you want to "inherit" some fields, you can use structural composition.\nIf you want to "inherit" some implementation, you can use traits. Traits can have a ',(0,r.kt)("a",{parentName:"p",href:"https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations"},"default implementation")," or a ",(0,r.kt)("a",{parentName:"p",href:"https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods"},"generic implementation"),".\nThe traits in Rust can't contain fields, it is pure interfaces."),(0,r.kt)("p",null,"Based on that information we propose you the following concept of smart contract development:"),(0,r.kt)("h3",{id:"storage-trait"},"Storage trait"),(0,r.kt)("p",null,"Extract the logic of data storing into a separate trait to have the ability to define the default implementation without knowing what contract will inherit that. That trait can have a simple naming like ",(0,r.kt)("inlineCode",{parentName:"p"},"NAME_OF_ORIGINAL_TRAIT")," + ",(0,r.kt)("inlineCode",{parentName:"p"},"Storage")," suffix."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"pub trait PointStorage {\n    fn get(&self) -> & PointData;\n    fn get_mut(&mut self) -> &mut PointData;\n}\n")),(0,r.kt)("h3",{id:"data-of-the-trait"},"Data of the trait"),(0,r.kt)("p",null,"That trait returns some data with fields that can be used in the implementation. The data is a simple struct with fields. Later that struct can be embedded into the contract struct."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"pub struct PointData {\n    pub x: u32,\n    pub y: u32,\n}\n")),(0,r.kt)("h3",{id:"default-implementation"},"Default implementation"),(0,r.kt)("p",null,"Define the default or generic implementation for your main trait with the restriction that ",(0,r.kt)("inlineCode",{parentName:"p"},"Self")," should also implement storage trait."),(0,r.kt)("p",null,"A default implementation:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'pub trait Point: PointStorage {\n    fn x(&self) -> u32 {\n        PointStorage::get(self).x\n    }\n    \n    fn y(&self) -> u32 {\n        PointStorage::get(self).y\n    }\n    \n    fn name(&self) -> String {\n        "AlphaPoint".to_string()\n    }\n}\n')),(0,r.kt)("p",null,"or a generic implementation:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![feature(min_specialization)]\n\npub trait Point {\n    fn x(&self) -> u32;\n\n    fn y(&self) -> u32;\n\n    fn name(&self) -> String;\n}\n\nimpl<T: PointStorage> Point for T {\n    default fn x(&self) -> u32 {\n        PointStorage::get(self).x\n    }\n\n    default fn y(&self) -> u32 {\n        PointStorage::get(self).y\n    }\n\n    default fn name(&self) -> String {\n        "AlphaPoint".to_string()\n    }\n}\n')),(0,r.kt)("h3",{id:"inheritance-of-the-implementation"},'"Inheritance" of the implementation'),(0,r.kt)("p",null,'When someone wants to "inherit" implementation and fields, he can embed the data structure, implement the storage trait, and define an impl section of the main trait:'),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"struct PointContract {\n    point: PointData,\n}\n\nimpl PointStorage for PointContract {\n    fn get(&self) -> & PointData {\n        &self.point\n    }\n    fn get_mut(&mut self) -> &mut PointData {\n        &mut self.point\n    }\n}\n\nimpl Point for PointContract {}\n")),(0,r.kt)("h2",{id:"macros-from-openbrush"},"Macros from OpenBrush"),(0,r.kt)("p",null,"Some macros from OpenBrush allows to remove boilerplate code and simplify the development:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/blob/main/lang/macro/src/lib.rs"},(0,r.kt)("inlineCode",{parentName:"a"},"declare_storage_trait!")),"\nmacro define the storage like described in the ",(0,r.kt)("a",{parentName:"li",href:"/smart-contracts/example/data#storage-trait"},"Storage trait"))),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"use openbrush::declare_storage_trait;\ndeclare_storage_trait!(PointStorage, PointData);\n")),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/blob/main/lang/macro/src/lib.rs"},(0,r.kt)("inlineCode",{parentName:"a"},"impl_storage_trait!")),"\nmacro implements the storage trait for the contract and return the field from that contract of the data type")),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"use openbrush::impl_storage_trait;\nimpl_storage_trait!(PointStorage, PointContract, point, PointData);\n")),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},"Also, if you are familiar with ",(0,r.kt)("a",{parentName:"li",href:"https://doc.rust-lang.org/rust-by-example/trait/derive.html"},"derive")," macro:")),(0,r.kt)("p",null,"You can create a derive macro for your storage trait by yourself with\n",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/blob/main/lang/src/derive.rs"},(0,r.kt)("inlineCode",{parentName:"a"},"declare_derive_storage_trait!")),"\nfrom OpenBrush.\nTo define a derive macro you need a separate directory(let's call it ",(0,r.kt)("inlineCode",{parentName:"p"},"derive"),").\nThis directory contains the standard stuff of a Cargo folder - ",(0,r.kt)("inlineCode",{parentName:"p"},".gitignore"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"Cargo.toml"),", and the ",(0,r.kt)("inlineCode",{parentName:"p"},"lib.rs")," file,\ninside of which we will define a derive. So in the end, our ",(0,r.kt)("inlineCode",{parentName:"p"},"lib.rs")," file will\nlook like this:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n\nextern crate proc_macro;\n\nuse openbrush::declare_derive_storage_trait;\n\ndeclare_derive_storage_trait!(derive_point_storage, PointStorage, PointStorageField);\n')),(0,r.kt)("p",null,"In the ",(0,r.kt)("inlineCode",{parentName:"p"},"Cargo.toml")," of the derive folder you need to import ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush")," dependencies:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-toml"},'[dependencies]\nsyn = { version = "1.0" }\nquote = "1.0"\nproc-macro2 = "1"\nopenbrush = { version = "~2.0.0", default-features = false }\n\n[lib]\nname = "point_derive"\npath = "lib.rs"\nproc-macro = true\n\n[features]\ndefault = ["std"]\nstd = []\n')),(0,r.kt)("p",null,"After importing that derive crate into your main contract,\nyou can use ",(0,r.kt)("inlineCode",{parentName:"p"},"derive(PointStorage)")," instead of ",(0,r.kt)("inlineCode",{parentName:"p"},"impl_storage_trait!"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"use point_derive::PointStorage;\n\n#[derive(PointStorage)]\nstruct PointContract {\n    #[PointStorageField]\n    point: PointData,\n}\n\nimpl Point for PointContract {}\n")))}u.isMDXComponent=!0}}]);