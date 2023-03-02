"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[34633],{3905:(e,n,t)=>{t.d(n,{Zo:()=>d,kt:()=>f});var a=t(67294);function r(e,n,t){return n in e?Object.defineProperty(e,n,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[n]=t,e}function o(e,n){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);n&&(a=a.filter((function(n){return Object.getOwnPropertyDescriptor(e,n).enumerable}))),t.push.apply(t,a)}return t}function i(e){for(var n=1;n<arguments.length;n++){var t=null!=arguments[n]?arguments[n]:{};n%2?o(Object(t),!0).forEach((function(n){r(e,n,t[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):o(Object(t)).forEach((function(n){Object.defineProperty(e,n,Object.getOwnPropertyDescriptor(t,n))}))}return e}function s(e,n){if(null==e)return{};var t,a,r=function(e,n){if(null==e)return{};var t,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)t=o[a],n.indexOf(t)>=0||(r[t]=e[t]);return r}(e,n);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)t=o[a],n.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(r[t]=e[t])}return r}var l=a.createContext({}),c=function(e){var n=a.useContext(l),t=n;return e&&(t="function"==typeof e?e(n):i(i({},n),e)),t},d=function(e){var n=c(e.components);return a.createElement(l.Provider,{value:n},e.children)},p="mdxType",u={inlineCode:"code",wrapper:function(e){var n=e.children;return a.createElement(a.Fragment,{},n)}},m=a.forwardRef((function(e,n){var t=e.components,r=e.mdxType,o=e.originalType,l=e.parentName,d=s(e,["components","mdxType","originalType","parentName"]),p=c(t),m=r,f=p["".concat(l,".").concat(m)]||p[m]||u[m]||o;return t?a.createElement(f,i(i({ref:n},d),{},{components:t})):a.createElement(f,i({ref:n},d))}));function f(e,n){var t=arguments,r=n&&n.mdxType;if("string"==typeof e||r){var o=t.length,i=new Array(o);i[0]=m;var s={};for(var l in n)hasOwnProperty.call(n,l)&&(s[l]=n[l]);s.originalType=e,s[p]="string"==typeof e?e:r,i[1]=s;for(var c=2;c<o;c++)i[c]=t[c];return a.createElement.apply(null,i)}return a.createElement.apply(null,t)}m.displayName="MDXCreateElement"},60113:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>l,contentTitle:()=>i,default:()=>u,frontMatter:()=>o,metadata:()=>s,toc:()=>c});var a=t(87462),r=(t(67294),t(3905));const o={sidebar_position:9,title:"Lending contract"},i=void 0,s={unversionedId:"smart-contracts/example/contract",id:"version-2.1.0/smart-contracts/example/contract",title:"Lending contract",description:"The main logic of the LendingContract is defined in the impls/lending directory.",source:"@site/versioned_docs/version-2.1.0/smart-contracts/example/contract.md",sourceDirName:"smart-contracts/example",slug:"/smart-contracts/example/contract",permalink:"/2.1.0/smart-contracts/example/contract",draft:!1,editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/versioned_docs/version-2.1.0/smart-contracts/example/contract.md",tags:[],version:"2.1.0",sidebarPosition:9,frontMatter:{sidebar_position:9,title:"Lending contract"},sidebar:"tutorialSidebar",previous:{title:"Errors",permalink:"/2.1.0/smart-contracts/example/errors"},next:{title:"Notes about methods",permalink:"/2.1.0/smart-contracts/example/implementation"}},l={},c=[{value:"Add dependencies",id:"add-dependencies",level:2},{value:"Define the contract storage",id:"define-the-contract-storage",level:2},{value:"Implement traits",id:"implement-traits",level:2},{value:"Define the constructor",id:"define-the-constructor",level:2}],d={toc:c},p="wrapper";function u(e){let{components:n,...t}=e;return(0,r.kt)(p,(0,a.Z)({},d,t,{components:n,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"The main logic of the ",(0,r.kt)("inlineCode",{parentName:"p"},"LendingContract")," is defined in the ",(0,r.kt)("inlineCode",{parentName:"p"},"impls/lending"),' directory.\nIn this file, we only need to "inherit" it.'),(0,r.kt)("h2",{id:"add-dependencies"},"Add dependencies"),(0,r.kt)("p",null,(0,r.kt)("inlineCode",{parentName:"p"},"LendingContract")," instantiates the ",(0,r.kt)("inlineCode",{parentName:"p"},"SharesContract")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"LoanContract"),", so we\nshould import them as ",(0,r.kt)("inlineCode",{parentName:"p"},"ink-as-dependency"),". Also we want to use the ",(0,r.kt)("inlineCode",{parentName:"p"},"AccessControl"),"\nand ",(0,r.kt)("inlineCode",{parentName:"p"},"Pausable"),' from OpenBrush, so we import them too. We also want to "inherit" the\nimplementation of ',(0,r.kt)("inlineCode",{parentName:"p"},"Lending")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"LendingPermissioned")," traits defined in the ",(0,r.kt)("inlineCode",{parentName:"p"},"lending_project")," crate."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-toml"},'[package]\nname = "lending_contract"\nversion = "2.1.0"\nauthors = ["Supercolony <dominik.krizo@supercolony.net>"]\nedition = "2021"\n\n[dependencies]\nink_primitives = { version = "~3.3.0", default-features = false }\nink_metadata = { version = "~3.3.0", default-features = false, features = ["derive"], optional = true }\nink_env = { version = "~3.3.0", default-features = false }\nink_storage = { version = "~3.3.0", default-features = false }\nink_lang = { version = "~3.3.0", default-features = false }\nink_prelude = { version = "~3.3.0", default-features = false }\nink_engine = { version = "~3.3.0", default-features = false, optional = true }\n\nscale = { package = "parity-scale-codec", version = "3", default-features = false, features = ["derive"] }\nscale-info = { version = "2", default-features = false, features = ["derive"], optional = true }\n\n# These dependencies\nshares_contract = { path = "../shares", default-features = false, features = ["ink-as-dependency"]  }\nloan_contract = { path = "../loan", default-features = false, features = ["ink-as-dependency"]  }\nlending_project = { path = "../..", default-features = false }\nopenbrush = { version = "~2.1.0", default-features = false, features = ["psp22", "psp34", "pausable", "access_control"] }\n\n[lib]\nname = "lending_contract"\npath = "lib.rs"\ncrate-type = [\n    "cdylib",\n]\n\n[features]\ndefault = ["std"]\nstd = [\n    "ink_primitives/std",\n    "ink_metadata",\n    "ink_metadata/std",\n    "ink_env/std",\n    "ink_storage/std",\n    "ink_lang/std",\n    "scale/std",\n    "scale-info",\n    "scale-info/std",\n\n    # These dependencies\n    "loan_contract/std",\n    "shares_contract/std",\n    "openbrush/std",\n]\nink-as-dependency = []\n\n[profile.dev]\noverflow-checks = false\ncodegen-units = 16\n\n[profile.release]\noverflow-checks = false\n')),(0,r.kt)("h2",{id:"define-the-contract-storage"},"Define the contract storage"),(0,r.kt)("p",null,"As described earlier, we want our smart contract to be paused by the Manager account.\nTo do that, we need our contract to be ",(0,r.kt)("inlineCode",{parentName:"p"},"Pausable")," and we need a manager role.\nWe can do this with the ",(0,r.kt)("inlineCode",{parentName:"p"},"AccessControl"),". Also, we want to use the ",(0,r.kt)("inlineCode",{parentName:"p"},"LendingStorage")," we have declared.\nSo we will declare a struct and derive all the needed traits."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, SpreadAllocate, AccessControlStorage, PausableStorage, LendingStorage)]\npub struct LendingContract {\n    #[AccessControlStorageField]\n    access: AccessControlData,\n    #[PausableStorageField]\n    pause: PausableData,\n    #[LendingStorageField]\n    lending: LendingData,\n}\n")),(0,r.kt)("h2",{id:"implement-traits"},"Implement traits"),(0,r.kt)("p",null,'We need to "inherit" the implementation of ',(0,r.kt)("inlineCode",{parentName:"p"},"AccessControll"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"Pausable"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"Lending"),",\n",(0,r.kt)("inlineCode",{parentName:"p"},"LendingPermissioned")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"LendingPermissionedInternal"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'impl AccessControl for LendingContract {}\n\nimpl Pausable for LendingContract {}\n\nimpl Lending for LendingContract {}\n\nimpl LendingPermissioned for LendingContract {}\n\nimpl LendingPermissionedInternal for LendingContract {\n    fn _instantiate_shares_contract(&self, contract_name: &str, contract_symbol: &str) -> AccountId {\n        let code_hash = self.lending.shares_contract_code_hash;\n        let (hash, _) =\n            ink_env::random::<ink_env::DefaultEnvironment>(contract_name.as_bytes()).expect("Failed to get salt");\n        let hash = hash.as_ref();\n        let contract = SharesContractRef::new(Some(String::from(contract_name)), Some(String::from(contract_symbol)))\n                .endowment(0)\n                .code_hash(code_hash)\n                .salt_bytes(&hash[..4])\n                .instantiate()\n                .unwrap();\n        contract.to_account_id()\n    }\n}\n')),(0,r.kt)("p",null,"Now the ",(0,r.kt)("inlineCode",{parentName:"p"},"LendingContract")," has functionality of all that traits."),(0,r.kt)("h2",{id:"define-the-constructor"},"Define the constructor"),(0,r.kt)("p",null,"Finally, we will add a constructor, in which we will initiate the admin of\nthe contract, to whom we will also grant the manager role declared before,\nand we will also instantiate the ",(0,r.kt)("inlineCode",{parentName:"p"},"LoanContract")," here and store its AccountId\nin ",(0,r.kt)("inlineCode",{parentName:"p"},"LendingContract"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'impl LendingContract {\n    /// constructor with name and symbol\n    #[ink(constructor, payable)]\n    pub fn new(shares_hash: Hash, loan_hash: Hash) -> Self {\n        ink_lang::codegen::initialize_contract(|instance: &mut LendingContract| {\n            let caller = instance.env().caller();\n            instance._init_with_admin(caller);\n            instance.grant_role(MANAGER, caller).expect("Can not set manager role");\n            instance.lending.shares_contract_code_hash = shares_hash;\n            // instantiate NFT contract and store its account id\n            let nft = LoanContractRef::new()\n                .endowment(0)\n                .code_hash(loan_hash)\n                .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])\n                .instantiate()\n                .unwrap();\n            instance.lending.loan_account = nft.to_account_id();\n        })\n    }\n}\n')))}u.isMDXComponent=!0}}]);