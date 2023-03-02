"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[36585],{3905:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>b});var r=n(67294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function i(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var l=r.createContext({}),c=function(e){var t=r.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},p=function(e){var t=c(e.components);return r.createElement(l.Provider,{value:t},e.children)},u="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},P=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,l=e.parentName,p=i(e,["components","mdxType","originalType","parentName"]),u=c(n),P=o,b=u["".concat(l,".").concat(P)]||u[P]||m[P]||a;return n?r.createElement(b,s(s({ref:t},p),{},{components:n})):r.createElement(b,s({ref:t},p))}));function b(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,s=new Array(a);s[0]=P;var i={};for(var l in t)hasOwnProperty.call(t,l)&&(i[l]=t[l]);i.originalType=e,i[u]="string"==typeof e?e:o,s[1]=i;for(var c=2;c<a;c++)s[c]=n[c];return r.createElement.apply(null,s)}return r.createElement.apply(null,n)}P.displayName="MDXCreateElement"},71643:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>s,default:()=>m,frontMatter:()=>a,metadata:()=>i,toc:()=>c});var r=n(87462),o=(n(67294),n(3905));const a={sidebar_position:3,title:"PSP22 Burnable"},s=void 0,i={unversionedId:"smart-contracts/PSP22-Pallet/Extensions/burnable",id:"smart-contracts/PSP22-Pallet/Extensions/burnable",title:"PSP22 Burnable",description:"This example shows how you can reuse the implementation of",source:"@site/docs/smart-contracts/PSP22-Pallet/Extensions/burnable.md",sourceDirName:"smart-contracts/PSP22-Pallet/Extensions",slug:"/smart-contracts/PSP22-Pallet/Extensions/burnable",permalink:"/next/smart-contracts/PSP22-Pallet/Extensions/burnable",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/docs/smart-contracts/PSP22-Pallet/Extensions/burnable.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{sidebar_position:3,title:"PSP22 Burnable"},sidebar:"tutorialSidebar",previous:{title:"PSP22 Mintable",permalink:"/next/smart-contracts/PSP22-Pallet/Extensions/mintable"},next:{title:"PSP34",permalink:"/next/smart-contracts/PSP34/"}},l={},c=[{value:"How to use this extension",id:"how-to-use-this-extension",level:2}],p={toc:c},u="wrapper";function m(e){let{components:t,...n}=e;return(0,o.kt)(u,(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"This example shows how you can reuse the implementation of\n",(0,o.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet"},"PSP22")," token with ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet/extensions/burnable.rs"},"PSP22Burnable")," extension via ",(0,o.kt)("inlineCode",{parentName:"p"},"pallet-assets")," chain extension."),(0,o.kt)("h2",{id:"how-to-use-this-extension"},"How to use this extension"),(0,o.kt)("p",null,"First, you should implement basic version of ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/docs/smart-contracts/PSP22-Pallet"},"PSP22 Pallet"),"."),(0,o.kt)("p",null,"For your smart contract to use this extension, you only need to implement the\n",(0,o.kt)("inlineCode",{parentName:"p"},"PSP22Burnable")," trait in your ",(0,o.kt)("inlineCode",{parentName:"p"},"PSP22 Pallet")," smart contract. Add import for\n",(0,o.kt)("inlineCode",{parentName:"p"},"openbrush::contracts::psp22_pallet::extensions::burnable::*"),", inherit the\nimplementation for ",(0,o.kt)("inlineCode",{parentName:"p"},"PSP22Burnable")," trait, where you can also customize (override)\nthe original functions from ",(0,o.kt)("inlineCode",{parentName:"p"},"PSP22Burnable"),"."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"use openbrush::contracts::psp22_pallet::extensions::burnable::*;\n\nimpl PSP22Burnable for Contract {}\n")),(0,o.kt)("p",null,"And that's it! Your ",(0,o.kt)("inlineCode",{parentName:"p"},"PSP22 Pallet")," is now extended by the ",(0,o.kt)("inlineCode",{parentName:"p"},"PSP22Burnable")," extension and ready to use its functions!"))}m.isMDXComponent=!0}}]);