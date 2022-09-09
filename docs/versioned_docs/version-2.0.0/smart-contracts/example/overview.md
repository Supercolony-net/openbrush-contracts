---
sidebar_position: 1
title: Overview
---

This example will show you how you can reuse OpenBrush smart contracts and macros in your project to ease the development process. We will also pay attention to the project structure to keep the maintenance and future development of the project simple.

We will be implementing a simple lending protocol, in which users can lend [PSP-22](/smart-contracts/PSP22) tokens, borrow them against a collateral token, repay their loans with interest, and of course withdraw the deposited assets. We will create a [PSP-22](/smart-contracts/PSP22) implementation which will be used for a stable coin and a collateral token, another [PSP-22](/smart-contracts/PSP22) token which will represent the shares of assets in the contract, [PSP-34](/smart-contracts/PSP34) token which will represent the loans and the lending contract itself. The simple [PSP-22](/smart-contracts/PSP22) token implementation will be created just for this example and to test the contract's functions. The contract will have the following features:

## Lending of assets accepted by the smart contract

Users can lend [PSP-22](/smart-contracts/PSP22) tokens, which are accepted by the contract. The allowance of lending specific tokens is decided in the smart contract by the accounts which have the Manager role. Upon lending the user gets a [PSP-22](/smart-contracts/PSP22) token representing their share of the asset pool.

## Borrowing of assets by depositing accepted assets as collateral

Users can borrow [PSP-22](/smart-contracts/PSP22) tokens, which are available in the contract. To borrow an asset, the user has to deposit an accepted [PSP-22](/smart-contracts/PSP22) token as collateral. The allowance of specific tokens being used as collateral is decided in the smart contract by the accounts which have the Manager role. The value of the borrowed assets can be equal at most to 70% of the value of the deposited collateral. If the value of the deposited collateral drops to or below 75% of the original value, the loan can be liquidated. Upon borrowing the assets user gets a [PSP-34](/smart-contracts/PSP34) token representing info about their loan (how much assets were borrowed, when did they borrow, what asset was borrowed, what asset was used as collateral, amount of collateral asset deposited, the liquidation price of the loan and if it was liquidated or not). This NFT token can be then used to repay the loan and get the collateral back.

## Repaying the loan

Users can repay their loan by depositing the borrowed amount of the borrowed assets with the interest which is calculated by the contract. Our contract has an interest rate of 10% per year. Users can repay the whole loan or a portion of the loan. The user will use their NFT to repay the loan. If the loan was liquidated in the meantime, they do not get their collateral back and the NFT is burned.

## Withdraw deposited assets

Users will deposit their share tokens to the smart contract and get back the deposited assets along with the interest generated if any.

## Liquidate a loan

Users can liquidate a loan which's collateral value is below or equal to 75% of the original value of the collateral. After the loan is liquidated, the liquidator gets 1% of the liquidated assets. 

## Allow an asset for lending or being used as a collateral

Users with the Manager role can allow an asset to be available for lending and borrowing or for being used as collateral.

## Pause the contract

Users with the Manager role can pause the contract. When the contract is paused, users can not deposit new assets for lending or borrowing assets. Users can still repay their loans, liquidate loans or withdraw their deposits when paused.