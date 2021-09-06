"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[353],{3905:function(t,e,n){n.d(e,{Zo:function(){return u},kt:function(){return h}});var r=n(7294);function o(t,e,n){return e in t?Object.defineProperty(t,e,{value:n,enumerable:!0,configurable:!0,writable:!0}):t[e]=n,t}function a(t,e){var n=Object.keys(t);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(t);e&&(r=r.filter((function(e){return Object.getOwnPropertyDescriptor(t,e).enumerable}))),n.push.apply(n,r)}return n}function c(t){for(var e=1;e<arguments.length;e++){var n=null!=arguments[e]?arguments[e]:{};e%2?a(Object(n),!0).forEach((function(e){o(t,e,n[e])})):Object.getOwnPropertyDescriptors?Object.defineProperties(t,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(e){Object.defineProperty(t,e,Object.getOwnPropertyDescriptor(n,e))}))}return t}function s(t,e){if(null==t)return{};var n,r,o=function(t,e){if(null==t)return{};var n,r,o={},a=Object.keys(t);for(r=0;r<a.length;r++)n=a[r],e.indexOf(n)>=0||(o[n]=t[n]);return o}(t,e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(t);for(r=0;r<a.length;r++)n=a[r],e.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(t,n)&&(o[n]=t[n])}return o}var i=r.createContext({}),p=function(t){var e=r.useContext(i),n=e;return t&&(n="function"==typeof t?t(e):c(c({},e),t)),n},u=function(t){var e=p(t.components);return r.createElement(i.Provider,{value:e},t.children)},l={inlineCode:"code",wrapper:function(t){var e=t.children;return r.createElement(r.Fragment,{},e)}},m=r.forwardRef((function(t,e){var n=t.components,o=t.mdxType,a=t.originalType,i=t.parentName,u=s(t,["components","mdxType","originalType","parentName"]),m=p(n),h=o,f=m["".concat(i,".").concat(h)]||m[h]||l[h]||a;return n?r.createElement(f,c(c({ref:e},u),{},{components:n})):r.createElement(f,c({ref:e},u))}));function h(t,e){var n=arguments,o=e&&e.mdxType;if("string"==typeof t||o){var a=n.length,c=new Array(a);c[0]=m;var s={};for(var i in e)hasOwnProperty.call(e,i)&&(s[i]=e[i]);s.originalType=t,s.mdxType="string"==typeof t?t:o,c[1]=s;for(var p=2;p<a;p++)c[p]=n[p];return r.createElement.apply(null,c)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},6843:function(t,e,n){n.r(e),n.d(e,{frontMatter:function(){return s},contentTitle:function(){return i},metadata:function(){return p},toc:function(){return u},default:function(){return m}});var r=n(7462),o=n(3366),a=(n(7294),n(3905)),c=["components"],s={sidebar_position:1,title:"Overview"},i=void 0,p={unversionedId:"smart-contracts/overview",id:"smart-contracts/overview",isDocsHomePage:!1,title:"Overview",description:"This doc contains examples of how the library can be used and how to customize the base implementation.",source:"@site/docs/smart-contracts/overview.md",sourceDirName:"smart-contracts",slug:"/smart-contracts/overview",permalink:"/openbrush-contracts/smart-contracts/overview",editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/smart-contracts/overview.md",tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"Overview"},sidebar:"tutorialSidebar",previous:{title:"Getting Started",permalink:"/openbrush-contracts/"},next:{title:"PSP22",permalink:"/openbrush-contracts/smart-contracts/psp22"}},u=[],l={toc:u};function m(t){var e=t.components,n=(0,o.Z)(t,c);return(0,a.kt)("wrapper",(0,r.Z)({},l,n,{components:e,mdxType:"MDXLayout"}),(0,a.kt)("p",null,"This doc contains examples of how the library can be used and how to customize the base implementation."),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/openbrush-contracts/smart-contracts/psp22"},"PSP22")," shows an example of how you can reuse the implementation of\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22"},"psp22")," token (in the same way you can reuse\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721"},"psp721")," and ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155"},"psp1155"),")."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/openbrush-contracts/smart-contracts/access-control"},"Access Control")," shows how you can use the implementation of\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/access/access-control"},"access-control")," and\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721"},"psp721")," together to provide rights to mint and burn NFT tokens."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/openbrush-contracts/smart-contracts/ownable"},"Ownable")," shows how you can use the implementation of\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/access/ownable"},"ownable")," and\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155"},"psp1155")," together to provide rights to mint and burn tokens."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/openbrush-contracts/smart-contracts/reentrancy-guard"},"ReentrancyGuard")," shows how you can use the implementation of\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/security/reentrancy-guard"},"non_reentrant"),"\nmodifier to prevent reentrancy during certain functions."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/openbrush-contracts/smart-contracts/pausable"},"Pausable")," shows how you can use the implementation of\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/security/pausable"},"pausable"),"\ncontract and modifiers."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/openbrush-contracts/smart-contracts/timelock-controller"},"TimelockController")," shows how you can use the implementation of\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/governance/timelock-controller"},"timelock-controller"),"\nto execute a transaction with some delay via governance."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"/openbrush-contracts/smart-contracts/payment-splitter"},"PaymentSplitter")," shows how you can use the implementation of\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/finance/payment-splitter"},"payment-splitter"),"\nto split received native tokens between participants of the contract.")))}m.isMDXComponent=!0}}]);