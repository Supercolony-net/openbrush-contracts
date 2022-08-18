"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[1258],{3905:(e,t,a)=>{a.d(t,{Zo:()=>p,kt:()=>u});var r=a(67294);function o(e,t,a){return t in e?Object.defineProperty(e,t,{value:a,enumerable:!0,configurable:!0,writable:!0}):e[t]=a,e}function n(e,t){var a=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),a.push.apply(a,r)}return a}function s(e){for(var t=1;t<arguments.length;t++){var a=null!=arguments[t]?arguments[t]:{};t%2?n(Object(a),!0).forEach((function(t){o(e,t,a[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(a)):n(Object(a)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(a,t))}))}return e}function l(e,t){if(null==e)return{};var a,r,o=function(e,t){if(null==e)return{};var a,r,o={},n=Object.keys(e);for(r=0;r<n.length;r++)a=n[r],t.indexOf(a)>=0||(o[a]=e[a]);return o}(e,t);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);for(r=0;r<n.length;r++)a=n[r],t.indexOf(a)>=0||Object.prototype.propertyIsEnumerable.call(e,a)&&(o[a]=e[a])}return o}var i=r.createContext({}),c=function(e){var t=r.useContext(i),a=t;return e&&(a="function"==typeof e?e(t):s(s({},t),e)),a},p=function(e){var t=c(e.components);return r.createElement(i.Provider,{value:t},e.children)},h={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var a=e.components,o=e.mdxType,n=e.originalType,i=e.parentName,p=l(e,["components","mdxType","originalType","parentName"]),d=c(a),u=o,m=d["".concat(i,".").concat(u)]||d[u]||h[u]||n;return a?r.createElement(m,s(s({ref:t},p),{},{components:a})):r.createElement(m,s({ref:t},p))}));function u(e,t){var a=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var n=a.length,s=new Array(n);s[0]=d;var l={};for(var i in t)hasOwnProperty.call(t,i)&&(l[i]=t[i]);l.originalType=e,l.mdxType="string"==typeof e?e:o,s[1]=l;for(var c=2;c<n;c++)s[c]=a[c];return r.createElement.apply(null,s)}return r.createElement.apply(null,a)}d.displayName="MDXCreateElement"},83226:(e,t,a)=>{a.r(t),a.d(t,{assets:()=>i,contentTitle:()=>s,default:()=>h,frontMatter:()=>n,metadata:()=>l,toc:()=>c});var r=a(87462),o=(a(67294),a(3905));const n={sidebar_position:1,title:"Overview"},s=void 0,l={unversionedId:"smart-contracts/example/overview",id:"version-1.6.0/smart-contracts/example/overview",title:"Overview",description:"This example will show you how you can reuse OpenBrush smart contracts and macros in your project to ease the development process. We will also pay attention to the project structure to keep the maintenance and future development of the project simple.",source:"@site/versioned_docs/version-1.6.0/smart-contracts/example/overview.md",sourceDirName:"smart-contracts/example",slug:"/smart-contracts/example/overview",permalink:"/1.6.0/smart-contracts/example/overview",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/versioned_docs/version-1.6.0/smart-contracts/example/overview.md",tags:[],version:"1.6.0",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"Overview"},sidebar:"tutorialSidebar",previous:{title:"PSP1155 Burnable",permalink:"/1.6.0/smart-contracts/PSP1155/Extensions/burnable"},next:{title:"Setup the project",permalink:"/1.6.0/smart-contracts/example/setup_project"}},i={},c=[{value:"Lending of assets accepted by the smart contract",id:"lending-of-assets-accepted-by-the-smart-contract",level:2},{value:"Borrowing of assets by depositing accepted assets as collateral",id:"borrowing-of-assets-by-depositing-accepted-assets-as-collateral",level:2},{value:"Repaying the loan",id:"repaying-the-loan",level:2},{value:"Withdraw deposited assets",id:"withdraw-deposited-assets",level:2},{value:"Liquidate a loan",id:"liquidate-a-loan",level:2},{value:"Allow an asset for lending or being used as a collateral",id:"allow-an-asset-for-lending-or-being-used-as-a-collateral",level:2},{value:"Pause the contract",id:"pause-the-contract",level:2}],p={toc:c};function h(e){let{components:t,...a}=e;return(0,o.kt)("wrapper",(0,r.Z)({},p,a,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"This example will show you how you can reuse OpenBrush smart contracts and macros in your project to ease the development process. We will also pay attention to the project structure to keep the maintenance and future development of the project simple."),(0,o.kt)("p",null,"We will be implementing a simple lending protocol, in which users can lend ",(0,o.kt)("a",{parentName:"p",href:"/1.6.0/smart-contracts/PSP22/"},"PSP-22")," tokens, borrow them against a collateral token, repay their loans with interest, and of course withdraw the deposited assets. We will create a ",(0,o.kt)("a",{parentName:"p",href:"/1.6.0/smart-contracts/PSP22/"},"PSP-22")," implementation which will be used for a stable coin and a collateral token, another ",(0,o.kt)("a",{parentName:"p",href:"/1.6.0/smart-contracts/PSP22/"},"PSP-22")," token which will represent the shares of assets in the contract, ",(0,o.kt)("a",{parentName:"p",href:"/1.6.0/smart-contracts/PSP34/"},"PSP-34")," token which will represent the loans and the lending contract itself. The simple ",(0,o.kt)("a",{parentName:"p",href:"/1.6.0/smart-contracts/PSP22/"},"PSP-22")," token implementation will be created just for this example and to test the contract's functions. The contract will have the following features:"),(0,o.kt)("h2",{id:"lending-of-assets-accepted-by-the-smart-contract"},"Lending of assets accepted by the smart contract"),(0,o.kt)("p",null,"Users can lend ",(0,o.kt)("a",{parentName:"p",href:"/1.6.0/smart-contracts/PSP22/"},"PSP-22")," tokens, which are accepted by the contract. The allowance of lending specific tokens is decided in the smart contract by the accounts which have the Manager role. Upon lending the user gets a ",(0,o.kt)("a",{parentName:"p",href:"/1.6.0/smart-contracts/PSP22/"},"PSP-22")," token representing their share of the asset pool."),(0,o.kt)("h2",{id:"borrowing-of-assets-by-depositing-accepted-assets-as-collateral"},"Borrowing of assets by depositing accepted assets as collateral"),(0,o.kt)("p",null,"Users can borrow ",(0,o.kt)("a",{parentName:"p",href:"/1.6.0/smart-contracts/PSP22/"},"PSP-22")," tokens, which are available in the contract. To borrow an asset, the user has to deposit an accepted ",(0,o.kt)("a",{parentName:"p",href:"/1.6.0/smart-contracts/PSP22/"},"PSP-22")," token as collateral. The allowance of specific tokens being used as collateral is decided in the smart contract by the accounts which have the Manager role. The value of the borrowed assets can be equal at most to 70% of the value of the deposited collateral. If the value of the deposited collateral drops to or below 75% of the original value, the loan can be liquidated. Upon borrowing the assets user gets a ",(0,o.kt)("a",{parentName:"p",href:"/1.6.0/smart-contracts/PSP34/"},"PSP-34")," token representing info about their loan (how much assets were borrowed, when did they borrow, what asset was borrowed, what asset was used as collateral, amount of collateral asset deposited, the liquidation price of the loan and if it was liquidated or not). This NFT token can be then used to repay the loan and get the collateral back."),(0,o.kt)("h2",{id:"repaying-the-loan"},"Repaying the loan"),(0,o.kt)("p",null,"Users can repay their loan by depositing the borrowed amount of the borrowed assets with the interest which is calculated by the contract. Our contract has an interest rate of 10% per year. Users can repay the whole loan or a portion of the loan. The user will use their NFT to repay the loan. If the loan was liquidated in the meantime, they do not get their collateral back and the NFT is burned."),(0,o.kt)("h2",{id:"withdraw-deposited-assets"},"Withdraw deposited assets"),(0,o.kt)("p",null,"Users will deposit their share tokens to the smart contract and get back the deposited assets along with the interest generated if any."),(0,o.kt)("h2",{id:"liquidate-a-loan"},"Liquidate a loan"),(0,o.kt)("p",null,"Users can liquidate a loan which's collateral value is below or equal to 75% of the original value of the collateral. After the loan is liquidated, the liquidator gets 1% of the liquidated assets. "),(0,o.kt)("h2",{id:"allow-an-asset-for-lending-or-being-used-as-a-collateral"},"Allow an asset for lending or being used as a collateral"),(0,o.kt)("p",null,"Users with the Manager role can allow an asset to be available for lending and borrowing or for being used as collateral."),(0,o.kt)("h2",{id:"pause-the-contract"},"Pause the contract"),(0,o.kt)("p",null,"Users with the Manager role can pause the contract. When the contract is paused, users can not deposit new assets for lending or borrowing assets. Users can still repay their loans, liquidate loans or withdraw their deposits when paused."))}h.isMDXComponent=!0}}]);