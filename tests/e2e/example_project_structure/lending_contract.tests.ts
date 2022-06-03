import { expect, setupContract, fromSigner } from '../helpers'
import BN from 'bn.js'

describe('LENDING_CONTRACT', () => {
  async function setup() {
    let stable_coin = await setupContract('stable_coin_contract', 'new', `Green Coin`, `GC`)
    let collateral_stable_coin = await setupContract('collateral_stable_coin_contract', 'new', `Collateral stable coin`, `CSC`)
    let loan = await (await setupContract('loan_contract', 'new' )).abi
    let shares = await (await setupContract('shares_contract', 'new', '', '')).abi
    let lending = await setupContract('lending_contract', 'new', shares.source.hash, loan.source.hash)

    return { lending, stable_coin, alice: stable_coin.defaultSigner, collateral_stable_coin }
  }

  // is_accepted_lending

  it('LENDING CONTRACT - is accepted lending', async () => {
    const { lending, stable_coin, alice } = await setup()

    await expect(lending.query.isAcceptedLending(stable_coin.contract.address)).to.have.output(false)
    // Allow new asset(stable coin) in the lending protocol
    await expect(lending.tx.allowAsset(stable_coin.contract.address)).to.eventually.be.fulfilled
    // check is accepted lending
    await expect(lending.query.isAcceptedLending(stable_coin.contract.address)).to.have.output(true)
  })

  it('LENDING CONTRACT - lend asset', async () => {
    const { lending, stable_coin, alice } = await setup()

    const amount = 100;

    // Alice balance should be >= than lending `amount`
    let alice_balance = (await stable_coin.query.balanceOf(alice.address)).output;
    expect(alice_balance).to.gte(amount);

    // Alice approves `amount` for lending contact
    await expect(stable_coin.tx.approve(lending.contract.address, amount)).to.eventually.be.fulfilled

    // Allow new asset(stable coin) in the lending
    await expect(lending.tx.allowAsset(stable_coin.contract.address)).to.eventually.be.fulfilled

    // Alice lends `amount` tokens into lending
    await expect(stable_coin.query.balanceOf(lending.contract.address)).to.have.output(0)
    await expect(stable_coin.query.balanceOf(alice.address)).to.have.output(alice_balance)
    await expect(fromSigner(lending.contract, alice.address).tx.lendAssets(stable_coin.contract.address, amount)).to.eventually.be.fulfilled
    await expect(stable_coin.query.balanceOf(lending.contract.address)).to.have.output(amount)
    // @ts-ignore
    await expect(stable_coin.query.balanceOf(alice.address)).to.have.output((alice_balance.sub(new BN(amount))))
  })

  it('LENDING CONTRACT - disallow lending', async () => {
    const { lending, stable_coin, alice } = await setup()

    await expect(lending.tx.allowAsset(stable_coin.contract.address)).to.eventually.be.fulfilled
    //disallow lending
    await expect(fromSigner(lending.contract, alice.address).tx.disallowLending(stable_coin.contract.address)).to.eventually.be.fulfilled
  })

  it('LENDING CONTRACT - allow collateral', async () => {
    const { lending, stable_coin, alice } = await setup()

    await expect(lending.tx.allowAsset(stable_coin.contract.address)).to.eventually.be.fulfilled
    //allow collateral
    await expect(fromSigner(lending.contract, alice.address).tx.allowCollateral(stable_coin.contract.address)).to.eventually.be.fulfilled
  })

  it('LENDING CONTRACT - disallow collateral', async () => {
    //arrange
    const { lending, stable_coin, alice } = await setup()

    await expect(lending.tx.allowAsset(stable_coin.contract.address)).to.eventually.be.fulfilled
    await expect(fromSigner(lending.contract, alice.address).tx.allowCollateral(stable_coin.contract.address)).to.eventually.be.fulfilled
    //act
    const ret = fromSigner(lending.contract, alice.address).tx.disallowCollateral(stable_coin.contract.address)
    //assert
    await expect(ret).to.eventually.be.fulfilled
  })

  it('LENDING CONTRACT - withdraw asset', async () => {
    //arrange
    const { lending, stable_coin, alice } = await setup()

    let alice_balance = (await stable_coin.query.balanceOf(alice.address)).output;
    // Alice approves `amount` for lending contact
    await expect(stable_coin.tx.approve(lending.contract.address, 100)).to.eventually.be.fulfilled

    // Allow new asset(stable coin) in the lending
    await expect(lending.tx.allowAsset(stable_coin.contract.address)).to.eventually.be.fulfilled

    // Alice lends `amount` tokens into lending
    await expect(stable_coin.query.balanceOf(lending.contract.address)).to.have.output(0)
    await expect(stable_coin.query.balanceOf(alice.address)).to.have.output(alice_balance)
    await expect(fromSigner(lending.contract, alice.address).tx.lendAssets(stable_coin.contract.address, 100)).to.eventually.be.fulfilled
    await expect(stable_coin.query.balanceOf(lending.contract.address)).to.have.output(100)
    let sharesAddress = await lending.query.getSharesAddress(stable_coin.contract.address);
    
    //withdraw asset
    await expect(fromSigner(lending.contract, alice.address).tx.withdrawAsset(sharesAddress, 1)).to.eventually.be.fulfilled
    await expect(stable_coin.query.balanceOf(lending.contract.address)).to.have.output(0)
    await expect(stable_coin.query.balanceOf(alice.address)).to.have.output(0)
  })

  it('LENDING CONTRACT - borrow and repay assets', async () => {
    //arrange
    const { lending, stable_coin, alice, collateral_stable_coin } = await setup()

    let alice_balance = (await stable_coin.query.balanceOf(alice.address)).output;
    // Alice approves `amount` for lending contact
    await expect(stable_coin.tx.approve(lending.contract.address, 100)).to.eventually.be.fulfilled

    // Allow new asset(stable coin) in the lending
    await expect(lending.tx.allowAsset(stable_coin.contract.address)).to.eventually.be.fulfilled

    // Alice lends `amount` tokens into lending
    await expect(stable_coin.query.balanceOf(lending.contract.address)).to.have.output(0)
    await expect(stable_coin.query.balanceOf(alice.address)).to.have.output(alice_balance)
    await expect(fromSigner(lending.contract, alice.address).tx.lendAssets(stable_coin.contract.address, 100)).to.eventually.be.fulfilled
    await expect(stable_coin.query.balanceOf(lending.contract.address)).to.have.output(100)
    let sharesAddress = await lending.query.getSharesAddress(stable_coin.contract.address);
    
    //allow collateral for stable coint
    await expect(fromSigner(lending.contract, alice.address).tx.allowCollateral(stable_coin.contract.address)).to.eventually.be.fulfilled
    await expect(lending.query.isAcceptedCollateral(stable_coin.contract.address)).to.have.output(true)
    //borrow asset
    await expect(lending.tx.borrowAssets(stable_coin.contract.address, stable_coin.contract.address, 1)).to.eventually.be.fulfilled
    //repay
  })
  


})
