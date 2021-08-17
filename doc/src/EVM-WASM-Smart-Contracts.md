## Comparison of EVM Smart Contracts & Substrate WASM Smart contracts

### EVM

##### Background
Ethereum was the first Turing complete blockchain and was mainly developed because of limitations of Bitcoin's script. In order to execute Smart Contracts a sandbox environment is needed. The sandbox environment where Smart contracts are executed in Ethereum is the  **Ethereum virtual machine** (*EVM*)

##### Bytecode
EVM uses a stack-based bytecode language called EVM bytecode. EVM bytecode is a series of OP_CODE (instructions) that are executed by the EVM. These `OP_CODE`s offers rather limited instructions compare to a full extend language (Java, WASM, ..)  

The primitive of bytecode is *256bit* integer which can be a big limitation as it is needed to use *256bit* integer to do calculations on even low numbers and also it will take *256bit* storage for any value stored on persistence.  

The main OP_CODEs are **SLOAD** to load data, **SSTORE** to write a *256bit* integer to storage and **CALL** to call another contract [full list of OP_CODES](https://github.com/crytic/evm-opcodes). It's via these instructions that EVM interact with the chain.

![image info](pictures/eth-1.png)

##### Gas model
The incentive model of interacting with EVM is gas. It acts like a fee that is calculated for each instruction you are executing. Every instruction has a certain pre-calculated fee amount, and your total gas is the sum of all the instruction you will execute.  

Please check the entire table of [fees](https://blockgeeks.com/wp-content/uploads/2018/03/image2-2.png)
A gas fee is only due when interacting with EVM. There is no charge for the space on storage you are using, and it is resulting by having a ton of deadcode (or non-used contracts) staying on ethereum chain forever.

##### Evm process

![image info](pictures/eth2.png)

### Substrate Contract-Pallet

##### Intro
`pallet-contract` is a module in Substrate (the framework to build blockchain on `Polkadot`). Its purpose is a sandbox environment providing WASM interpreter to execute smart contracts in WASM.

![image info](pictures/WASM1.png)

##### WASM interpreter

At the moment contract-pallet use [wasmi](https://github.com/paritytech/wasmi) as interpreter. Wasmi is a pure interpreter-type WASM virtual machine as execution of smart contract needs a high degree of correctness.

##### Storage rent & Gas

In order to incentives the deletion of unused code on chain, pallet-contract implemented a storage-rent principle.  

Data that persist on chain (the contract code + all of its storage) will be charged. So every smart contract will have a rent to pay (small amount at every block). When the smart contract will not have enough fund to pay the rent the smart contract will become a Tombstone (storage will be deleted).  

The gas system of ethereum (price depending on the complexity of the computation) is still present, but it is charged after the call is executed as it is basically a fee on the time of execution (the more time it takes for the node to execute your call the more you will pay). The `pallet-contract` will define the [amount of gas](https://substrate.dev/docs/en/knowledgebase/smart-contracts/contracts-pallet)

##### Contract code & instance are decoupled

Even though pallet-contract use an account model for their contracts alike ethereum does as well, there is still a big difference:  

When you deploy a WASM smart contract on chain it will only create a hash of the WASM code, that's mean this contract will not have an address, nor an associated storage (so no interaction possible). As opposed to ethereum where every contract code deployed on chain will have a unique address (instance), and an associated storage.  

When the WASM hash is on chain you can create as mush as instance of this contract code. Each instance will have a unique address to interact with as well as its own associated storage.  

It is practical for:
- Different contract instance with different **constructors** can be instantiated to reduce the space needed on chain to store WASM code
- Storage and balance are decoupled from contract code logic, it can enable to **patch or upgrade** the underlying contract code

### EVM vs contract-pallet

- Common point: they both are a sandbox to execute smart contracts
- The engine to execute contract is different. In ethereum, as it was the first blockchain to implement n sandbox environment, it is rather limited and slow compare to a wasm interpreter
- Storage-rent has been introduced in contract-pallet to incentives the deletion of unused code
- Contract pallet integrated a [two-step-deployment](https://substrate.dev/docs/en/knowledgebase/smart-contracts/contracts-pallet#two-step-deployment) to decouple contract code and contract instances


### Comparing WASM Smart contracts to EVM smart contracts

- WASM is broadly adopted as EVM usage is only for EVM -> There is way more tools available for WASM development
- EVM bytecode can only be compiled from Solidity or Vyper while WASM can be compiled from a lot of Popular languages (Rust, C/C++, C#, Java, Typescript, Haxe, Kotlin and even from Solidity)
- Excellent integration of Rust to compile in WASM
- Lightweight: it produces lightweight binaries that can ship easily
- performance near native code (2x faster than Javascipt)
- Continually developed by major companies such as Google, Apple, Microsoft, Mozilla, and Facebook.

[more info here](https://paritytech.github.io/ink-docs/why-webassembly-for-smart-contracts)