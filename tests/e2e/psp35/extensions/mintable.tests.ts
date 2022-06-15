import { expect, setupContract } from '../../helpers'

describe('MY_PSP35_MINTABLE', () => {
  async function setup() {
    return setupContract('my_psp35_mintable', 'new')
  }

  it('Mint works', async () => {
    const { contract, defaultSigner: sender, query, accounts: [alice] } = await setup()

    const token1 = {
      'u8': 0
    }
    const token2 = {
      'u8': 1
    }
    const amount1 = 1
    const amount2 = 10

    await expect(query.balanceOf(sender.address, token1)).to.have.output(0)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token1)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(0)

    await expect(contract.tx.mint(sender.address, [[token1, amount1]])).to.eventually.be.fulfilled
    await expect(contract.tx.mint(sender.address, [[token2, amount2]])).to.eventually.be.fulfilled
    await expect(contract.tx.mint(alice.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.output(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(amount2)
    await expect(query.balanceOf(alice.address, token1)).to.have.output(amount1)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(amount2)
  })
})
