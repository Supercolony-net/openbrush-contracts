"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[728],{3905:function(e,t,n){n.d(t,{Zo:function(){return p},kt:function(){return h}});var r=n(7294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var c=r.createContext({}),l=function(e){var t=r.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},p=function(e){var t=l(e.components);return r.createElement(c.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,c=e.parentName,p=s(e,["components","mdxType","originalType","parentName"]),m=l(n),h=o,f=m["".concat(c,".").concat(h)]||m[h]||u[h]||a;return n?r.createElement(f,i(i({ref:t},p),{},{components:n})):r.createElement(f,i({ref:t},p))}));function h(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,i=new Array(a);i[0]=m;var s={};for(var c in t)hasOwnProperty.call(t,c)&&(s[c]=t[c]);s.originalType=e,s.mdxType="string"==typeof e?e:o,i[1]=s;for(var l=2;l<a;l++)i[l]=n[l];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},8148:function(e,t,n){n.r(t),n.d(t,{frontMatter:function(){return s},contentTitle:function(){return c},metadata:function(){return l},toc:function(){return p},default:function(){return m}});var r=n(7462),o=n(3366),a=(n(7294),n(3905)),i=["components"],s={sidebar_position:5,title:"PSP22 FlashMint"},c=void 0,l={unversionedId:"smart-contracts/PSP22/Extensions/flashmint",id:"smart-contracts/PSP22/Extensions/flashmint",isDocsHomePage:!1,title:"PSP22 FlashMint",description:"This example shows how you can reuse the implementation of PSP22 token with PSP22FlashMint extension, which allows the user to perform a flash loan on the token by minting the borrowed amount and then burning it along with fees for the loan.",source:"@site/docs/smart-contracts/PSP22/Extensions/flashmint.md",sourceDirName:"smart-contracts/PSP22/Extensions",slug:"/smart-contracts/PSP22/Extensions/flashmint",permalink:"/smart-contracts/PSP22/Extensions/flashmint",editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/docs/smart-contracts/PSP22/Extensions/flashmint.md",tags:[],version:"current",sidebarPosition:5,frontMatter:{sidebar_position:5,title:"PSP22 FlashMint"},sidebar:"tutorialSidebar",previous:{title:"PSP22 Wrapper",permalink:"/smart-contracts/PSP22/Extensions/wrapper"},next:{title:"PSP22 Pausable",permalink:"/smart-contracts/PSP22/Extensions/pausable"}},p=[{value:"1. Implement the FlashMint extension",id:"1-implement-the-flashmint-extension",children:[]}],u={toc:p};function m(e){var t=e.components,n=(0,o.Z)(e,i);return(0,a.kt)("wrapper",(0,r.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("p",null,"This example shows how you can reuse the implementation of ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22"},"PSP22")," token with ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22/src/extensions/flashmint.rs"},"PSP22FlashMint")," extension, which allows the user to perform a flash loan on the token by minting the borrowed amount and then burning it along with fees for the loan."),(0,a.kt)("h2",{id:"1-implement-the-flashmint-extension"},"1. Implement the FlashMint extension"),(0,a.kt)("p",null,"For your smart contract to use this extension, you need to implement the ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22FlashMint")," trait in your ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22")," smart contract. Import everything from ",(0,a.kt)("inlineCode",{parentName:"p"},"brush::contracts::psp22::extensions::flashmint::*")," and inherit the implementation for ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22FlashMint")," trait. You can also customize (override) the original functions from ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22FlashMint"),"."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"use brush::contracts::psp22::extensions::flashmint::*;\n\nimpl FlashLender for MyPSP22FlashMint {}\n")),(0,a.kt)("p",null,"And that's it! Your ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22")," is now extended by the ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22FlashMint")," extension and ready to use its functions!\nYou can check the full example of the implementation of this extension ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/flashmint"},"here"),"."),(0,a.kt)("p",null,"You can also check the documentation for the basic implementation of ",(0,a.kt)("a",{parentName:"p",href:"/smart-contracts/PSP22/psp22"},"PSP22"),"."))}m.isMDXComponent=!0}}]);