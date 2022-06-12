import { expect, setupContract } from './../../helpers'

describe('MY_PSP22_PAUSABLE', () => {
  async function setup() {
    return await setupContract('my_psp22_pausable', 'new', '1000')
  }

  it('Can transfer when not paused', async () => {
    const { contract: pausable, query, defaultSigner: sender, accounts: [receiver] } = await setup()

    // sender has 1000 tokens
    await expect(query.balanceOf(sender.address)).to.have.output(1000)
    // receiver has 0 tokens
    await expect(query.balanceOf(receiver.address)).to.have.output(0)

    // sender sends tokens to the receiver
    await expect(pausable.tx.transfer(receiver.address, 100, [])).to.eventually.be.fulfilled

    // sender has 900 tokens
    await expect(query.balanceOf(sender.address)).to.have.output(900)
    // receiver has 100 tokens
    await expect(query.balanceOf(receiver.address)).to.have.output(100)
  })

  it('Can not transfer when paused', async () => {
    const { contract: pausable, query, defaultSigner: sender, accounts: [receiver] } = await setup()

    // sender has 1000 tokens
    await expect(query.balanceOf(sender.address)).to.have.output(1000)
    // receiver has 0 tokens
    await expect(query.balanceOf(receiver.address)).to.have.output(0)
    // we pause the contract
    await expect(pausable.tx.changeState()).to.eventually.be.fulfilled

    // sender sends tokens to the receiver
    await expect(pausable.tx.transfer(receiver.address, 100, [])).to.eventually.be.rejected

    // sender has 1000 tokens
    await expect(query.balanceOf(sender.address)).to.have.output(1000)
    // receiver has 0 tokens
    await expect(query.balanceOf(receiver.address)).to.have.output(0)
  })

})
