import { consts } from './constants'
import {expect, getSigners} from './helpers'
import BN from 'bn.js'
import {ApiPromise} from '@polkadot/api'
import Constructors from '../../typechain-generated/constructors/my_ownable'
import Contract from '../../typechain-generated/contracts/my_ownable'
import {Id, IdBuilder} from '../../typechain-generated/types-arguments/my_ownable'

describe('MY_OWNABLE', () => {
  async function setup() {
    // return setupContract('my_ownable', 'new')
    const api = await ApiPromise.create()

    const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]))
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
      accounts: signers,
      contractFactory,
      contract,
      one,
      query: contract.query,
      tx: contract.tx
    }

  }

  it('OWNABLE - owner is by default contract deployer', async () => {
    const { api, query, defaultSigner: sender } = await setup()

    // Assert - Sender is by default the owner of the contract
    await expect(query.owner()).to.have.output(sender.address)

    await api.disconnect()
  })

  it('OWNABLE - only owner is allowed to mint', async () => {
    const {
      api,
      contract,
      query,
      defaultSigner: sender,
      alice
    } = await setup()

    // Arrange - Alice is not the owner hence minting should fail
    await expect(query.owner()).to.have.output(sender.address)


    await expect(contract.tx.mint(
      sender.address, [[IdBuilder.U8(0), 1]]
    )).to.eventually.be.fulfilled

    // Act & Assert - Alice can mint a token
    // TODO: ? what should be here
    // await expect(contract.tx.mint(alice.address, bnArg(0), 100)).to.eventually.be.rejected

    await api.disconnect()
  })

  it('OWNABLE - transfer ownership works', async () => {
    const {
      api,
      contract,
      query,
      tx,
      defaultSigner: sender,
      accounts: [alice]
    } = await setup()

    const token = IdBuilder.U8(1)
    const ids_amounts: [Id, number][] = [[token, 123]]

    // Arrange - Alice is not the owner hence minting should fail
    await expect(query.owner()).to.have.output(sender.address)
    await expect(contract.withSigner(alice).tx.mint(alice.address, ids_amounts)).to.eventually.be.rejected
    const balanceBefore = await query.balanceOf(alice.address, token)
    expect(balanceBefore.value.unwrapRecursively().toString()).to.be.eq('0')

    // Act - transfer ownership to Alice96
    await tx.transferOwnership(alice.address)
    await expect(query.owner()).to.have.output(alice.address)

    // Assert - Alice can mint a token
    await contract.withSigner(alice).tx.mint(alice.address, ids_amounts)
    await expect(query.owner()).to.have.output(alice.address)
    const balanceAfter = await query.balanceOf(alice.address, token)
    expect(balanceAfter.value.unwrapRecursively().toString()).to.be.eq('123')

    await api.disconnect()
  })

  it('OWNABLE - renounce ownership works', async () => {
    const { api, query, tx, defaultSigner: sender } = await setup()

    // Arrange - Sender is the owner
    await expect(query.owner()).to.have.output(sender.address)

    // Act - Sender renounce his role
    await tx.renounceOwnership()

    // Assert - Zero account is now the owner
    await expect(query.owner()).to.have.output(consts.EMPTY_ADDRESS)

    await api.disconnect()
  })

  it('OWNABLE - cannot renounce ownership if not owner', async () => {
    const {
      api,
      contract,
      query,
      defaultSigner: sender,
      accounts: [alice]
    } = await setup()

    // Arrange - Sender is the owner
    await expect(query.owner()).to.have.output(sender.address)

    // Act - Alice try to call renounce his role
    await expect(contract.withSigner(alice).tx.renounceOwnership()).to.eventually.be.rejected

    // Assert - Sender is still the owner
    await expect(query.owner()).to.have.output(sender.address)

    await api.disconnect()
  })
})
