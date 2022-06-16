import { expect, fromSigner, setupContract} from '../../helpers'

describe('MY_PSP35_BATCH', () => {
  async function setup() {
    return setupContract('my_psp35_batch', 'new')
  }

  it('Batch Transfer should work', async () =>{
    const { defaultSigner: sender, accounts: [alice], query, tx } = await setup()

    const token1 = { 
      'u8': 0
    }
    const token2 = { 
      'u8': 1
    }
    const amount1 = 1
    const amount2 = 20

    await expect(tx.mint(sender.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    await expect(tx.batchTransfer(alice.address, [[token1, amount1], [token2, amount2]], [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(alice.address, token1)).to.have.output(amount1)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(amount2)
    await expect(query.balanceOf(sender.address, token1)).to.have.output(0)
    await expect(query.balanceOf(sender.address, token1)).to.have.output(0)
  })

  it(' Batch transfer from should work', async () => {
    const { defaultSigner: sender, accounts: [alice], query, tx } = await setup()

    const token1 = {
      'u8': 0
    }
    const token2 = {
      'u8': 1
    }
    const amount1 = 1
    const amount2 = 20

    await expect(tx.mint(sender.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    await expect(query.balanceOf(alice.address, token1)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(0)
    await expect(query.balanceOf(sender.address, token1)).to.have.output(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(amount2)

    await expect(tx.batchTransferFrom(sender.address, alice.address, [[token1, amount1], [token2, amount2]], [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.output(0)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token1)).to.have.output(amount1)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(amount2)
  })

  it('Batch transfer from with insufficient balance should fail', async () => {
    const { defaultSigner: sender, accounts: [alice], tx } = await setup()

    const token1 = {
      'u8': 0
    }
    const token2 = {
      'u8': 1
    }
    const amount1 = 1
    const amount2 = 20

    await expect(tx.mint(sender.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    await expect(tx.batchTransferFrom(sender.address, alice.address, [[token1, amount1 + 1], [token2, amount2 + 1]], [])).to.eventually.be.rejected
  })

  it('Batch transfer from with no approve should fail', async () => {
    const { defaultSigner: sender, accounts: [alice], tx } = await setup()

    const token1 = {
      'u8': 0
    }
    const token2 = {
      'u8': 1
    }
    const amount1 = 1
    const amount2 = 20

    await expect(tx.mint(alice.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled
    await expect(tx.batchTransferFrom(alice.address, sender.address, [[token1, amount1], [token2, amount2]], [])).to.eventually.be.rejected
  })

  it('Batch transfer from with approve should work', async () => {
    const { contract, defaultSigner: sender, accounts: [alice], query, tx } = await setup()

    const token1 = {
      'u8': 0
    }
    const token2 = {
      'u8': 1
    }
    const amount1 = 1
    const amount2 = 20

    await expect(tx.mint(sender.address, [[token1, amount1], [token2, amount2]], [])).to.eventually.be.fulfilled
    await expect(tx.approve(alice.address, null, 1)).to.eventually.be.fulfilled

    await expect(fromSigner(contract, alice.address).tx.batchTransferFrom(sender.address, alice.address, [[token1, amount1], [token2, amount2]], [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(alice.address, token1)).to.have.output(amount1)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(amount2)
    await expect(query.balanceOf(sender.address, token1)).to.have.output(0)
    await expect(query.balanceOf(sender.address, token1)).to.have.output(0)
  })
})