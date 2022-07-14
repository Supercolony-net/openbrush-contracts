import { expect, setupContract, fromSigner } from '../helpers'

describe('MY_PSP35', () => {
  async function setup() {
    return setupContract('my_psp35', 'new')
  }

  async function setup_receiver() {
    return setupContract('psp35_receiver', 'new')
  }

  it('Balance of works', async () => {
    const { query, defaultSigner: sender, tx } = await setup()
    const token1 = {
      'u8': 0
    }
    const token2 = {
      'u8': 1
    }

    const amount1 = 1
    const amount2 = 20

    await expect(query.balanceOf(sender.address, null)).to.have.output(0)
    await expect(tx.mintTokens(token1, amount1)).to.eventually.be.fulfilled
    await expect(tx.mintTokens(token2, amount2)).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.output(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(amount2)
    await expect(query.balanceOf(sender.address, null)).to.have.output(2)
  })

  it('Total supply works', async () => {
    const { query, defaultSigner: sender, tx } = await setup()
    const token1 = {
      'u8': 0
    }
    const token2 = {
      'u8': 1
    }
    
    const amount1 = 1
    const amount2 = 20

    await expect(query.totalSupply(null)).to.have.output(0)
    await expect(tx.mintTokens(token1, amount1)).to.eventually.be.fulfilled
    
    await expect(query.totalSupply(token1)).to.have.output(amount1)
    await expect(query.totalSupply(null)).to.have.output(1)

    await expect(tx.mintTokens(token2, amount2)).to.eventually.be.fulfilled

    await expect(query.totalSupply(token2)).to.have.output(amount2)
    await expect(query.totalSupply(null)).to.have.output(2)
  })

  it('Allowance works', async () => {
    const { query, defaultSigner: sender, accounts: [alice], tx } = await setup()
    const token = {
      'u8': 0
    }

    await expect(query.allowance(sender.address, alice.address, token)).to.have.output(0)
    await expect(tx.approve(alice.address, token, 10)).to.eventually.be.fulfilled
    await expect(query.allowance(sender.address, alice.address, token)).to.have.output(10)
  })

  it('PSP 35 - contract(not receiver) can accept the transfer', async () => {
    const { tx, query, defaultSigner: sender } = await setup()

    const { contract } = await setup()
    const token = {
      'u8': 0
    }

    // Arrange
    await expect(tx.mintTokens(token, 1)).to.eventually.be.fulfilled
    await expect(query.balanceOf(contract.address, token)).to.have.output(0)
    await expect(query.balanceOf(sender.address, token)).to.have.output(1)

    // Assert - Sender can send token to receiver
    await expect(tx.transferFrom(sender.address, contract.address, token, 1, 'data')).to.eventually.be.fulfilled
    await expect(query.balanceOf(contract.address, token)).to.have.output(1)
    await expect(query.balanceOf(sender.address, token)).to.have.output(0)
  })

  it('PSP 35 - receiver can accept the transfer', async () => {
    const { tx, query, defaultSigner: sender } = await setup()

    const { contract } = await setup_receiver()
    const token = {
      'u8': 0
    }

    // Arrange
    await expect(tx.mintTokens(token, 1)).to.eventually.be.fulfilled
    await expect(query.balanceOf(contract.address, token)).to.have.output(0)
    await expect(query.balanceOf(sender.address, token)).to.have.output(1)


    // Assert - Sender can send token to receiver
    await expect(tx.transferFrom(sender.address, contract.address, token, 1, 'data')).to.eventually.be.fulfilled
    await expect(query.balanceOf(contract.address, token)).to.have.output(1)
    await expect(query.balanceOf(sender.address, token)).to.have.output(0)
  })

  it('PSP 35 - receiver can reject the transfer', async () => {
    const { tx, query, defaultSigner: sender } = await setup()

    const { contract } = await setup_receiver()
    const token = {
      'u8': 0
    }

    // Arrange
    await expect(tx.mintTokens(token, 1)).to.eventually.be.fulfilled
    await expect(query.balanceOf(contract.address, token)).to.have.output(0)
    await expect(query.balanceOf(sender.address, token)).to.have.output(1)


    // Act - Receiver wants to reject the next transfer
    await expect(contract.tx.revertNextTransfer()).to.eventually.be.fulfilled

    // Assert - Sender cannot send token to receiver
    await expect(tx.transferFrom(sender.address, contract.address, token, 1, 'data')).to.eventually.be.rejected
    await expect(query.balanceOf(contract.address, token)).to.have.output(0)
    await expect(query.balanceOf(sender.address, token)).to.have.output(1)
  })

  it('Approve works', async () => {
    const { contract, query, defaultSigner: sender, accounts: [alice] } = await setup()

    const token = {
      'u8': 0
    }
    const tokenAmount = 20

    await expect(query.allowance(sender.address, alice.address, token))
      .to.have.output(0)

    await expect(contract.tx.approve(alice.address, token, tokenAmount)).to.eventually.be.fulfilled
    await expect(query.allowance(sender.address, alice.address, token))
      .to.have.output(tokenAmount)

    await expect(contract.tx.approve(alice.address, null, 1)).to.eventually.be.fulfilled
    await expect(query.allowance(sender.address, alice.address, token))
      .to.have.output('340282366920938463463374607431768211455')
  })

  it('Transfer works', async () => {
    const { contract, query, defaultSigner: sender, accounts: [alice], tx } = await setup()

    const token1 = {
      'u8': 0
    }
    const token2 = {
      'u8': 1
    }
    const token1Amount = 1
    const token2Amount = 10
    await expect(tx.mintTokens(token1, token1Amount)).to.eventually.be.fulfilled
    await expect(tx.mintTokens(token2, token2Amount)).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, null)).to.have.output(2)
    await expect(query.balanceOf(sender.address, token1)).to.have.output(token1Amount)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(token2Amount)
    await expect(query.totalSupply(null)).to.have.output(2)

    await expect(contract.tx.transfer(alice.address, token2, token2Amount, [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.output(token1Amount)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token1)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(token2Amount)
    await expect(query.balanceOf(sender.address, null)).to.have.output(1)
    await expect(query.balanceOf(alice.address, null)).to.have.output(1)

    await expect(contract.tx.transfer(alice.address, token1, token1Amount, [])).to.eventually.be.fulfilled
    await expect(fromSigner(contract, alice.address).tx.transfer(sender.address, token2, token1Amount, [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.output(0)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(token1Amount)
    await expect(query.balanceOf(alice.address, token1)).to.have.output(token1Amount)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(token2Amount - token1Amount)
    await expect(query.balanceOf(sender.address, null)).to.have.output(1)
    await expect(query.balanceOf(alice.address, null)).to.have.output(2)
  })

  it('Transfer from works', async () => {
    const { contract, query, defaultSigner: sender, accounts: [alice], tx } = await setup()

    const token1 = {
      'u8': 0
    }
    const token2 = {
      'u8': 1
    }
    const token1Amount = 1
    const token2Amount = 10
    await expect(tx.mintTokens(token1, token1Amount)).to.eventually.be.fulfilled
    await expect(tx.mintTokens(token2, token2Amount)).to.eventually.be.fulfilled

    await expect(fromSigner(contract, alice.address).tx.approve(sender.address, null, 1)).to.eventually.be.fulfilled
    await expect(contract.tx.transferFrom(sender.address, alice.address, token2, token2Amount, [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.output(token1Amount)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token1)).to.have.output(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(token2Amount)

    await expect(contract.tx.transferFrom(sender.address, alice.address, token1, token1Amount, [])).to.eventually.be.fulfilled
    await expect(contract.tx.transferFrom(alice.address, sender.address, token2, token1Amount, [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.output(0)
    await expect(query.balanceOf(sender.address, token2)).to.have.output(token1Amount)
    await expect(query.balanceOf(alice.address, token1)).to.have.output(token1Amount)
    await expect(query.balanceOf(alice.address, token2)).to.have.output(token2Amount - token1Amount)
  })

  it('Transfer from insufficient balance should fail', async () => {
    const { contract, defaultSigner: sender, query, accounts: [alice], tx } = await setup()

    const token = {
      'u8': 0
    }
    const tokenAmount = 1
    await expect(tx.mintTokens(token, tokenAmount)).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token)).to.have.output(tokenAmount)
    await fromSigner(contract, alice.address).tx.approve(sender.address, token, tokenAmount)

    await expect(contract.tx.transferFrom(sender.address, alice.address, token, tokenAmount + 1, []))
      .to.eventually.be.rejected

    await expect(query.balanceOf(sender.address, token)).to.have.output(tokenAmount)
  })

  it('Transfer from without allowance should fail', async () => {
    const { contract, defaultSigner: sender, accounts: [alice], query, tx } = await setup()

    const token = {
      'u8': 0
    }
    const tokenAmount = 1
    await expect(tx.mintTokens(token, tokenAmount)).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token)).to.have.output(tokenAmount)

    await expect(fromSigner(contract, alice.address).tx.transferFrom(sender.address, alice.address, token, tokenAmount, []))
      .to.eventually.be.rejected

    await expect(query.balanceOf(sender.address, token)).to.have.output(tokenAmount)
  })
})
