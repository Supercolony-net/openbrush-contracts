"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[8108],{3905:(e,t,n)=>{n.d(t,{Zo:()=>l,kt:()=>f});var r=n(67294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function c(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var i=r.createContext({}),p=function(e){var t=r.useContext(i),n=t;return e&&(n="function"==typeof e?e(t):c(c({},t),e)),n},l=function(e){var t=p(e.components);return r.createElement(i.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,i=e.parentName,l=s(e,["components","mdxType","originalType","parentName"]),m=p(n),f=a,d=m["".concat(i,".").concat(f)]||m[f]||u[f]||o;return n?r.createElement(d,c(c({ref:t},l),{},{components:n})):r.createElement(d,c({ref:t},l))}));function f(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,c=new Array(o);c[0]=m;var s={};for(var i in t)hasOwnProperty.call(t,i)&&(s[i]=t[i]);s.originalType=e,s.mdxType="string"==typeof e?e:a,c[1]=s;for(var p=2;p<o;p++)c[p]=n[p];return r.createElement.apply(null,c)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},1446:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>i,contentTitle:()=>c,default:()=>u,frontMatter:()=>o,metadata:()=>s,toc:()=>p});var r=n(87462),a=(n(67294),n(3905));const o={sidebar_position:7,title:"PSP22 Capped"},c=void 0,s={unversionedId:"smart-contracts/PSP22/Extensions/capped",id:"version-1.4.0/smart-contracts/PSP22/Extensions/capped",title:"PSP22 Capped",description:"This example shows how you can implement a PSP22 contract with a supply cap, analogue to ERC20Capped.",source:"@site/versioned_docs/version-1.4.0/smart-contracts/PSP22/Extensions/capped.md",sourceDirName:"smart-contracts/PSP22/Extensions",slug:"/smart-contracts/PSP22/Extensions/capped",permalink:"/1.4.0/smart-contracts/PSP22/Extensions/capped",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/versioned_docs/version-1.4.0/smart-contracts/PSP22/Extensions/capped.md",tags:[],version:"1.4.0",sidebarPosition:7,frontMatter:{sidebar_position:7,title:"PSP22 Capped"},sidebar:"tutorialSidebar",previous:{title:"PSP22 Pausable",permalink:"/1.4.0/smart-contracts/PSP22/Extensions/pausable"},next:{title:"PSP22 Token Timelock",permalink:"/1.4.0/smart-contracts/PSP22/Utils/token-timelock"}},i={},p=[{value:"Step 1: Define storage",id:"step-1-define-storage",level:2},{value:"Step 2: Define constructor and contract functions",id:"step-2-define-constructor-and-contract-functions",level:2}],l={toc:p};function u(e){let{components:t,...n}=e;return(0,a.kt)("wrapper",(0,r.Z)({},l,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("p",null,"This example shows how you can implement a ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22"},"PSP22")," contract with a supply cap, analogue to ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/extensions/ERC20Capped.sol"},"ERC20Capped"),"."),(0,a.kt)("h2",{id:"step-1-define-storage"},"Step 1: Define storage"),(0,a.kt)("p",null,"Declare the storage struct and the field related to the ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22Storage")," trait, derive the ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22Storage")," trait and mark the corresponding field with the ",(0,a.kt)("inlineCode",{parentName:"p"},"#[PSP22StorageField]")," attribute. Also add the storage variable for cap."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, PSP22Storage)]\npub struct MyPSP22Capped {\n    #[PSP22StorageField]\n    psp22: PSP22Data,\n    cap: Balance,\n}\n")),(0,a.kt)("h2",{id:"step-2-define-constructor-and-contract-functions"},"Step 2: Define constructor and contract functions"),(0,a.kt)("p",null,"Define constructor, inherit ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22"),", and override the basic functions for capped implementation. Your ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP22Capped")," contract is ready!"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},'impl PSP22 for MyPSP22Capped {}\n\nimpl MyPSP22Capped {\n    #[ink(constructor)]\n    pub fn new(inital_supply: Balance, cap: Balance) -> Self {\n        let mut instance = Self::default();\n        assert!(instance.init_cap(cap).is_ok());\n        assert!(instance._mint(instance.env().caller(), inital_supply).is_ok());\n        instance\n    }\n\n    /// Expose the `_mint` function\n    #[ink(message)]\n    pub fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {\n        self._mint(account, amount)\n    }\n\n    #[ink(message)]\n    /// Method to return token\'s cap\n    pub fn cap(&self) -> Balance {\n        self.cap\n    }\n\n    /// Overrides the `_mint` function to check for cap overflow before minting tokens\n    /// Performs `PSP22::_mint` after the check succeeds\n    fn _mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {\n        if (self.total_supply() + amount) > self.cap() {\n            return Err(PSP22Error::Custom(String::from("Cap exceeded")))\n        }\n        PSP22::_mint(self, account, amount)\n    }\n\n    /// Initializes the token\'s cap\n    fn init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error> {\n        if cap <= 0 {\n            return Err(PSP22Error::Custom(String::from("Cap must be above 0")))\n        }\n        self.cap = cap;\n        Ok(())\n    }\n}\n')),(0,a.kt)("p",null,"You can check an implementation example of ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/capped"},"PSP22 Capped"),"."),(0,a.kt)("p",null,"You can also check the documentation for the basic implementation of ",(0,a.kt)("a",{parentName:"p",href:"/1.4.0/smart-contracts/PSP22/"},"PSP22"),"."))}u.isMDXComponent=!0}}]);