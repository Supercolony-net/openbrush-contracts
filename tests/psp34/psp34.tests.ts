import { expect, setupContract, fromSigner } from '../helpers'

describe('MY_PSP34', () => {
  async function setup() {
    return setupContract('my_psp34', 'new')
  }

  async function setup_receiver() {
    return setupContract('psp34_receiver', 'new')
  }

  // TODO: change it when collection_id is implemented
  it('Fail to return collection_id (as it is not implemented yet)', async () => {
    const { query } = await setup()

    await expect(query.collectionId()).to.throw
  })

  it('Returns total supply', async () => {
    const {
      query,
      tx
    } = await setup()

    await expect(query.totalSupply()).to.have.output(0)
    await expect(tx.mintToken()).to.be.fulfilled
    await expect(tx.mintToken()).to.be.fulfilled
    await expect(tx.mintToken()).to.be.fulfilled

    await expect(query.totalSupply()).to.have.output(3)
  })

  it('Transfer works', async () => {
    const {
      contract,
      defaultSigner: sender,
      accounts: [alice],
      query,
      tx
    } = await setup()

    await expect(tx.mintToken()).to.be.fulfilled

    await expect(query.balanceOf(sender.address)).to.have.output(1)
    await expect(query.balanceOf(alice.address)).to.have.output(0)

    await contract.tx.transferFrom(sender.address, alice.address, 0, [])

    await expect(query.balanceOf(sender.address)).to.have.output(0)
    await expect(query.balanceOf(alice.address)).to.have.output(1)
  })

  it('Transfer from works', async () => {
    const {
      contract,
      defaultSigner: sender,
      accounts: [alice],
      query,
      tx
    } = await setup()

    await expect(tx.mintToken()).to.be.fulfilled

    await expect(query.balanceOf(sender.address)).to.have.output(1)
    await expect(query.balanceOf(alice.address)).to.have.output(0)
    await contract.tx.setApprovalForAll(alice.address, true)

    await fromSigner(contract, alice.address).tx.transferFrom(sender.address, alice.address, 0, [])

    await expect(query.balanceOf(sender.address)).to.have.output(0)
    await expect(query.balanceOf(alice.address)).to.have.output(1)
  })

  it('PSP 34 - safe transfer works', async () => {
    const {
      tx,
      query,
      defaultSigner: sender
    } = await setup()

    const { contract } = await setup_receiver()

    // Arrange - Sender mint a Token and Approve Receiver as spender of this token
    await expect(tx.mintToken()).to.be.fulfilled
    await expect(query.ownerOf(0)).to.have.output(sender.address)

    // Act - Alice transfers the token form sender to bob
    await expect(contract.query.getCallCounter()).to.have.output(0)
    await expect(tx.transferFrom(sender.address, contract.address, 0, 'data')).to.eventually.be.fulfilled
    await expect(contract.query.getCallCounter()).to.have.output(1)

    // Assert - Bob is now owner of the token
    await expect(query.ownerOf(0)).to.have.output(contract.address.toString())
  })

  it('PSP 34 - safe transfer works to contract but not PSP34Receiver', async () => {
    const {
      tx,
      query,
      defaultSigner: sender
    } = await setup()

    const { contract } = await setup()

    // Arrange - Sender mint a Token and Approve Receiver as spender of this token
    await expect(tx.mintToken()).to.be.fulfilled
    await expect(query.ownerOf(0)).to.have.output(sender.address)

    // Act - Alice transfers the token form sender to bob
    await expect(tx.transferFrom(sender.address, contract.address, 0, 'data')).to.eventually.be.fulfilled

    // Assert - Bob is now owner of the token
    await expect(query.ownerOf(0)).to.have.output(contract.address.toString())
  })

  it('PSP 34 - safe transfer works to contract that implements PSP34Receiver', async () => {
    const { tx, query, defaultSigner: sender } = await setup()

    const { contract } = await setup_receiver()

    // Arrange - Sender mint a token
    await expect(tx.mintToken()).to.be.fulfilled
    await expect(query.ownerOf(0)).to.have.output(sender.address)

    // Assert - Sender cannot send token to receiver & Sender still own the token
    await expect(tx.transferFrom(sender.address, contract.address, 0, 'data')).to.eventually.be.fulfilled
    await expect(query.ownerOf(0)).to.have.output(contract.address)
  })

  it('PSP 34 - receiver can reject the transfer', async () => {
    const { tx, query, defaultSigner: sender } = await setup()

    const { contract } = await setup_receiver()

    // Arrange - Sender mint a token
    await expect(tx.mintToken()).to.be.fulfilled
    await expect(query.ownerOf(0)).to.have.output(sender.address)

    // Act - Receiver wants to reject the next transfer
    await expect(contract.tx.revertNextTransfer()).to.eventually.be.fulfilled

    // Assert - Sender cannot send token to receiver & Sender still own the token
    await expect(tx.transferFrom(sender.address, contract.address, 0, 'data')).to.eventually.be.rejected
    await expect(query.ownerOf(0)).to.have.output(sender.address)
  })

  it('Can not transfer non-existing token', async () => {
    const {
      contract,
      accounts: [receiver],
      defaultSigner: sender,
      query
    } = await setup()

    await expect(query.balanceOf(sender.address)).to.have.output(0)

    await expect(contract.tx.transferFrom(sender.address, receiver.address, 0)).to.eventually.be.rejected

    await expect(query.balanceOf(sender.address)).to.have.output(0)
  })

  it('Can not transfer without allowance', async () => {
    const {
      contract,
      accounts: [alice],
      defaultSigner: sender,
      query,
      tx
    } = await setup()

    await expect(tx.mintToken()).to.be.fulfilled
    await expect(query.balanceOf(sender.address)).to.have.output(1)

    await expect(fromSigner(contract, alice.address).tx.transferFrom(sender.address, alice.address, 0, []))
      .to.eventually.be.rejected

    await expect(query.balanceOf(sender.address)).to.have.output(1)
  })

})
