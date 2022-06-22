import { expect, setupContract } from '../../helpers'

import { Roles } from '../../constants'

describe('MY_ACCESS_CONTROL_ENUMERABLE', () => {
  async function setup() {
    return setupContract('my_access_control_enumerable', 'new')
  }

  it('ACCESS CONTROL ENUMERABLE - should have not member', async () => {
    const {
      query
    } = await setup()

    // Assert - No minter member for index 1
    await expect(query.getRoleMember(Roles.Minter, 1)).to.have.output(null)
  })

  it('ACCESS CONTROL ENUMERABLE - should get role member', async () => {
    const {
      defaultSigner: sender,
      query
    } = await setup()

    // Assert - Minter role for sender was granter in contract constructor
    const minter = await query.getRoleMember(Roles.Minter, 0)
    await expect(minter.output).equal(sender.address)
  })

  it('ACCESS CONTROL ENUMERABLE - should grant roles and get role members', async () => {
    const {
      accounts: [alice],
      query,
      tx
    } = await setup()

    // Arrange - Check that Alice has not a minter role
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)

    // Act - Grant Alice the minter Role
    await expect(tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled

    // Assert - Now Alice is the second on the minter list
    const minter = await query.getRoleMember(Roles.Minter, 1)
    await expect(minter.output).equal(alice.address)
  })

  it('ACCESS CONTROL ENUMERABLE - should revoke and count roles', async () => {
    const {
      defaultSigner: sender,
      accounts: [alice],
      query,
      tx
    } = await setup()

    // Arrange - Check that Alice has not a minter role, sender has a minter role
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)
    await expect(query.hasRole(Roles.Minter, sender.address)).to.have.output(true)
    await expect(query.getRoleMemberCount(Roles.Minter)).to.have.output(1)

    // Act - Revoke sender the minter role
    await expect(tx.revokeRole(Roles.Minter, sender.address)).to.eventually.be.fulfilled

    // Assert - no minter members
    await expect(query.getRoleMemberCount(Roles.Minter)).to.have.output(0)
  })
})
