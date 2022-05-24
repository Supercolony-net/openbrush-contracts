import { bnArg, expect, setupContract } from '../../helpers'

describe('MY_PSP35_BURNABLE', () => {
  async function setup() {
    return setupContract('my_psp35_burnable', 'new')
  }

  it('Burn works', async () => {
    const { contract, query, defaultSigner: sender, accounts: [alice] } = await setup()

    const tokenId = bnArg(0)
    const tokenId2 = bnArg(1)
    const mintAmount = 1
    const mintAmount2 = 20

    await expect(contract.tx.transferFrom(sender.address, alice.address, tokenId, mintAmount, [])).to.be.fulfilled
    await expect(query.balanceOf(alice.address, tokenId)).to.have.output(mintAmount)
    await expect(query.balanceOf(sender.address, tokenId2)).to.have.output(mintAmount2)

    await expect(contract.tx.burn(sender.address, [[tokenId2, mintAmount2]])).to.be.fulfilled
    await expect(contract.tx.burn(alice.address, [[tokenId, mintAmount]])).to.be.fulfilled

    await expect(query.balanceOf(sender.address, tokenId)).to.have.output(0)
    await expect(query.balanceOf(alice.address, tokenId2)).to.have.output(0)
  })

  it('Burn batch works', async () => {
    const { contract, query, defaultSigner: sender, accounts: [alice] } = await setup()

    const token1 = bnArg(0)
    const token2 = bnArg(1)
    const amount1 = 1
    const amount2 = 10

    await expect(contract.tx.transferFrom(sender.address, alice.address, token2, amount2, [])).to.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.output(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(amount2)
    await expect(query.balanceOf(alice.address, token1)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(amount2)

    await contract.tx.burn(sender.address, [[token1, amount1], [token2, amount2]])
    await contract.tx.burn(alice.address, [[token1, 0], [token2, amount2]])

    await expect(query.balanceOf(sender.address, token1)).to.have.output(0)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token1)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(0)

  })

  it('Burn insufficient balance should fail', async () => {
    const { contract, defaultSigner: sender, query, accounts: [alice] } = await setup()

    const token1 = bnArg(0)
    const token2 = bnArg(1)
    const amount1 = 1
    const amount2 = 20

    await expect(query.balanceOf(sender.address, token1)).to.have.output(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(amount2)
    await expect(query.balanceOf(alice.address, token1)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(0)

    await expect(contract.tx.burn(sender.address, [[token1, amount1 + 1], [token2, amount2]]))
      .to.eventually.be.rejected
    await expect(contract.tx.burn(sender.address, [[token1, amount1 + 1]]))
      .to.eventually.be.rejected

    await expect(contract.tx.burn(alice.address, [[token1, amount1 + 1], [token2, amount2]]))
      .to.eventually.be.rejected
    await expect(contract.tx.burn(alice.address, [[token1, amount1 + 1]]))
      .to.eventually.be.rejected

    await expect(query.balanceOf(sender.address, token1)).to.have.output(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(amount2)
    await expect(query.balanceOf(alice.address, token1)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(0)
  })
})
