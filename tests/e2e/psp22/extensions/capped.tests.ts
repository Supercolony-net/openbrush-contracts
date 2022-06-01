import { expect, setupContract } from './../../helpers'

describe('MY_PSP22_CAPPED', () => {
  async function setup() {
    return await setupContract('my_psp22_capped', 'new', '1000', '2000')
  }

  it('New works', async () => {
    const { query, defaultSigner: sender } = await setup()

    await expect(query.balanceOf(sender.address)).to.have.output(1000)
    await expect(query.totalSupply()).to.have.output(1000)
    await expect(query.cap()).to.have.output(2000)
  })

  it('Can mint when total supply is lower than cap', async () => {
    const { contract, query, defaultSigner: sender } = await setup()

    const mintAmount = 1000

    await expect(query.balanceOf(sender.address)).to.have.output(mintAmount)
    await expect(query.totalSupply()).to.have.output(mintAmount)

    // mint tokens to sender
    await expect(contract.tx.mint(sender.address, mintAmount)).to.eventually.be.fulfilled

    // sender's balance changed
    await expect(query.balanceOf(sender.address)).to.have.output(mintAmount + mintAmount)
    // total supply changed
    await expect(query.totalSupply()).to.have.output(mintAmount + mintAmount)
  })

  it('Can not mint if total supply will exceed the cap', async () => {
    const { contract, query, defaultSigner: sender } = await setup()
        
    const mintAmount = 1000
    const newMintAmount = 1001
        
    await expect(query.balanceOf(sender.address)).to.have.output(mintAmount)
    await expect(query.totalSupply()).to.have.output(mintAmount)
        
    // mint tokens to sender
    await expect(contract.tx.mint(sender.address, newMintAmount)).to.eventually.be.rejected
        
    // sender's balance did not change
    await expect(query.balanceOf(sender.address)).to.have.output(mintAmount)
    // total supply did not change
    await expect(query.totalSupply()).to.have.output(mintAmount)
  })

})
