import { consts, Roles } from './constants'
import {bnArg, expect, fromSigner, setupContract} from './helpers'

describe('MY_ACCESS_CONTROL', () => {
  async function setup() {
    return setupContract('my_access_control', 'new')
  }

  it('ERC 721 - mint works', async () => {
    const {
      contract,
      query,
      defaultSigner: sender
    } = await setup()

    // Arrange - Ensure sender balance is 0
    await expect(query.balanceOf(sender.address)).to.have.output(0)

    // Act - Sender mint a token
    await expect(contract.tx.mint(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, sender.address, bnArg(0))

    // Assert - Sender balance is now 1
    await expect(query.balanceOf(sender.address)).to.have.output(1)
  })

  it('ERC 721 - mint existing should fail', async () => {
    const {
      contract,
      query,
      defaultSigner: sender
    } = await setup()

    // Arrange - Sender mint a token with id [0; 32] and have balance of 1
    await expect(contract.tx.mint(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, sender.address, bnArg(0))
    await expect(query.balanceOf(sender.address)).to.have.output(1)
    await expect(query.ownerOf(bnArg(0))).to.have.output(sender.address)

    // Act - Minting the id [0; 32] should fail
    await expect(contract.tx.mint(bnArg(0))).to.eventually.be.rejected

    // Assert - Alice balance is still 1
    await expect(query.balanceOf(sender.address)).to.have.output(1)
  })

  it('ERC 721 - approved transfer works', async () => {
    const {
      contract,
      query,
      defaultSigner: sender,
      accounts: [alice, bob]
    } = await setup()

    // Arrange - Sender mint a Token and Approve Alice as spender of this token
    await expect(contract.tx.mint(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, sender.address, bnArg(0))
    await expect(query.ownerOf(bnArg(0))).to.have.output(sender.address)
    await expect(contract.tx.approve(alice.address, bnArg(0)))
      .to.emit(contract, 'Approval').withArgs(sender.address, alice.address, bnArg(0))

    // Act - Alice transfer the token form sender to bob
    await expect(fromSigner(contract, alice.address).tx.transferFrom(sender.address, bob.address, bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(sender.address, bob.address, bnArg(0))

    // Assert - Bob is now owner of the token
    await expect(query.ownerOf(bnArg(0))).to.have.output(bob.address)
  })

  it('ERC 721 - approved for all works', async () => {
    const {
      contract,
      query,
      defaultSigner: sender,
      accounts: [alice, bob]
    } = await setup()

    // Arrange - Sender mint 2 tokens and Approve Alice to spend on all his tokens
    await expect(contract.tx.mint(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, sender.address, bnArg(0))
    await expect(contract.tx.mint(bnArg(1)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, sender.address, bnArg(1))
    await expect(query.balanceOf(sender.address)).to.have.output(2)
    await expect(contract.tx.setApprovalForAll(alice.address, true))
      .to.emit(contract, 'ApprovalForAll').withArgs(sender.address, alice.address, true)

    // Act - Alice Transfer the two tokens to bob
    await expect(fromSigner(contract, alice.address).tx.transferFrom(sender.address, bob.address, bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(sender.address, bob.address, bnArg(0))
    await expect(fromSigner(contract, alice.address).tx.transferFrom(sender.address, bob.address, bnArg(1)))
      .to.emit(contract, 'Transfer').withArgs(sender.address, bob.address, bnArg(1))

    // Assert - Bob owns the two tokens
    await expect(query.ownerOf(bnArg(0))).to.have.output(bob.address)
    await expect(query.ownerOf(bnArg(1))).to.have.output(bob.address)

    // Assert - Bob is now owner of the tokens:c check that sender cannot transfer tokens on his behalf
    await expect(fromSigner(contract, sender.address).tx.transferFrom(bob.address, alice.address, bnArg(0)))
      .to.eventually.be.rejected
  })

  it('ERC 721 - burn works', async () => {
    const {
      contract,
      query,
      defaultSigner: sender
    } = await setup()

    // Arrange - Alice mint a token & have a balance of 1
    await expect(contract.tx.mint(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, sender.address, bnArg(0))
    await expect(query.balanceOf(sender.address)).to.have.output(1)

    // Act - Alice burn the token
    await expect(contract.tx.burn(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(sender.address, consts.EMPTY_ADDRESS, bnArg(0))

    // Assert - Alice balance is 0
    await expect(query.balanceOf(sender.address)).to.have.output(0)
  })

  it('ACCESS CONTROL - only minter role is allowed to mint', async () => {
    const {
      contract,
      query,
      tx,
      accounts: [alice]
    } = await setup()

    // Arrange - Alice doesn't have Minter role hence minting should fail
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)
    await expect(fromSigner(contract, alice.address).tx.mint(bnArg(1))).to.eventually.be.rejected

    // Act - Grant Alice the minter role
    await tx.grantRole(Roles.Minter, alice.address)
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)

    // Assert - Alice can mint a token
    await expect(fromSigner(contract, alice.address).tx.mint(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, alice.address, bnArg(0))
    await expect(query.ownerOf(bnArg(0))).to.have.output(alice.address)
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
    await tx.grantRole(Roles.Minter, alice.address)

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
    await tx.grantRole(Roles.Minter, alice.address)

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
    await tx.grantRole(Roles.Minter, alice.address)
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)

    // Act - Revoke Alice minter role
    await tx.revokeRole(Roles.Minter, alice.address)

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
    await contract.tx.grantRole(Roles.Minter, alice.address)
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)

    // Act - Alice renounce his minter role
    await fromSigner(contract, alice.address).tx.renounceRole(Roles.Minter, alice.address)

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
    await tx.grantRole(Roles.Minter, bob.address)

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
    await tx.grantRole(Roles.Minter, alice.address)
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
    await tx.grantRole(Roles.Minter, alice.address)
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)
    await expect(fromSigner(contract, alice.address).tx.mint(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, alice.address, bnArg(0))
    await expect(query.ownerOf(bnArg(0))).to.have.output(alice.address)

    // Act - revoke Alice minter role
    await tx.revokeRole(Roles.Minter, alice.address)
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)

    // Assert - Alice cannot burn token
    await expect(fromSigner(contract, alice.address).tx.burn(alice.address, bnArg(0), 1)).to.eventually.be.rejected
  })
})
