import { expect, setupContract } from './../../helpers'

describe('MY_PSP22_FLASHMINT', () => {
  async function setup() {
    const flashmint = await setupContract('my_psp22_flashmint', 'new', '1000')
    const receiver = await setupContract('flash_borrower', 'new')
    return { flashmint, receiver }
  }

  it('New works', async () => {
    const { flashmint } = await setup()

    // flash fee should be 1%
    const fee = await flashmint.query.flashFee(flashmint.contract.address, 100)
    await expect(fee.output!.toJSON()!['ok']).to.be.equal(1)
  })

  it('Flashloan works', async () => {
    const { flashmint, receiver } = await setup()
    const { contract: flashmintContract, query: flashmintQuery } = flashmint
    const { contract: receiverContract } = receiver

    const borrowAmount = 100
    const fee = borrowAmount / 100
    const sendAmount = borrowAmount + fee
    const minted = 1000

    // sender has the initial supply of tokens, we send some to the receiver
    await expect(flashmintContract.tx.transfer(receiverContract.address, sendAmount, [])).to.eventually.be.fulfilled
    await expect(flashmintQuery.balanceOf(receiverContract.address)).to.have.output(sendAmount)
    await expect(flashmintQuery.totalSupply()).to.have.output(minted)

    // we will do the flashloan
    await expect(flashmintContract.tx.flashloan(receiverContract.address, flashmintContract.address, sendAmount, []))
      .to.eventually.be.fulfilled

    // receiver should have the fee deducted
    await expect(flashmintQuery.balanceOf(receiverContract.address))
      .to.have.output(sendAmount - fee)
    // one token should be burned
    await expect(flashmintQuery.totalSupply()).to.have.output(minted - fee)
  })

})
