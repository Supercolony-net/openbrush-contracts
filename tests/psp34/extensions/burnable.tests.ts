import { bnArg, expect, fromSigner, setupContract } from '../../helpers'

describe('MY_PSP34_BURNABLE', () => {
  async function setup() {
    return setupContract('my_psp34_burnable', 'new')
  }

  it('Burn works', async () => {
    const {
      contract,
      defaultSigner: sender,
      query
    } = await setup()

    await expect(query.balanceOf(sender.address)).to.have.output(3)

    await expect(contract.tx.burn(sender.address, 0)).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address)).to.have.output(2)
  })

  it('Burn from works', async () => {
    const {
      contract,
      defaultSigner: sender,
      accounts: [alice],
      query
    } = await setup()

    await expect(query.balanceOf(sender.address)).to.have.output(3)

    await expect(fromSigner(contract, alice.address).tx.burn(sender.address, 0)).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address)).to.have.output(2)
  })
})
