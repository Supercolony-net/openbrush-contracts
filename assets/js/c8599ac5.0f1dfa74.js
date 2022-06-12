"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[997],{3905:function(e,t,n){n.d(t,{Zo:function(){return p},kt:function(){return b}});var r=n(7294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var c=r.createContext({}),u=function(e){var t=r.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},p=function(e){var t=u(e.components);return r.createElement(c.Provider,{value:t},e.children)},l={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,c=e.parentName,p=s(e,["components","mdxType","originalType","parentName"]),m=u(n),b=o,f=m["".concat(c,".").concat(b)]||m[b]||l[b]||a;return n?r.createElement(f,i(i({ref:t},p),{},{components:n})):r.createElement(f,i({ref:t},p))}));function b(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,i=new Array(a);i[0]=m;var s={};for(var c in t)hasOwnProperty.call(t,c)&&(s[c]=t[c]);s.originalType=e,s.mdxType="string"==typeof e?e:o,i[1]=s;for(var u=2;u<a;u++)i[u]=n[u];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},1975:function(e,t,n){n.r(t),n.d(t,{assets:function(){return p},contentTitle:function(){return c},default:function(){return b},frontMatter:function(){return s},metadata:function(){return u},toc:function(){return l}});var r=n(7462),o=n(3366),a=(n(7294),n(3905)),i=["components"],s={sidebar_position:3,title:"PSP35 Burnable"},c=void 0,u={unversionedId:"smart-contracts/PSP35/Extensions/burnable",id:"smart-contracts/PSP35/Extensions/burnable",title:"PSP35 Burnable",description:"This example shows how you can reuse the implementation of PSP35 token with PSP35Burnable extension.",source:"@site/docs/smart-contracts/PSP35/Extensions/burnable.md",sourceDirName:"smart-contracts/PSP35/Extensions",slug:"/smart-contracts/PSP35/Extensions/burnable",permalink:"/smart-contracts/PSP35/Extensions/burnable",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/docs/smart-contracts/PSP35/Extensions/burnable.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{sidebar_position:3,title:"PSP35 Burnable"},sidebar:"tutorialSidebar",previous:{title:"PSP35 Mintable",permalink:"/smart-contracts/PSP35/Extensions/mintable"},next:{title:"Overview",permalink:"/smart-contracts/example/overview"}},p={},l=[{value:"How to use this extension",id:"how-to-use-this-extension",level:2}],m={toc:l};function b(e){var t=e.components,n=(0,o.Z)(e,i);return(0,a.kt)("wrapper",(0,r.Z)({},m,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("p",null,"This example shows how you can reuse the implementation of ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35"},"PSP35")," token with ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35/src/extensions/burnable.rs"},"PSP35Burnable")," extension."),(0,a.kt)("h2",{id:"how-to-use-this-extension"},"How to use this extension"),(0,a.kt)("p",null,"For your smart contract to use this extension, you only need to implement the ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP35Burnable")," trait in your ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP35")," smart contract. Add import for ",(0,a.kt)("inlineCode",{parentName:"p"},"openbrush::contracts::psp35::extensions::burnable::*"),", inherit the implementation for ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP35Burnable")," trait, where you can also customize (override) the original functions from ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP35Burnable"),"."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"use openbrush::contracts::psp35::extensions::burnable::*;\n\nimpl PSP35Burnable for MyPSP35 {}\n")),(0,a.kt)("p",null,"And that's it! Your ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP35")," is now extended by the ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP35Burnable")," extension and ready to use its functions!\nYou can check an example of the usage of ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp35_extensions/burnable"},"PSP35 Burnable"),"."),(0,a.kt)("p",null,"You can also check the documentation for the basic implementation of ",(0,a.kt)("a",{parentName:"p",href:"/smart-contracts/PSP35"},"PSP35"),"."))}b.isMDXComponent=!0}}]);