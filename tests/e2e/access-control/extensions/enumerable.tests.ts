import {expect, getSigners} from '../../helpers'

import { Roles } from '../../constants'
import {ApiPromise} from '@polkadot/api'
import Constructors from '../../../../typechain-generated/constructors/my_access_control_enumerable'
import Contract from '../../../../typechain-generated/contracts/my_access_control_enumerable'

describe('MY_ACCESS_CONTROL_ENUMERABLE', () => {
  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[1]
    const alice = signers[0]

    const contractFactory = new Constructors(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
    const contract = new Contract(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      contract,
      query: contract.query,
      tx: contract.tx
    }
  }

  it('ACCESS CONTROL ENUMERABLE - should have not member', async () => {
    const {
      api,
      query
    } = await setup()

    // Assert - No minter member for index 1
    await expect(query.getRoleMember(Roles.Minter, 1)).to.have.output(null)

    await api.disconnect()
  })

  it('ACCESS CONTROL ENUMERABLE - should get role member', async () => {
    const {
      api,
      defaultSigner: sender,
      query
    } = await setup()

    // Assert - Minter role for sender was granter in contract constructor
    await expect(query.getRoleMember(Roles.Minter, 0)).to.have.output(sender.address)

    await api.disconnect()
  })

  it('ACCESS CONTROL ENUMERABLE - should grant roles and get role members', async () => {
    const {
      api,
      alice,
      query,
      tx
    } = await setup()

    // Arrange - Check that Alice has not a minter role
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)

    // Act - Grant Alice the minter Role
    await expect(tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.fulfilled

    // Assert - Now Alice is the second on the minter list
    const minter = await query.getRoleMember(Roles.Minter, 1)
    await expect(minter.value).equal(alice.address)

    await api.disconnect()
  })

  it('ACCESS CONTROL ENUMERABLE - should revoke and count roles', async () => {
    const {
      api,
      defaultSigner: sender,
      alice,
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

    await api.disconnect()
  })
})
