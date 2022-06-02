import { expect, fromSigner, setupContract } from '../helpers'

describe('MY_PSP34', () => {
  async function setup() {
    return setupContract('my_psp34', 'new')
  }

  async function setup_receiver() {
    return setupContract('psp34_receiver', 'new')
  }

  it('Return collection_id of account', async () => {
    const { query, contract } = await setup()

    const expected_collection_id = {
      'bytes' : contract.address.toU8a()
    }
    const actual_collection_id = await query.collectionId()
    expect(expected_collection_id).to.have.output(actual_collection_id)
  })

  it('Returns total supply', async () => {
    const {
      query,
      tx
    } = await setup()

    await expect(query.totalSupply()).to.have.output(0)
    await expect(tx.mintToken()).to.eventually.be.fulfilled
    await expect(tx.mintToken()).to.eventually.be.fulfilled
    await expect(tx.mintToken()).to.eventually.be.fulfilled

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

    await expect(tx.mintToken()).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address)).to.have.output(1)
    await expect(query.balanceOf(alice.address)).to.have.output(0)

    await expect(contract.tx.transfer(alice.address, 0, [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address)).to.have.output(0)
    await expect(query.balanceOf(alice.address)).to.have.output(1)
  })

  it('Approved transfer works', async () => {
    const {
      contract,
      defaultSigner: sender,
      accounts: [alice],
      query,
      tx
    } = await setup()

    await expect(tx.mintToken()).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address)).to.have.output(1)
    await expect(query.balanceOf(alice.address)).to.have.output(0)

    const token_id = {
      'u8' : 0
    }

    // Approve only transfer for token 1
    await expect(contract.tx.approve(alice.address, token_id, true)).to.eventually.be.fulfilled

    await fromSigner(contract, alice.address).tx.transfer(alice.address, 0, [])

    await expect(query.balanceOf(sender.address)).to.have.output(0)
    await expect(query.balanceOf(alice.address)).to.have.output(1)
  })

  it('Approved operator transfer works', async () => {
    const {
      contract,
      defaultSigner: sender,
      accounts: [alice],
      query,
      tx
    } = await setup()

    await expect(tx.mintToken()).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address)).to.have.output(1)
    await expect(query.balanceOf(alice.address)).to.have.output(0)
    // Approved transfer for any token
    await expect(contract.tx.approve(alice.address, null, true)).to.eventually.be.fulfilled

    await fromSigner(contract, alice.address).tx.transfer(alice.address, 0, [])

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

    // Arrange - Sender mint a Token
    await expect(tx.mintToken()).to.eventually.be.fulfilled
    await expect(query.ownerOf(0)).to.have.output(sender.address)

    // Act - Alice transfers the token form sender to bob
    await expect(contract.query.getCallCounter()).to.have.output(0)
    await expect(tx.transfer(contract.address, 0, 'data')).to.eventually.be.fulfilled
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

    // Arrange - Sender mint a Token
    await expect(tx.mintToken()).to.eventually.be.fulfilled
    await expect(query.ownerOf(0)).to.have.output(sender.address)

    // Act - Alice transfers the token form sender to bob
    await expect(tx.transfer(contract.address, 0, 'data')).to.eventually.be.fulfilled

    // Assert - Bob is now owner of the token
    await expect(query.ownerOf(0)).to.have.output(contract.address.toString())
  })

  it('PSP 34 - safe transfer works to contract that implements PSP34Receiver', async () => {
    const { tx, query, defaultSigner: sender } = await setup()

    const { contract } = await setup_receiver()

    // Arrange - Sender mint a token
    await expect(tx.mintToken()).to.eventually.be.fulfilled
    await expect(query.ownerOf(0)).to.have.output(sender.address)

    // Assert - Sender cannot send token to receiver & Sender still own the token
    await expect(tx.transfer(contract.address, 0, 'data')).to.eventually.be.fulfilled
    await expect(query.ownerOf(0)).to.have.output(contract.address)
  })

  it('PSP 34 - receiver can reject the transfer', async () => {
    const { tx, query, defaultSigner: sender } = await setup()

    const { contract } = await setup_receiver()

    // Arrange - Sender mint a token
    await expect(tx.mintToken()).to.eventually.be.fulfilled
    await expect(query.ownerOf(0)).to.have.output(sender.address)

    // Act - Receiver wants to reject the next transfer
    await expect(contract.tx.revertNextTransfer()).to.eventually.be.fulfilled

    // Assert - Sender cannot send token to receiver & Sender still own the token
    await expect(tx.transfer(contract.address, 0, 'data')).to.eventually.be.rejected
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

    await expect(contract.tx.transfer(receiver.address, 0)).to.eventually.be.rejected

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

    await expect(tx.mintToken()).to.eventually.be.fulfilled
    await expect(query.balanceOf(sender.address)).to.have.output(1)

    await expect(fromSigner(contract, alice.address).tx.transfer(alice.address, 0, []))
      .to.eventually.be.rejected

    await expect(query.balanceOf(sender.address)).to.have.output(1)
  })

  it('Can mint any Id', async () => {
    const {
      contract,
      accounts: [alice],
      defaultSigner: sender,
      query,
      tx
    } = await setup()

    await expect(query.balanceOf(sender.address)).to.have.output(0)
    await expect(query.ownerOf({ 'u8': 123 })).to.have.output(null)
    await expect(tx.mint({ 'u8': 123 })).to.eventually.be.fulfilled
    await expect(query.ownerOf({ 'u8': 123 })).to.have.output(sender.address)

    await expect(query.balanceOf(sender.address)).to.have.output(1)
    await expect(query.ownerOf({ 'u16': 123 })).to.have.output(null)
    await expect(tx.mint({ 'u16': 123 })).to.eventually.be.fulfilled
    await expect(query.ownerOf({ 'u16': 123 })).to.have.output(sender.address)

    await expect(query.balanceOf(sender.address)).to.have.output(2)
    await expect(query.ownerOf({ 'u32': 123 })).to.have.output(null)
    await expect(tx.mint({ 'u32': 123 })).to.eventually.be.fulfilled
    await expect(query.ownerOf({ 'u32': 123 })).to.have.output(sender.address)

    await expect(query.balanceOf(sender.address)).to.have.output(3)
    await expect(query.ownerOf({ 'u64': 123 })).to.have.output(null)
    await expect(tx.mint({ 'u64': 123 })).to.eventually.be.fulfilled
    await expect(query.ownerOf({ 'u64': 123 })).to.have.output(sender.address)

    await expect(query.balanceOf(sender.address)).to.have.output(4)
    await expect(query.ownerOf({ 'u128': 123 })).to.have.output(null)
    await expect(tx.mint({ 'u128': 123 })).to.eventually.be.fulfilled
    await expect(query.ownerOf({ 'u128': 123 })).to.have.output(sender.address)

    await expect(query.balanceOf(sender.address)).to.have.output(5)
    await expect(query.ownerOf({ 'bytes': [123] })).to.have.output(null)
    await expect(tx.mint({ 'bytes': [123] })).to.eventually.be.fulfilled
    await expect(query.ownerOf({ 'bytes': [123] })).to.have.output(sender.address)

    await expect(query.balanceOf(sender.address)).to.have.output(6)
  })
})
