import { bnArg, expect, fromSigner, setupContract } from '../helpers'

import { Roles } from '../constants'

describe('MY_ACCESS_CONTROL', () => {
  async function setup() {
    return setupContract('my_access_control', 'new')
  }

  it('ACCESS CONTROL - only minter role is allowed to mint', async () => {
    const {
      contract,
      query,
      tx,
      accounts: [alice]
    } = await setup()

    // Arrange - Alice doesn't have Minter role hence minting should fail
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)
    await expect(fromSigner(contract, alice.address).tx.mint(alice.address, 1)).to.eventually.be.rejected

    // Act - Grant Alice the minter role
    await expect(tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)

    // Assert - Alice can mint a token
    await expect(fromSigner(contract, alice.address).tx.mint(alice.address, 0)).to.eventually.be.fulfilled
    await expect(query.ownerOf(0)).to.have.output(alice.address)
  })


  it('ACCESS CONTROL - should grant initial roles to default signer', async () => {
    const { query, defaultSigner: sender } = await setup()

    // Assert - After sender has deployed a contract instance, Sender should has default roles
    await expect(query.hasRole(Roles.DefaultAdminRole, sender.address)).to.have.output(true)
    await expect(query.hasRole(Roles.Minter, sender.address)).to.have.output(true)
  })

  it('ACCESS CONTROL - should not grant initial roles for random role', async () => {
    const {
      query,
      accounts: [alice]
    } = await setup()

    // Assert - After sender has deployed a contract instance, Alice should not has any role
    await expect(query.hasRole(Roles.DefaultAdminRole, alice.address)).to.have.output(false)
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)
  })

  it('ACCESS CONTROL - should grant role', async () => {
    const {
      query,
      tx,
      accounts: [alice]
    } = await setup()

    // Arrange - Check that Alice has not a minter role
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)

    // Act - Grant Alice the minter Role
    await expect(tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled

    // Assert - Alice has minter role
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)
  })

  it('ACCESS CONTROL - should not change old roles after grant role', async () => {
    const {
      query,
      tx,
      defaultSigner,
      accounts: [alice]
    } = await setup()

    // Arrange - Alice don't have minter role, sender has default roles
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)
    await expect(query.hasRole(Roles.DefaultAdminRole, defaultSigner.address)).to.have.output(true)
    await expect(query.hasRole(Roles.Minter, defaultSigner.address)).to.have.output(true)

    // Act - Grant Alice the minter role
    await expect(tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled

    // Assert - Alice has minter role, and sender still have gra
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)
    await expect(query.hasRole(Roles.DefaultAdminRole, defaultSigner.address)).to.have.output(true)
    await expect(query.hasRole(Roles.Minter, defaultSigner.address)).to.have.output(true)
  })

  it('ACCESS CONTROL - should revoke role', async () => {
    const {
      query,
      tx,
      accounts: [alice]
    } = await setup()

    // Arrange - Grant Alice minter role
    await expect(tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)

    // Act - Revoke Alice minter role
    await expect(tx.revokeRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled

    // Assert - Alice don't have minter role
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)
  })

  it('ACCESS CONTROL - should renounce role', async () => {
    const {
      query,
      contract,
      accounts: [alice]
    } = await setup()

    // Arrange - Grant Alice minter role
    await expect(contract.tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)

    // Act - Alice renounce his minter role
    await expect(fromSigner(contract, alice.address).tx.renounceRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled

    // Assert - Alice don't have minter role
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)
  })

  it('ACCESS CONTROL - should reject when grant/revoke not by admin role', async () => {
    const {
      tx,
      contract,
      accounts: [alice, bob]
    } = await setup()

    // Assert - Only sender has admin role
    await expect(tx.grantRole(Roles.Minter, bob.address)).to.eventually.be.fulfilled

    // Act & Assert - Alice & Bob can't grant or revoke roles
    await expect(fromSigner(contract, alice.address).tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.rejected
    await expect(fromSigner(contract, alice.address).tx.revokeRole(Roles.Minter, bob.address)).to.eventually.be.rejected
  })

  it('ACCESS CONTROL - should reject when renounce not self role', async () => {
    const {
      tx,
      query,
      defaultSigner,
      contract,
      accounts: [alice]
    } = await setup()

    // Arrange - Grant Alice minter role
    await expect(tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)

    // Act & Assert - Sender calling renounce for Alice should fail
    await expect(fromSigner(contract, defaultSigner.address).tx.renounceRole(Roles.Minter, alice.address)).to.eventually.be.rejected
  })

  it('ACCESS CONTROL - should reject burn if no minter role', async () => {
    const {
      tx,
      query,
      contract,
      accounts: [alice]
    } = await setup()

    // Assert - Grant Alice minter role & mint a token
    await expect(tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)
    await expect(contract.tx.mint(alice.address, 0)).to.eventually.be.fulfilled
    await expect(query.ownerOf(0)).to.have.output(alice.address)

    // Act - revoke Alice minter role
    await expect(tx.revokeRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)

    // Assert - Alice cannot burn token
    await expect(contract.tx.burn(alice.address, 0, 1)).to.eventually.be.rejected
  })
})
