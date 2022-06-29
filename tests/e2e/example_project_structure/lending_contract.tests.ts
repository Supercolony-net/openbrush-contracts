import { expect, setupContract, fromSigner } from '../helpers'
import BN from 'bn.js'
import { Roles } from '../constants'

interface Result {
  ok
}

describe('LENDING_CONTRACT', () => {
  async function setup() {
    const stablecoin = await setupContract('stable_coin_contract', 'new', 'Stable Coin', 'SC')
    const loan = await (await setupContract('loan_contract', 'new')).abi
    const shares = await (await setupContract('shares_contract', 'new', '', '')).abi
    const lending = await setupContract('lending_contract', 'new', shares.source.hash, loan.source.hash)
    const greencoin = await setupContract('stable_coin_contract', 'new', 'Green Coin', 'GC')

    return { lending: lending.contract, stablecoin: stablecoin.contract, alice: stablecoin.defaultSigner.address, greencoin: greencoin.contract }
  }

  function result(s: string | undefined) {
    const result: Result = s != null ? JSON.parse(s) : null
    return result
  }

  async function borrow(lendingContract, borrowedToken, collateralToken, user, approveAmount, collateralAmount, price) {
    // collateralToken approves amount of tokens for lending contact
    await expect(collateralToken.tx.approve(lendingContract.address, approveAmount)).to.eventually.be.fulfilled
    // Grant user the manager role
    await expect(lendingContract.tx.grantRole(Roles.Manager, user)).to.eventually.be.fulfilled
    // Allow collateral
    await expect(fromSigner(lendingContract, user).tx.allowCollateral(collateralToken.address)).to.eventually.be.fulfilled

    // user approves amount of tokens for lending contact
    await expect(fromSigner(borrowedToken, user).tx.approve(lendingContract.address, approveAmount)).to.eventually.be.fulfilled
    // Allow new asset
    await expect(lendingContract.tx.allowAsset(borrowedToken.address)).to.eventually.be.fulfilled
    // Alice lends tokens into lending
    await expect(fromSigner(lendingContract, user).tx.lendAssets(borrowedToken.address, approveAmount)).to.eventually.be.fulfilled

    // user approves amount of tokens for lending contact
    await expect(fromSigner(borrowedToken, user).tx.approve(lendingContract.address, approveAmount)).to.eventually.be.fulfilled
    // Set the price of collateralToken for borrowedToken
    await expect(fromSigner(lendingContract, user).tx.setAssetPrice(collateralToken.address, borrowedToken.address, price)).to.eventually.be.fulfilled

    const alice_balance = (await collateralToken.query.balanceOf(user)).output
    // user borrow borrowedToken
    await expect(fromSigner(lendingContract, user).tx.borrowAssets(borrowedToken.address, collateralToken.address, collateralAmount)).to.eventually.be
      .fulfilled
    // @ts-ignore
    await expect(collateralToken.query.balanceOf(user)).to.have.output(alice_balance.sub(new BN(collateralAmount)))
  }

  it('LENDING CONTRACT - accept lending', async () => {
    const { lending, stablecoin } = await setup()

    // Arrange - Stablecoin is not accepted for lending
    await expect(lending.query.isAcceptedLending(stablecoin.address)).to.have.output(false)
    // Act - Allow stablecoin for lending
    await expect(lending.tx.allowAsset(stablecoin.address)).to.eventually.be.fulfilled
    // Assert - Stablecoin is accepted
    await expect(lending.query.isAcceptedLending(stablecoin.address)).to.have.output(true)
  })

  it('LENDING CONTRACT - disallow lending', async () => {
    const { lending, stablecoin, alice } = await setup()

    // Arrange - Stablecoin is accepted for lending
    await expect(lending.tx.allowAsset(stablecoin.address)).to.eventually.be.fulfilled
    await expect(lending.query.isAcceptedLending(stablecoin.address)).to.have.output(true)

    // Act - Grant Alice the manager role
    await expect(lending.tx.grantRole(Roles.Manager, alice)).to.eventually.be.fulfilled
    // Act - Disallow stablecoin for lending
    await expect(fromSigner(lending, alice).tx.disallowLending(stablecoin.address)).to.eventually.be.fulfilled

    // Assert - Stablecoin is not accepted for lending
    await expect(lending.query.isAcceptedLending(stablecoin.address)).to.have.output(false)
  })

  it('LENDING CONTRACT - allow collateral', async () => {
    const { lending, stablecoin, alice } = await setup()

    // Arrange - Stablecoin is not accepted for collateral
    await expect(lending.query.isAcceptedCollateral(stablecoin.address)).to.have.output(false)

    // Act - Grant Alice the manager role
    await expect(lending.tx.grantRole(Roles.Manager, alice)).to.eventually.be.fulfilled
    // Act - Allow collateral for stablecoin
    await expect(fromSigner(lending, alice).tx.allowCollateral(stablecoin.address)).to.eventually.be.fulfilled

    // Assert - Stablecoin is accepted for collateral
    await expect(lending.query.isAcceptedCollateral(stablecoin.address)).to.have.output(true)
  })

  it('LENDING CONTRACT - disallow collateral', async () => {
    const { lending, stablecoin, alice } = await setup()

    // Act - Grant Alice the manager role
    await expect(lending.tx.grantRole(Roles.Manager, alice)).to.eventually.be.fulfilled
    // Act - Allow collateral for stablecoin
    await expect(fromSigner(lending, alice).tx.allowCollateral(stablecoin.address)).to.eventually.be.fulfilled
    await expect(lending.query.isAcceptedCollateral(stablecoin.address)).to.have.output(true)
    // Act - Disallow collateral for stablecoin
    await expect(fromSigner(lending, alice).tx.disallowCollateral(stablecoin.address)).to.eventually.be.fulfilled

    // Assert - Stablecoin is not accepted for collateral
    await expect(lending.query.isAcceptedCollateral(stablecoin.address)).to.have.output(false)
  })

  it('LENDING CONTRACT - lend asset', async () => {
    const { lending, stablecoin, alice } = await setup()

    const amount = 100

    // Arrange - Alice balance should be >= than lending amount
    const alice_balance = (await stablecoin.query.balanceOf(alice)).output
    expect(alice_balance).to.gte(amount)

    // Act - Stablecoin contract approves amount for lending contact
    await expect(stablecoin.tx.approve(lending.address, amount)).to.eventually.be.fulfilled

    // Act - Allow stablecoin for lending
    await expect(lending.tx.allowAsset(stablecoin.address)).to.eventually.be.fulfilled

    // Act - Alice lends the amount of stablecoin tokens
    await expect(fromSigner(lending, alice).tx.lendAssets(stablecoin.address, amount)).to.eventually.be.fulfilled

    // Assert - Lending contract has the amount of stablecoin tokens
    await expect(stablecoin.query.balanceOf(lending.address)).to.have.output(amount)
    // Assert - Alice balance is changed
    // @ts-ignore
    await expect(stablecoin.query.balanceOf(alice)).to.have.output(alice_balance.sub(new BN(amount)))
  })

  it('LENDING CONTRACT - borrow and repay full amount', async () => {
    const { lending, stablecoin, greencoin, alice } = await setup()

    const amount = 1000
    const collateralAmount = 100
    const price = 1

    await greencoin.tx.mint(alice, new BN('1000000'))

    // Act - Alice borrows redcoin
    const alice_balance = (await stablecoin.query.balanceOf(alice)).output
    await borrow(lending, greencoin, stablecoin, alice, amount, collateralAmount, price)

    // Act - Alice repays loan
    await expect(fromSigner(lending, alice).tx.repay({ u8: 1 }, collateralAmount)).to.eventually.be.fulfilled

    // Assert - Alice received borrowed tokens
    // @ts-ignore
    await expect(stablecoin.query.balanceOf(alice)).to.have.output(alice_balance)
  })

  it('LENDING CONTRACT - borrow and repay part of amount', async () => {
    const { lending, stablecoin, greencoin, alice } = await setup()

    const amount = 1000
    const collateralAmount = 100
    const price = 2

    await greencoin.tx.mint(alice, new BN('1000000'))

    // Act - Alice borrows redcoin
    const alice_balance = (await stablecoin.query.balanceOf(alice)).output
    await borrow(lending, greencoin, stablecoin, alice, amount, collateralAmount, price)

    // Act - Calculate half of the amount Alice should repay (borrowed amount = 70% of collateral amount)
    const loanAmount = (collateralAmount * 70) / 100
    const halfOfLoan = (loanAmount * price) / 2
    // Act - Alice repays half of loan
    await expect(fromSigner(lending, alice).tx.repay({ u8: 1 }, halfOfLoan)).to.eventually.be.fulfilled

    // Assert - Alice received half of collateral tokens
    // @ts-ignore
    await expect(stablecoin.query.balanceOf(alice)).to.have.output(alice_balance.sub(new BN(collateralAmount / 2 + 1)))
  })

  it('LENDING CONTRACT - withdraw asset', async () => {
    const { lending, stablecoin, alice } = await setup()

    const amount = 100

    // Act - Stablecoin contract approves amount for lending contact
    await expect(stablecoin.tx.approve(lending.address, amount)).to.eventually.be.fulfilled
    // Act - Allow stablecoin for lending
    await expect(lending.tx.allowAsset(stablecoin.address)).to.eventually.be.fulfilled
    // Act - Alice lends the amount of stablecoin tokens
    await expect(fromSigner(lending, alice).tx.lendAssets(stablecoin.address, amount)).to.eventually.be.fulfilled
    // Act - Alice withdraws stablecoin token
    const alice_balance = (await stablecoin.query.balanceOf(alice)).output
    const sharesAddress = result((await lending.query.getSharesFromAsset(stablecoin.address)).output?.toString()).ok
    const withdrawAmount = 1
    await expect(fromSigner(lending, alice).tx.withdrawAsset(sharesAddress, withdrawAmount)).to.eventually.be.fulfilled

    // Assert - Balance of lending contract decreased at withdraw amount
    await expect(stablecoin.query.balanceOf(lending.address)).to.have.output(amount - withdrawAmount)
    // Assert - Alice balance increased at withdraw amount
    // @ts-ignore
    await expect(stablecoin.query.balanceOf(alice)).to.have.output(alice_balance.add(new BN(withdrawAmount)))
  })

  it('LENDING CONTRACT - liquidate loan', async () => {
    const { lending, stablecoin, greencoin, alice } = await setup()

    const amount = 1000
    const collateralAmount = 100
    const price = 10

    await greencoin.tx.mint(alice, new BN('1000000'))

    // Act - Alice borrows redcoin
    await borrow(lending, greencoin, stablecoin, alice, amount, collateralAmount, price)
    await expect(fromSigner(lending, alice).tx.liquidateLoan({ u8: 1 })).to.eventually.be.rejected

    // Act - Decrease redcoin price, now redcoin price < liquidation price
    await expect(fromSigner(lending, alice).tx.setAssetPrice(stablecoin.address, greencoin.address, 1)).to.eventually.be.fulfilled

    // Assert - Alice can liquidate loan
    await expect(fromSigner(lending, alice).tx.liquidateLoan({ u8: 1 })).to.eventually.be.fulfilled
  })
})
