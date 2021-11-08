import { consts } from './constants'
import { bnArg, expect, fromSigner, setupContract } from './helpers'

describe('MY_OWNABLE', () => {
  async function setup() {
    return setupContract('my_ownable', 'new')
  }

  it('OWNABLE - owner is by default contract deployer', async () => {
    const { query, defaultSigner: sender } = await setup()

    // Assert - Sender is by default the owner of the contract
    await expect(query.owner()).to.have.output(sender.address)
  })

  it('OWNABLE - only owner is allowed to mint', async () => {
    const {
      contract,
      query,
      defaultSigner: sender,
      accounts: [alice]
    } = await setup()

    // Arrange - Alice is not the owner hence minting should fail
    await expect(query.owner()).to.have.output(sender.address)
    await expect(contract.tx.mint(bnArg(0), 1)).to.eventually.be.fulfilled

    // Act & Assert - Alice can mint a token
    await expect(fromSigner(contract, alice.address).tx.mint(bnArg(0), 100)).to.eventually.be.rejected
  })

  it('OWNABLE - transfer ownership works', async () => {
    const {
      contract,
      query,
      tx,
      defaultSigner: sender,
      accounts: [alice]
    } = await setup()

    // Arrange - Alice is not the owner hence minting should fail
    await expect(query.owner()).to.have.output(sender.address)
    await expect(fromSigner(contract, alice.address).tx.mint(bnArg(0), 100)).to.eventually.be.rejected

    // Act - transfer ownership to Alice
    await tx.transferOwnership(alice.address)
    await expect(query.owner()).to.have.output(alice.address)

    // Assert - Alice can mint a token
    await expect(fromSigner(contract, alice.address).tx.mint(bnArg(0), 100)).to.eventually.be.fulfilled
    await expect(query.balanceOf(alice.address, bnArg(0))).to.have.output(100)
  })

  it('OWNABLE - renounce ownership works', async () => {
    const { query, tx, defaultSigner: sender } = await setup()

    // Arrange - Sender is the owner
    await expect(query.owner()).to.have.output(sender.address)

    // Act - Sender renounce his role
    await expect(tx.renounceOwnership()).to.eventually.be.fulfilled

    // Assert - Zero account is now the owner
    await expect(query.owner()).to.have.output(consts.EMPTY_ADDRESS)
  })

  it('OWNABLE - cannot renounce ownership if not owner', async () => {
    const {
      contract,
      query,
      defaultSigner: sender,
      accounts: [alice]
    } = await setup()

    // Arrange - Sender is the owner
    await expect(query.owner()).to.have.output(sender.address)

    // Act - Alice try to call renounce his role
    await expect(fromSigner(contract, alice.address).tx.renounceOwnership()).to.eventually.be.rejected

    // Assert - Sender is still the owner
    await expect(query.owner()).to.have.output(sender.address)
  })
})
