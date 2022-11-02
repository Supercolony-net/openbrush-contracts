---
sidebar_position: 4
title: Deployment
sidebar_label: Deployment
---

- Deployment of ink! based smart contracts

This document contains description of how to deploy and test smart contracts locally and in testnet.

### Ecosystem

Polkadot doesn't support smart contract execution, only parachains can provide this functionality. More information
about how it works you can find on [official wiki](https://wiki.polkadot.network/docs/en/build-smart-contracts).

The list of standalone blockchain/parachains that support ink! smart contracts:

* [Astar](https://astar.network/)

### Overview

- To deploy contract you should build your own contract or get some example from [Openbrush](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples). You can find instruction how to build ink! smart contract in [docs](https://ink.substrate.io/getting-started/building-your-contract)
- You have to choose substrate network to deploy your contract.
  There are several option you have:

  - Local substrate node with pallet contracts.
  - `Canvas` network
  - `Shibuya` - Astar testnet
  - `Shiden` - Astar canary network
  - `Astar` main network (will support pallet contracts in near futures)
  - Other networks which supports pallet contracts
- Be sure that you have installed `polkadot.js.org` [wallet](#install-polkadot-extention-for-your-browser-and-create-account) extenstion for your browser
- Here you can find how to [Build](https://ink.substrate.io/cargo-contract-cli/#usage) **ink!** smart contract
- Let's [deploy to local network](#deployment-on-local-network)
- You can manuly [call](#call-the-smart-contract) our deployed contract
- [Canvas](https://github.com/paritytech/cumulus#canvas-) - a Smart Contracts [Parachain](https://wiki.polkadot.network/docs/learn-parachains) which was deployed on [Polkadot](https://polkadot.network/ru/) test network - [Rococo](https://polkadot.network/tag/rococo/). You need to get free `ROC` token using [faucet](#rococo-faucet) to deploy contract to Canvas network. Finally deploy your ink! smart contract to [canvas](#deploy-to-=anvas)
- [Astar](#astar) - [WASM](https://webassembly.org/) + [EVM](https://ethereum.org/en/developers/docs/evm/) Hub on [Polkadot](https://polkadot.network/). More info about astar [here](https://docs.astar.network/)
- You can deploy **ink!** smart contract to [Shibuya](#deploy-to-shibuya) (astar test network). How to get free `SBY` using [faucet](https://docs.astar.network/integration/testnet-faucet)

### Build

- navigate to `./openbrush/examples/psp22`
- build ink! contract using:

```
cargo +nightly contract build
```

Once the compilation is completed, a target folder is created. In this folder, under the ink subdirectory, you will be able to see a `my_psp22.wasm` file and a `metadata.json` file. `my_psp22.wasm` is your contract that has been compiled to web assembly and the `metadata.json` is a JSON abstraction of your contract.

You will find 3 files in folder `./openbrush/examples/psp22/target/ink`

- `my_psp22.contract` (code + metadata)
- `my_psp22.wasm` (the contract’s code)
- `metadata.json` (the contract’s metadata)

### Install polkadot extention for your browser and create account

- Navigate to [Polkadot.js.org](https://polkadot.js.org/extension/) extention tab and install to your browser. You need wallet extention to sign and submit transaction for deployment contract and manual testing via UI
- Create or import polkadot account. You need account and some tokens on that account to deploy and test contracts on test network like `Canvas`, `Shibuya` or main network like `Shiden` and `Astar` in near futures. How to get free tokens for test net you will find [there](#rococo-Faucet)
- Please write down your wallet's mnemonic seed and keep it in a safe place. The mnemonic can be used to restore your wallet. Keep it carefully to not lose your assets.

![](assets/20220605_155001_create-wallet.gif)

### Deployment on local network

- Substrate framework pre requisites [guide](https://ink.substrate.io/getting-started/setup/#substrate-framework-pre-requisites)
- Run a Substrate Node [guide](https://ink.substrate.io/getting-started/running-substrate)
- Navigate to the [Polkadot.js.org](https://polkadot.js.org) in a web browser
- Verify that you are connected to the [Local Node](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer).

![](assets/20220604_183027_go-to-polkadot.gif)

- Upload and deploy contract

  Click `Developer` -> `Contracts` -> `Upload & deploy code`. Specify the user account to use for deployment. Any fees will be deducted from deployment account. Upload `*.contract` file. `*.contract` file contains the `ABI` for the `WASM` code. The `ABI` is required and stored for future operations such as sending messages. Type a descriptive name for the smart contract. Set value `1000` for `totalSupply` when initialize the contract using constructor. And finally click `Sign and Submit` transaction.![](assets/20220605_122254_upload-contract.gif)

The Polkadot UI displays information about the content of the smart contract.

Depending on the account you used, you might be prompted for the account password. If you used a predefined account, you won’t need to provide a password.

### Call the smart contract

Now that your contract has been deployed on the blockchain, you can interact with it. Our deployed smart contract has  functions — `totalSupply()` and `balanceOf()` — and you can use the Polkadot UI to try them out.

To test the `balanceOf()` function:

Select any account from the Account list.

This contract doesn’t place restrictions on who is allowed to send the `balanceOf()` request.

- Click `Read`. Verify that the value `1,000,000,000,000,000` is returned in the Call Results.

![](assets/20220605_124705_balance-of.gif)

### Rococo Faucet

**Canvas** - parachain on **Rococo** ‒ a testnet for **Polkadot and Kusama parachains**.
As a first step, you should create an account. [See here for a detailed guide.](https://wiki.polkadot.network/docs/learn-account-generation)

As a second step, you have to get `ROC` testnet tokens through the [Rococo Faucet](https://wiki.polkadot.network/docs/learn-DOT#getting-rococo-tokens). This is a chat room in which you need to write:

`!drip YOUR_SS_58_ADDRESS:1002`

send message to [#rococo-faucet:matrix.org](https://matrix.to/#/#rococo-faucet:matrix.org)

The number `1002` is the parachain id of **Canvas on Rococo**, by supplying it the faucet will teleport `ROC` tokens directly to your account on the parachain

### Deploy to Canvas

- Navigate to the [Polkadot.js.org](https://polkadot.js.org/appshttps://paritytech.github.io/contracts-u) in a web browser.
- Verify that you are connected to the **Contracts Node**.

![](assets/20220605_125943_contracts-node.gif)

- Upload `my_psp22.contract` file the same way as to local node but we need some `ROC` tokens
- Use wallet which contains `ROC` tokens

### Astar

* **Astar** - Astar is a multi-chain smart contract platform that supports multiple
  blockchains and virtual machines.
* **Astar/Shiden Network Family:**
  Before starting the deployment, it's important to understand Astar/Shiden Network family. You should change the network based on what you want to do. Currently, there are 3 networks available, **Shiden**, **Shibuya**, and **Local** network. All networks support own standard Substrate RPC and EVM RPC.
* **Astar and Shiden**:
  Astar is the network that aims to be the parachain of Polkadot. Shiden is the sister network of Astar which is the parachain of Kusama. Basically, Astar and Shiden share the same code base. The biggest difference is the economic impact.

Please note that Shiden has its real economic value. So you need to pay in SDN, the native token of Shiden, when you execute transactions. You can buy SDN on crypto exchanges.

* **Shibuya**:
  Shibuya is the test network of Shiden and is connected to our own Relaychain. So Shibuya behaves almost the same as Shiden. Any new features are tested on Shibuya first and then deployed on Shiden. SBY, the native token of Shibuya, has no economic value and is available through our [faucet](https://docs.astar.network/integration/testnet-faucet). The best practice is to testing smart contract on Shibuya before deploying it on Shiden to check whether your smart contract works well or not.
* **Astar local Network**:
  Here is [tutorial](https://docs.astar.network/tutorial/develop-and-deploy-your-first-smart-contract-on-aster-shiden-evm/running-local-network) how to run local network

### Deploy to Shibuya

- Build smart contract the same way as for [local node](#build)
- Be sure that you have polkadot [wallet](https://docs.astar.network/stake2earn-festival/how-to-make-a-kusama-polkadot-address#recommend-polkadot-.js-browser-plugin) exension in your browser
- [Create polkadot account](https://docs.astar.network/tutorial/how-to/how-to-make-a-kusama-polkadot-address#create-account) if not have yet
- Use **Faucet** to get free **SBY** [token](https://docs.astar.network/integration/testnet-faucet)
- Go to [polkadot.js.org](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.shibuya.astar.network#/explorer)
- Switch network to **Shibuya** and deploy contract

![](assets/20220605_132655_shibuya_testnet.gif)

We use **“messages”** to communicate with smart contracts.

There are 2 types of messages:

- messages that change a smart contract’s state should be sent as transactions
- messages that don’t change a state can be made by using RPC calls

Next, let’s change the smart contract state by sending a transaction that calls the `transfer()` function.

![](assets/20220605_132803_transfer-shibuya.gif)

As expected, the value that was stored in the smart contract changed from `0` to `1` after the `transfer()` transaction is successfully executed

![](assets/20220605_133034_check-balance-of-shibuya.gif)

Congratulations, you deployed and test your first L1 Smart Contract to **Shibuya** network!
