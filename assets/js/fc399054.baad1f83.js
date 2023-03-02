"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[95711],{3905:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>y});var r=n(67294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var l=r.createContext({}),c=function(e){var t=r.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},p=function(e){var t=c(e.components);return r.createElement(l.Provider,{value:t},e.children)},m="mdxType",u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,l=e.parentName,p=s(e,["components","mdxType","originalType","parentName"]),m=c(n),d=o,y=m["".concat(l,".").concat(d)]||m[d]||u[d]||a;return n?r.createElement(y,i(i({ref:t},p),{},{components:n})):r.createElement(y,i({ref:t},p))}));function y(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,i=new Array(a);i[0]=d;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s[m]="string"==typeof e?e:o,i[1]=s;for(var c=2;c<a;c++)i[c]=n[c];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},54458:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>i,default:()=>u,frontMatter:()=>a,metadata:()=>s,toc:()=>c});var r=n(87462),o=(n(67294),n(3905));const a={sidebar_position:4,title:"Deploy"},i=void 0,s={unversionedId:"deployment",id:"version-1.5.0/deployment",title:"Deploy",description:"Deployment of ink! based smart contracts",source:"@site/versioned_docs/version-1.5.0/deployment.md",sourceDirName:".",slug:"/deployment",permalink:"/1.5.0/deployment",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/versioned_docs/version-1.5.0/deployment.md",tags:[],version:"1.5.0",sidebarPosition:4,frontMatter:{sidebar_position:4,title:"Deploy"},sidebar:"tutorialSidebar",previous:{title:"Notes about methods",permalink:"/1.5.0/smart-contracts/example/implementation"},next:{title:"EVM vs WASM Smart Contracts",permalink:"/1.5.0/evm-wasm-smart-contracts"}},l={},c=[{value:"Deployment of ink! based smart contracts",id:"deployment-of-ink-based-smart-contracts",level:2},{value:"Ecosystem",id:"ecosystem",level:3},{value:"Deployment",id:"deployment",level:3}],p={toc:c},m="wrapper";function u(e){let{components:t,...n}=e;return(0,o.kt)(m,(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h2",{id:"deployment-of-ink-based-smart-contracts"},"Deployment of ink! based smart contracts"),(0,o.kt)("p",null,"This document contains description of how to deploy and test smart contracts locally and in testnet."),(0,o.kt)("h3",{id:"ecosystem"},"Ecosystem"),(0,o.kt)("p",null,"Polkadot doesn't support smart contract execution, only parachains can provide this functionality. More information\nabout how it works you can find on ",(0,o.kt)("a",{parentName:"p",href:"https://wiki.polkadot.network/docs/en/build-smart-contracts"},"official wiki"),"."),(0,o.kt)("p",null,"The list of standalone blockchain/parachains that support ink! smart contracts:"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("a",{parentName:"li",href:"https://edgewa.re"},"Edgeware")),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("a",{parentName:"li",href:"https://astar.network/"},"Astar"))),(0,o.kt)("h3",{id:"deployment"},"Deployment"),(0,o.kt)("p",null,"TODO"))}u.isMDXComponent=!0}}]);