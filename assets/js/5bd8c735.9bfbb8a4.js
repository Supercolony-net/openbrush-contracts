"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[1090],{3905:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>h});var r=n(67294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var i=r.createContext({}),l=function(e){var t=r.useContext(i),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},p=function(e){var t=l(e.components);return r.createElement(i.Provider,{value:t},e.children)},m={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},u=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,i=e.parentName,p=c(e,["components","mdxType","originalType","parentName"]),u=l(n),h=a,f=u["".concat(i,".").concat(h)]||u[h]||m[h]||o;return n?r.createElement(f,s(s({ref:t},p),{},{components:n})):r.createElement(f,s({ref:t},p))}));function h(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,s=new Array(o);s[0]=u;var c={};for(var i in t)hasOwnProperty.call(t,i)&&(c[i]=t[i]);c.originalType=e,c.mdxType="string"==typeof e?e:a,s[1]=c;for(var l=2;l<o;l++)s[l]=n[l];return r.createElement.apply(null,s)}return r.createElement.apply(null,n)}u.displayName="MDXCreateElement"},86688:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>i,contentTitle:()=>s,default:()=>m,frontMatter:()=>o,metadata:()=>c,toc:()=>l});var r=n(87462),a=(n(67294),n(3905));const o={sidebar_position:1,title:"Overview"},s=void 0,c={unversionedId:"smart-contracts/overview",id:"version-1.1.0/smart-contracts/overview",title:"Overview",description:"This doc contains examples of how the library can be used and how to customize the base implementation.",source:"@site/versioned_docs/version-1.1.0/smart-contracts/overview.md",sourceDirName:"smart-contracts",slug:"/smart-contracts/overview",permalink:"/1.1.0/smart-contracts/overview",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/versioned_docs/version-1.1.0/smart-contracts/overview.md",tags:[],version:"1.1.0",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"Overview"},sidebar:"tutorialSidebar",previous:{title:"Getting started",permalink:"/1.1.0/"},next:{title:"Access Control",permalink:"/1.1.0/smart-contracts/access-control"}},i={},l=[],p={toc:l};function m(e){let{components:t,...n}=e;return(0,a.kt)("wrapper",(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("p",null,"This doc contains examples of how the library can be used and how to customize the base implementation."),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/PSP22/"},"PSP22")," shows an example of how you can reuse the implementation of\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22"},"psp22")," token (in the same way you can reuse\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721"},"psp721")," and ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155"},"psp1155"),").",(0,a.kt)("ul",{parentName:"li"},(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/PSP22/Extensions/metadata"},"PSP22Metadata"),": metadata for PSP22."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/PSP22/Extensions/mintable"},"PSP22Mintable"),": creation of new tokens."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/PSP22/Extensions/burnable"},"PSP22Burnable"),": destruction of own tokens."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/PSP22/Extensions/wrapper"},"PSP22Wrapper"),": wrapper for PSP22 token (useful for governance tokens etc.)."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/PSP22/Extensions/flashmint"},"PSP22FlashMint"),": extension which allows performing flashloans of the token by minting and burning the token."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/PSP22/Extensions/pausable"},"PSP22Pausable"),": example of using pausable extension in the PSP22 contract."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/PSP22/Extensions/capped"},"PSP22Capped"),": extension which adds a cap for total supply of PSP22 tokens."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/PSP22/Utils/token-timelock"},"PSP22TokenTimelock"),": Utility which allows token holders to lock their tokens for a specified amount of time."))),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/access-control"},"Access Control")," shows how you can use the implementation of\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/access/access-control"},"access-control")," and\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721"},"psp721")," together to provide rights to mint and burn NFT tokens."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/ownable"},"Ownable")," shows how you can use the implementation of\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/access/ownable"},"ownable")," and\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155"},"psp1155")," together to provide rights to mint and burn tokens."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/reentrancy-guard"},"ReentrancyGuard")," shows how you can use the implementation of\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/security/reentrancy-guard"},"non_reentrant"),"\nmodifier to prevent reentrancy during certain functions."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/pausable"},"Pausable")," shows how you can use the implementation of\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/security/pausable"},"pausable"),"\ncontract and modifiers."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/timelock-controller"},"TimelockController")," shows how you can use the implementation of\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/governance/timelock-controller"},"timelock-controller"),"\nto execute a transaction with some delay via governance."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/1.1.0/smart-contracts/payment-splitter"},"PaymentSplitter")," shows how you can use the implementation of\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/finance/payment-splitter"},"payment-splitter"),"\nto split received native tokens between participants of the contract.")))}m.isMDXComponent=!0}}]);