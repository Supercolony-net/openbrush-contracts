import { expect, setupContract, fromSigner } from './../../helpers'

describe('MY_PSP22_WRAPPER', () => {
  async function setup() {
    const psp22 = await setupContract('my_psp22', 'new', '1000')
    const wrapper = await setupContract('my_psp22_wrapper', 'new', psp22.contract.address)
    return { psp22, wrapper }
  }

  it('Deposit for works', async () => {
    const { psp22, wrapper } = await setup()
    const { contract: wrapperContract, query: wrapperQuery } = wrapper
    const { contract: psp22Contract, query: psp22Query, defaultSigner: sender } = psp22

    // sender has 1000 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.output(1000)
    // sender has 0 wrapper tokens
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.output(0)
    // contract has 0 tokens
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.output(0)

    // sender allows the contract to spend their tokens
    await expect(psp22Contract.tx.approve(wrapperContract.address, 100)).to.eventually.be.fulfilled
    // sender wraps 100 tokens
    await expect(fromSigner(wrapperContract, sender.address).tx.depositFor(sender.address, 100)).to.eventually.be.fulfilled

    // sender has 900 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.output(900)
    // sender has 100 wrapper tokens
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.output(100)
    // contract has 100 tokens
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.output(100)
  })

  it('Deposit without allowance should fail', async () => {
    const { psp22, wrapper } = await setup()
    const { contract: wrapperContract, query: wrapperQuery } = wrapper
    const { query: psp22Query, defaultSigner: sender } = psp22

    // sender has 1000 tokens and 0 wrapper tokens, contract has 0 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.output(1000)
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.output(0)
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.output(0)

    // sender deposits 100 tokens
    await expect(fromSigner(wrapperContract, sender.address).tx.depositFor(sender.address, 100)).to.eventually.be.rejected

    // sender has 1000 tokens and 0 wrapper tokens, contract has 0 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.output(1000)
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.output(0)
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.output(0)
  })

  it('Deposit without underlying tokens should fail', async () => {
    const { psp22, wrapper } = await setup()
    const { contract: wrapperContract, query: wrapperQuery } = wrapper
    const { contract: psp22Contract, query: psp22Query, defaultSigner: sender } = psp22

    // sender has 1000 tokens and 0 wrapper tokens, contract has 0 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.output(1000)
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.output(0)
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.output(0)

    // sender approves contract to spend their tokens, sender deposits 1001 tokens
    await expect(psp22Contract.tx.approve(wrapperContract.address, 100)).to.eventually.be.fulfilled
    await expect(fromSigner(wrapperContract, sender.address).tx.depositFor(sender.address, 1001)).to.eventually.be.rejected

    // sender has 1000 tokens and 0 wrapper tokens, contract has 0 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.output(1000)
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.output(0)
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.output(0)
  })

  it('Withdraw to works', async () => {
    const { psp22, wrapper } = await setup()
    const { contract: wrapperContract, query: wrapperQuery } = wrapper
    const { contract: psp22Contract, query: psp22Query, defaultSigner: sender } = psp22

    // sender allows the contract to spend their tokens
    await expect(psp22Contract.tx.approve(wrapperContract.address, 100)).to.eventually.be.fulfilled
    // sender wraps 100 tokens
    await expect(fromSigner(wrapperContract, sender.address).tx.depositFor(sender.address, 100)).to.eventually.be.fulfilled
    // sender has 900 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.output(900)
    // sender has 100 wrapper tokens
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.output(100)
    // contract has 100 tokens
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.output(100)

    // sender withdraws 100 tokens
    await expect(fromSigner(wrapperContract, sender.address).tx.withdrawTo(sender.address, 100)).to.eventually.be.fulfilled

    // sender has 1000 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.output(1000)
    // sender has 0 wrapped tokens
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.output(0)
    // contract has 0 tokens
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.output(0)
  })

  it('Withdraw without deposit should fail', async () => {
    const { psp22, wrapper } = await setup()
    const { contract: wrapperContract, query: wrapperQuery } = wrapper
    const { query: psp22Query, defaultSigner: sender } = psp22

    // sender has 1000 tokens and 0 wrapper tokens, contract has 0 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.output(1000)
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.output(0)
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.output(0)

    // sender withdraws 100 tokens
    await expect(fromSigner(wrapperContract, sender.address).tx.withdrawTo(sender.address, 100)).to.eventually.be.rejected

    // sender has 1000 tokens and 0 wrapper tokens, contract has 0 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.output(1000)
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.output(0)
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.output(0)
  })

  it('Recover works', async () => {
    const { psp22, wrapper } = await setup()
    const { contract: wrapperContract, query: wrapperQuery } = wrapper
    const { contract: psp22Contract, query: psp22Query, defaultSigner: sender } = psp22

    // sender send 100 tokens to the wrapper contract
    await expect(psp22Contract.tx.transfer(wrapperContract.address, 100, [])).to.eventually.be.fulfilled
    // sender has 900 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.output(900)
    // sender has 0 wrapper tokens
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.output(0)
    // contract has 100 tokens
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.output(100)

    // sender calls recover function
    await expect(fromSigner(wrapperContract, sender.address).tx.recover()).to.eventually.be.fulfilled

    // sender has 900 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.output(900)
    // sender has 100 wrapped tokens
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.output(100)
    // contract has 100 tokens
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.output(100)
  })

})
