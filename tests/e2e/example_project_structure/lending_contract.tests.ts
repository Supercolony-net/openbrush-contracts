import { expect, setupContract, fromSigner } from '../helpers'
import BN from 'bn.js'

describe('LENDING_CONTRACT', () => {
  async function setup() {
    const stable_coin = await setupContract('stable_coin_contract', 'new', 'Green Coin', 'GC')
    const loan = await (await setupContract('loan_contract', 'new' )).abi
    const shares = await (await setupContract('shares_contract', 'new', '', '')).abi
    const lending = await setupContract('lending_contract', 'new', shares.source.hash, loan.source.hash)

    return { lending, stable_coin, alice: stable_coin.defaultSigner }
  }

  it('LENDING CONTRACT - lend asset', async () => {
    const { lending, stable_coin, alice } = await setup()

    const amount = 100

    // Alice balance should be >= than lending `amount`
    const alice_balance = (await stable_coin.query.balanceOf(alice.address)).output
    expect(alice_balance).to.gte(amount)

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
})
