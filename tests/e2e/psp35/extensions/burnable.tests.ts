import { expect, setupContract } from '../../helpers'
import exp from "constants";

describe('MY_PSP35_BURNABLE', () => {
  async function setup() {
    return setupContract('my_psp35_burnable', 'new')
  }

  it('Burn works', async () => {
    const { contract, query, defaultSigner: sender, accounts: [alice] } = await setup()

    const token1 = {
      'u8': 0
    }
    const token2 = {
      'u8': 1
    }
    const amount1 = 1
    const amount2 = 20
    
    await expect(contract.tx.mintTo(sender.address, [[token1, amount1], [token2, amount2]])).to.be.fulfilled
    await expect(contract.tx.transferFrom(sender.address, alice.address, token1, amount1, [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, null)).to.have.output(1)
    await expect(query.balanceOf(alice.address, null)).to.have.output(1)

    await expect(query.balanceOf(alice.address, token1)).to.have.output(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(amount2)
    await expect(query.totalSupply(null)).to.have.output(2)

    await expect(contract.tx.burn(sender.address, [[token2, amount2]])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, null)).to.have.output(0)
    await expect(query.balanceOf(alice.address, null)).to.have.output(1)
    await expect(query.totalSupply(null)).to.have.output(1)
    await expect(query.totalSupply(token2)).to.have.output(0)
    await expect(query.totalSupply(token1)).to.have.output(amount1)

    await expect(contract.tx.burn(alice.address, [[token1, amount1]])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(0)
    await expect(query.totalSupply(null)).to.have.output(0)
    await expect(query.totalSupply(token1)).to.have.output(0)
    await expect(query.totalSupply(token2)).to.have.output(0)
  })

  it('Burn batch works', async () => {
    const { contract, query, defaultSigner: sender, accounts: [alice] } = await setup()

    const token1 = {
      'u8': 0
    }
    const token2 = {
      'u8': 1
    }
    const amount1 = 1
    const amount2 = 10

    await expect(contract.tx.mintTo(sender.address, [[token1, amount1], [token2, 20]])).to.eventually.be.fulfilled

    await expect(contract.tx.transferFrom(sender.address, alice.address, token2, amount2, [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.output(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(amount2)
    await expect(query.balanceOf(alice.address, token1)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(amount2)

    await expect(contract.tx.burn(sender.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled
    await expect(contract.tx.burn(alice.address, [[token1, 0], [token2, amount2]])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.output(0)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token1)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(0)

  })

  it('Burn insufficient balance should fail', async () => {
    const { contract, defaultSigner: sender, query, accounts: [alice] } = await setup()

    const token1 = {
      'u8': 0
    }
    const token2 = {
      'u8': 1
    }
    const amount1 = 1
    const amount2 = 20

    await expect(contract.tx.mintTo(sender.address, [[token1, amount1], [token2, amount2]])).to.be.fulfilled

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
