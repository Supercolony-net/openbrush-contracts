import {expect, getSigners} from '../helpers'

import { Roles } from '../constants'
import {ApiPromise} from '@polkadot/api'
import Constructors from '../../../typechain-generated/constructors/my_access_control'
import Contract from '../../../typechain-generated/contracts/my_access_control'
import {IdBuilder} from '../../../typechain-generated/types-arguments/my_access_control'

describe('MY_ACCESS_CONTROL', () => {
  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new Constructors(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
    const contract = new Contract(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      bob,
      contract,
      query: contract.query,
      tx: contract.tx
    }
  }

  it('ACCESS CONTROL - only minter role is allowed to mint', async () => {
    const {
      api,
      contract,
      query,
      tx,
      alice
    } = await setup()

    // Arrange - Alice doesn't have Minter role hence minting should fail
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)
    await expect(contract.withSigner(alice).tx.mint(alice.address, IdBuilder.U8(0))).to.eventually.be.rejected

    // Act - Grant Alice the minter role
    await expect(tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)

    // Assert - Alice can mint a token
    await expect(contract.withSigner(alice).tx.mint(alice.address, IdBuilder.U8(0))).to.eventually.be.fulfilled
    await expect(query.ownerOf(IdBuilder.U8(0))).to.have.output(alice.address)

    await api.disconnect()
  })


  it('ACCESS CONTROL - should grant initial roles to default signer', async () => {
    const { api, query, defaultSigner: sender } = await setup()

    // Assert - After sender has deployed a contract instance, Sender should has default roles
    await expect(query.hasRole(Roles.DefaultAdminRole, sender.address)).to.have.output(true)
    await expect(query.hasRole(Roles.Minter, sender.address)).to.have.output(true)

    await api.disconnect()
  })

  it('ACCESS CONTROL - should not grant initial roles for random role', async () => {
    const {
      api,
      query,
      alice
    } = await setup()

    // Assert - After sender has deployed a contract instance, Alice should not has any role
    await expect(query.hasRole(Roles.DefaultAdminRole, alice.address)).to.have.output(false)
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)

    await api.disconnect()
  })

  it('ACCESS CONTROL - should grant role', async () => {
    const {
      api,
      query,
      tx,
      alice
    } = await setup()

    // Arrange - Check that Alice has not a minter role
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)

    // Act - Grant Alice the minter Role
    await expect(tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled

    // Assert - Alice has minter role
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)

    await api.disconnect()
  })

  it('ACCESS CONTROL - should not change old roles after grant role', async () => {
    const {
      api,
      query,
      tx,
      defaultSigner,
      alice
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

    await api.disconnect()
  })

  it('ACCESS CONTROL - should revoke role', async () => {
    const {
      api,
      query,
      tx,
      alice
    } = await setup()

    // Arrange - Grant Alice minter role
    await expect(tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)

    // Act - Revoke Alice minter role
    await expect(tx.revokeRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled

    // Assert - Alice don't have minter role
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)

    await api.disconnect()
  })

  it('ACCESS CONTROL - should renounce role', async () => {
    const {
      api,
      query,
      contract,
      alice
    } = await setup()

    // Arrange - Grant Alice minter role
    await expect(contract.tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)

    // Act - Alice renounce his minter role
    await expect(contract.withSigner(alice).tx.renounceRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled

    // Assert - Alice don't have minter role
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)

    await api.disconnect()
  })

  it('ACCESS CONTROL - should reject when grant/revoke not by admin role', async () => {
    const {
      api,
      tx,
      contract,
      alice,
      bob
    } = await setup()

    // Assert - Only sender has admin role
    await expect(tx.grantRole(Roles.Minter, bob.address)).to.eventually.be.fulfilled

    // Act & Assert - Alice & Bob can't grant or revoke roles
    await expect(contract.withSigner(alice).tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.rejected
    await expect(contract.withSigner(alice).tx.revokeRole(Roles.Minter, bob.address)).to.eventually.be.rejected

    await api.disconnect()
  })

  it('ACCESS CONTROL - should reject when renounce not self role', async () => {
    const {
      api,
      tx,
      query,
      defaultSigner,
      contract,
      alice
    } = await setup()

    // Arrange - Grant Alice minter role
    await expect(tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)

    // Act & Assert - Sender calling renounce for Alice should fail
    await expect(contract.withSigner(defaultSigner).tx.renounceRole(Roles.Minter, alice.address)).to.eventually.be.rejected

    await api.disconnect()
  })

  it('ACCESS CONTROL - should reject burn if no minter role', async () => {
    const {
      api,
      tx,
      query,
      contract,
      alice
    } = await setup()

    // Assert - Grant Alice minter role & mint a token
    await expect(tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)
    await expect(contract.withSigner(alice).tx.mint(alice.address, IdBuilder.U8(0))).to.eventually.be.fulfilled
    await expect(query.ownerOf(IdBuilder.U8(0))).to.have.output(alice.address)

    // Act - revoke Alice minter role
    await expect(tx.revokeRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)

    // Assert - Alice cannot burn token
    await expect(contract.withSigner(alice).tx.burn(alice.address, IdBuilder.U8(0))).to.eventually.be.rejected

    await api.disconnect()
  })
})
