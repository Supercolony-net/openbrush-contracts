import { bnArg, expect, setupContract, fromSigner } from '../helpers'

describe('MY_PSP35', () => {
  async function setup() {
    return setupContract('my_psp35', 'new')
  }

  async function setup_receiver() {
    return setupContract('psp35_receiver', 'new')
  }

  it('Balance of works', async () => {
    const { query, defaultSigner: sender, tx } = await setup()

    await expect(query.balanceOf(sender.address, bnArg(0))).to.have.output(0)
    await expect(tx.mintTokens(bnArg(0), 1)).to.be.fulfilled
    await expect(query.balanceOf(sender.address, bnArg(0))).to.have.output(1)
  })

  it('Allowance works', async () => {
    const { query, defaultSigner: sender, accounts: [alice], tx } = await setup()
    await expect(query.allowance(sender.address, alice.address, bnArg(0))).to.have.output(0)
    await expect(tx.approve(alice.address, [bnArg(0), 10])).to.be.fulfilled
    await expect(query.allowance(sender.address, alice.address, bnArg(0))).to.have.output(10)
  })

  it('PSP 35 - contract(not receiver) can accept the transfer', async () => {
    const { tx, query, defaultSigner: sender } = await setup()

    const { contract } = await setup()

    // Arrange
    await expect(tx.mintTokens(bnArg(0), 1)).to.be.fulfilled
    await expect(query.balanceOf(contract.address, bnArg(0))).to.have.output(0)
    await expect(query.balanceOf(sender.address, bnArg(0))).to.have.output(1)

    // Assert - Sender can send token to receiver
    await expect(tx.transferFrom(sender.address, contract.address, bnArg(0), 1, 'data')).to.eventually.be.fulfilled
    await expect(query.balanceOf(contract.address, bnArg(0))).to.have.output(1)
    await expect(query.balanceOf(sender.address, bnArg(0))).to.have.output(0)
  })

  it('PSP 35 - receiver can accept the transfer', async () => {
    const { tx, query, defaultSigner: sender } = await setup()

    const { contract } = await setup_receiver()

    // Arrange
    await expect(tx.mintTokens(bnArg(0), 1)).to.be.fulfilled
    await expect(query.balanceOf(contract.address, bnArg(0))).to.have.output(0)
    await expect(query.balanceOf(sender.address, bnArg(0))).to.have.output(1)


    // Assert - Sender can send token to receiver
    await expect(tx.transferFrom(sender.address, contract.address, bnArg(0), 1, 'data')).to.eventually.be.fulfilled
    await expect(query.balanceOf(contract.address, bnArg(0))).to.have.output(1)
    await expect(query.balanceOf(sender.address, bnArg(0))).to.have.output(0)
  })

  it('PSP 35 - receiver can reject the transfer', async () => {
    const { tx, query, defaultSigner: sender } = await setup()

    const { contract } = await setup_receiver()

    // Arrange
    await expect(tx.mintTokens(bnArg(0), 1)).to.be.fulfilled
    await expect(query.balanceOf(contract.address, bnArg(0))).to.have.output(0)
    await expect(query.balanceOf(sender.address, bnArg(0))).to.have.output(1)


    // Act - Receiver wants to reject the next transfer
    await expect(contract.tx.revertNextTransfer()).to.eventually.be.fulfilled

    // Assert - Sender cannot send token to receiver
    await expect(tx.transferFrom(sender.address, contract.address, bnArg(0), 1, 'data')).to.eventually.be.rejected
    await expect(query.balanceOf(contract.address, bnArg(0))).to.have.output(0)
    await expect(query.balanceOf(sender.address, bnArg(0))).to.have.output(1)
  })

  it('Approve works', async () => {
    const { contract, query, defaultSigner: sender, accounts: [alice] } = await setup()

    const tokenId = bnArg(0)
    const tokenAmount = 20

    await expect(query.allowance(sender.address, alice.address, tokenId))
      .to.have.output(0)

    await expect(contract.tx.approve(alice.address, [tokenId, tokenAmount])).to.be.fulfilled
    await expect(query.allowance(sender.address, alice.address, tokenId))
      .to.have.output(tokenAmount)

    await expect(contract.tx.approve(alice.address, null)).to.be.fulfilled
    await expect(query.allowance(sender.address, alice.address, tokenId))
      .to.have.output('340282366920938463463374607431768211455')
  })

  it('Transfer works', async () => {
    const { contract, query, defaultSigner: sender, accounts: [alice], tx } = await setup()

    const tokenId1 = bnArg(0)
    const tokenId2 = bnArg(1)
    const token1Amount = 1
    const token2Amount = 10
    await expect(tx.mintTokens(tokenId1, token1Amount)).to.be.fulfilled
    await expect(tx.mintTokens(tokenId2, token2Amount)).to.be.fulfilled

    await expect(contract.tx.transfer(alice.address, tokenId2, token2Amount, [])).to.be.fulfilled

    await expect(query.balanceOf(sender.address, tokenId1)).to.have.output(token1Amount)
    await expect(query.balanceOf(sender.address, tokenId2)).to.have.output(0)
    await expect(query.balanceOf(alice.address, tokenId1)).to.have.output(0)
    await expect(query.balanceOf(alice.address, tokenId2)).to.have.output(token2Amount)

    await expect(contract.tx.transfer(alice.address, tokenId1, token1Amount, [])).to.be.fulfilled
    await expect(fromSigner(contract, alice.address).tx.transfer(sender.address, tokenId2, token1Amount, [])).to.be.fulfilled

    await expect(query.balanceOf(sender.address, tokenId1)).to.have.output(0)
    await expect(query.balanceOf(sender.address, tokenId2)).to.have.output(token1Amount)
    await expect(query.balanceOf(alice.address, tokenId1)).to.have.output(token1Amount)
    await expect(query.balanceOf(alice.address, tokenId2)).to.have.output(token2Amount - token1Amount)
  })

  it('Transfer from works', async () => {
    const { contract, query, defaultSigner: sender, accounts: [alice], tx } = await setup()

    const tokenId1 = bnArg(0)
    const tokenId2 = bnArg(1)
    const token1Amount = 1
    const token2Amount = 10
    await expect(tx.mintTokens(tokenId1, token1Amount)).to.be.fulfilled
    await expect(tx.mintTokens(tokenId2, token2Amount)).to.be.fulfilled

    await expect(fromSigner(contract, alice.address).tx.approve(sender.address, null)).to.be.fulfilled
    await expect(contract.tx.transferFrom(sender.address, alice.address, tokenId2, token2Amount, [])).to.be.fulfilled

    await expect(query.balanceOf(sender.address, tokenId1)).to.have.output(token1Amount)
    await expect(query.balanceOf(sender.address, tokenId2)).to.have.output(0)
    await expect(query.balanceOf(alice.address, tokenId1)).to.have.output(0)
    await expect(query.balanceOf(alice.address, tokenId2)).to.have.output(token2Amount)

    await expect(contract.tx.transferFrom(sender.address, alice.address, tokenId1, token1Amount, [])).to.be.fulfilled
    await expect(contract.tx.transferFrom(alice.address, sender.address, tokenId2, token1Amount, [])).to.be.fulfilled

    await expect(query.balanceOf(sender.address, tokenId1)).to.have.output(0)
    await expect(query.balanceOf(sender.address, tokenId2)).to.have.output(token1Amount)
    await expect(query.balanceOf(alice.address, tokenId1)).to.have.output(token1Amount)
    await expect(query.balanceOf(alice.address, tokenId2)).to.have.output(token2Amount - token1Amount)
  })

  it('Transfer from insufficient balance should fail', async () => {
    const { contract, defaultSigner: sender, query, accounts: [alice], tx } = await setup()

    const tokenId = bnArg(0)
    const tokenAmount = 1
    await expect(tx.mintTokens(tokenId, tokenAmount)).to.be.fulfilled

    await expect(query.balanceOf(sender.address, tokenId)).to.have.output(tokenAmount)
    await fromSigner(contract, alice.address).tx.approve(sender.address, [tokenId, tokenAmount])

    await expect(contract.tx.transferFrom(sender.address, alice.address, tokenId, tokenAmount + 1, []))
      .to.eventually.be.rejected

    await expect(query.balanceOf(sender.address, tokenId)).to.have.output(tokenAmount)
  })

  it('Transfer from without allowance should fail', async () => {
    const { contract, defaultSigner: sender, accounts: [alice], query, tx } = await setup()

    const tokenId = bnArg(0)
    const tokenAmount = 1
    await expect(tx.mintTokens(tokenId, tokenAmount)).to.be.fulfilled

    await expect(query.balanceOf(sender.address, tokenId)).to.have.output(tokenAmount)

    await expect(fromSigner(contract, alice.address).tx.transferFrom(sender.address, alice.address, tokenId, tokenAmount, []))
      .to.eventually.be.rejected

    await expect(query.balanceOf(sender.address, tokenId)).to.have.output(tokenAmount)
  })
})
