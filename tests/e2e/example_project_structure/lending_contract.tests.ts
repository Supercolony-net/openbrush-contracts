import { expect, getSigners } from '../helpers'
import BN from 'bn.js'
import { Roles } from '../constants'
import { ApiPromise } from '@polkadot/api'

import ConstructorsCoin from '../../../typechain-generated/constructors/stable_coin_contract'
import ContractCoin from '../../../typechain-generated/contracts/stable_coin_contract'

import ConstructorsLoan from '../../../typechain-generated/constructors/loan_contract'
import ContractLoan from '../../../typechain-generated/contracts/loan_contract'

import ConstructorsLending from '../../../typechain-generated/constructors/lending_contract'
import ContractLending from '../../../typechain-generated/contracts/lending_contract'

import ConstructorsShares from '../../../typechain-generated/constructors/shares_contract'
import ContractShares from '../../../typechain-generated/contracts/shares_contract'
import { assert } from 'chai'
import { IdBuilder } from '../../../typechain-generated/types-arguments/loan_contract'

describe('LENDING_CONTRACT', () => {
  async function setupCoin() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsCoin(api, defaultSigner)
    const contractAddress = (await contractFactory.new('Stable Coin' as unknown as string[], 'SC' as unknown as string[])).address
    const contract = new ContractCoin(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      bob,
      contract,
      query: contract.query,
      tx: contract.tx,
      close: async () => {
        await api.disconnect()
      }
    }
  }

  async function setupLoan() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsLoan(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
    const contract = new ContractLoan(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      bob,
      contract,
      query: contract.query,
      tx: contract.tx,
      close: async () => {
        await api.disconnect()
      }
    }
  }

  async function setupShares() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsShares(api, defaultSigner)
    const contractAddress = (await contractFactory.new([], [])).address
    const contract = new ContractShares(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      bob,
      contract,
      query: contract.query,
      tx: contract.tx,
      close: async () => {
        await api.disconnect()
      }
    }
  }

  async function setupLending(sharesHash: string, loanHash: string) {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsLending(api, defaultSigner)
    const contractAddress = (await contractFactory.new(sharesHash, loanHash)).address
    const contract = new ContractLending(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      bob,
      contract,
      query: contract.query,
      tx: contract.tx,
      close: async () => {
        await api.disconnect()
      }
    }
  }

  async function setupGreenCoin() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsCoin(api, defaultSigner)
    const contractAddress = (await contractFactory.new('Green Coin' as unknown as string[], 'GC' as unknown as string[])).address
    const contract = new ContractCoin(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      bob,
      contract,
      query: contract.query,
      tx: contract.tx,
      close: async () => {
        await api.disconnect()
      }
    }
  }

  async function setup() {
    const stablecoin = await setupCoin()
    const loan = await setupLoan()
    const shares = await setupShares()
    const lending = await setupLending(shares.contract.abi.info.source.wasmHash.toString(), loan.contract.abi.info.source.wasmHash.toString())
    const greencoin = await setupGreenCoin()

    return {
      lending: lending.contract,
      stablecoin: stablecoin.contract,
      alice: stablecoin.defaultSigner,
      bob: stablecoin.bob,
      greencoin: greencoin.contract,
      close: async () => {
        await stablecoin.close()
        await loan.close()
        await shares.close()
        await lending.close()
        await greencoin.close()
      }
    }
  }

  async function borrow(
    lendingContract: ContractLending,
    borrowedToken: ContractCoin,
    collateralToken: ContractCoin,
    user,
    approveAmount,
    collateralAmount,
    price
  ) {
    // collateralToken approves amount of tokens for lending contact
    await collateralToken.tx.approve(lendingContract.address, approveAmount)

    // Allow collateral
    await lendingContract.withSigner(user).tx.allowCollateral(collateralToken.address)

    // user approves amount of tokens for lending contact
    await borrowedToken.withSigner(user).tx.approve(lendingContract.address, approveAmount)
    // Allow new asset
    await lendingContract.tx.allowAsset(borrowedToken.address)
    // Alice lends tokens into lending
    await lendingContract.withSigner(user).tx.lendAssets(borrowedToken.address, approveAmount)

    // user approves amount of tokens for lending contact
    await borrowedToken.withSigner(user).tx.approve(lendingContract.address, approveAmount)
    // Set the price of collateralToken for borrowedToken
    await lendingContract.withSigner(user).tx.setAssetPrice(collateralToken.address, borrowedToken.address, price)

    const alice_balance = (await collateralToken.query.balanceOf(user.address)).value.unwrapRecursively().rawNumber
    // user borrow borrowedToken

    await expect(lendingContract.withSigner(user).tx.borrowAssets(borrowedToken.address, collateralToken.address, collateralAmount)).to.eventually.be
      .fulfilled

    await expect((await collateralToken.query.balanceOf(user.address)).value.unwrapRecursively().toString()).to.be.eq(
      alice_balance.sub(new BN(collateralAmount)).toString()
    )
  }

  it('LENDING CONTRACT - accept lending', async () => {
    const { lending, stablecoin, close } = await setup()

    // Arrange - Stablecoin is not accepted for lending
    await expect(lending.query.isAcceptedLending(stablecoin.address)).to.have.output(false)
    // Act - Allow stablecoin for lending
    await lending.tx.allowAsset(stablecoin.address)
    // Assert - Stablecoin is accepted
    await expect(lending.query.isAcceptedLending(stablecoin.address)).to.have.output(true)

    await close()
  })

  it('LENDING CONTRACT - disallow lending', async () => {
    const { lending, stablecoin, bob: alice, close } = await setup()

    // Arrange - Stablecoin is accepted for lending
    await lending.tx.allowAsset(stablecoin.address)
    await expect(lending.query.isAcceptedLending(stablecoin.address)).to.have.output(true)

    // Act - Grant Alice the manager role

    await lending.tx.grantRole(Roles.Manager, alice.address)
    // Act - Disallow stablecoin for lending
    await lending.withSigner(alice).tx.disallowLending(stablecoin.address)

    // Assert - Stablecoin is not accepted for lending
    await expect(lending.query.isAcceptedLending(stablecoin.address)).to.have.output(false)

    await close()
  })

  it('LENDING CONTRACT - allow collateral', async () => {
    const { lending, stablecoin, bob, close } = await setup()

    // Arrange - Stablecoin is not accepted for collateral
    await expect(lending.query.isAcceptedCollateral(stablecoin.address)).to.have.output(false)

    // Act - Grant Alice the manager role
    await lending.tx.grantRole(Roles.Manager, bob.address)
    // Act - Allow collateral for stablecoin
    await lending.withSigner(bob).tx.allowCollateral(stablecoin.address)

    // Assert - Stablecoin is accepted for collateral
    await expect(lending.query.isAcceptedCollateral(stablecoin.address)).to.have.output(true)

    await close()
  })

  it('LENDING CONTRACT - disallow collateral', async () => {
    const { lending, stablecoin, bob: alice, close } = await setup()

    // Act - Grant Alice the manager role
    await lending.tx.grantRole(Roles.Manager, alice.address)
    // Act - Allow collateral for stablecoin
    await lending.withSigner(alice).tx.allowCollateral(stablecoin.address)
    await expect(lending.query.isAcceptedCollateral(stablecoin.address)).to.have.output(true)
    // Act - Disallow collateral for stablecoin
    await lending.withSigner(alice).tx.disallowCollateral(stablecoin.address)

    // Assert - Stablecoin is not accepted for collateral
    await expect(lending.query.isAcceptedCollateral(stablecoin.address)).to.have.output(false)

    await close()
  })

  it('LENDING CONTRACT - lend asset', async () => {
    const { lending, stablecoin, alice, close } = await setup()

    const amount = 100

    // Arrange - Alice balance should be >= than lending amount
    const alice_balance = (await stablecoin.query.balanceOf(alice.address)).value.unwrapRecursively().rawNumber
    assert(alice_balance.gte(new BN(amount)))

    // Act - Stablecoin contract approves amount for lending contact
    await stablecoin.tx.approve(lending.address, amount)

    // Act - Allow stablecoin for lending
    await lending.tx.allowAsset(stablecoin.address)

    // Act - Alice lends the amount of stablecoin tokens
    await lending.withSigner(alice).tx.lendAssets(stablecoin.address, amount)

    // Assert - Lending contract has the amount of stablecoin tokens
    await expect(stablecoin.query.balanceOf(lending.address)).to.have.bnToNumber(amount)
    // Assert - A

    expect((await stablecoin.query.balanceOf(alice.address)).value.unwrapRecursively().toString()).to.be.eq(alice_balance.sub(new BN(amount)).toString())

    await close()
  })

  it('LENDING CONTRACT - borrow and repay full amount', async () => {
    const { lending, stablecoin, greencoin, alice, close } = await setup()

    const amount = 1000
    const collateralAmount = 100
    const price = 1

    await greencoin.tx.mint(alice.address, 1000000)

    // Act - Alice borrows greencoin
    const alice_balance = (await stablecoin.query.balanceOf(alice.address)).value.unwrapRecursively().rawNumber
    await borrow(lending, greencoin, stablecoin, alice, amount, collateralAmount, price)

    // Act - Alice repays loan
    await lending.withSigner(alice).tx.repay(IdBuilder.U128(1), collateralAmount)

    // Assert - Alice received borrowed tokens

    await expect((await stablecoin.query.balanceOf(alice.address)).value.unwrapRecursively().toString()).to.be.eq(alice_balance.toString())

    await close()
  })

  it('LENDING CONTRACT - borrow and repay part of amount', async () => {
    const { lending, stablecoin, greencoin, alice, close } = await setup()

    const amount = 1000
    const collateralAmount = 100
    const price = 2

    await greencoin.tx.mint(alice.address, new BN('1000000'))

    // Act - Alice borrows greencoin
    const alice_balance = (await stablecoin.query.balanceOf(alice.address)).value.unwrapRecursively().rawNumber
    await borrow(lending, greencoin, stablecoin, alice, amount, collateralAmount, price)

    // Act - Calculate half of the amount Alice should repay (borrowed amount = 70% of collateral amount)
    const loanAmount = (collateralAmount * 70) / 100
    const halfOfLoan = (loanAmount * price) / 2
    // Act - Alice repays half of loan
    await lending.withSigner(alice).tx.repay(IdBuilder.U128(1), halfOfLoan)

    // Assert - Alice received half of collateral tokens
    expect((await stablecoin.query.balanceOf(alice.address)).value.unwrapRecursively().toString()).to.be.eq(
      alice_balance.sub(new BN(collateralAmount / 2 + 1)).toString()
    )

    await close()
  })

  it('LENDING CONTRACT - withdraw asset', async () => {
    const { lending, stablecoin, alice, close } = await setup()

    const amount = 100

    // Act - Stablecoin contract approves amount for lending contact
    await stablecoin.tx.approve(lending.address, amount)
    // Act - Allow stablecoin for lending
    await lending.tx.allowAsset(stablecoin.address)
    // Act - Alice lends the amount of stablecoin tokens
    await lending.withSigner(alice).tx.lendAssets(stablecoin.address, amount)
    // Act - Alice withdraws stablecoin token
    const alice_balance = (await stablecoin.query.balanceOf(alice.address)).value.unwrapRecursively().rawNumber
    const sharesAddress = (await lending.query.getAssetShares(stablecoin.address)).value.unwrapRecursively().ok!
    const withdrawAmount = 1

    console.log(sharesAddress)
    console.log((await lending.withSigner(alice).query.withdrawAsset(sharesAddress, withdrawAmount)).value)

    await lending.withSigner(alice).tx.withdrawAsset(sharesAddress, withdrawAmount)

    // Assert - Balance of lending contract decreased at withdraw amount
    await expect(stablecoin.query.balanceOf(lending.address)).to.have.bnToNumber(amount - withdrawAmount)
    // Assert - Alice balance increased at withdraw amount

    expect((await stablecoin.query.balanceOf(alice.address)).value.unwrapRecursively().toString()).to.be.eq(alice_balance.add(new BN(withdrawAmount)).toString())

    await close()
  })

  it('LENDING CONTRACT - liquidate loan', async () => {
    const { lending, stablecoin, greencoin, alice, close } = await setup()

    const amount = 1000
    const collateralAmount = 100
    const price = 10

    await greencoin.tx.mint(alice.address, new BN('1000000'))

    // Act - Alice borrows greencoin
    await borrow(lending, greencoin, stablecoin, alice, amount, collateralAmount, price)
    await expect(lending.withSigner(alice).tx.liquidateLoan(IdBuilder.U8(1))).to.eventually.be.rejected

    // Act - Decrease greencoin price, now greencoin price < liquidation price
    await lending.withSigner(alice).tx.setAssetPrice(stablecoin.address, greencoin.address, 1)

    // Assert - Alice can liquidate loan
    await lending.withSigner(alice).tx.liquidateLoan(IdBuilder.U128(1))

    await close()
  })
})
