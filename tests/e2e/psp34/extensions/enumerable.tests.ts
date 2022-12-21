import {expect, getSigners} from '../../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP34 from '../../../../typechain-generated/constructors/my_psp34_enumerable'
import ContractPSP34 from '../../../../typechain-generated/contracts/my_psp34_enumerable'
import {IdBuilder} from '../../../../typechain-generated/types-arguments/my_psp34_enumerable'
import {IdBuilder as IdBuilderReturns} from '../../../../typechain-generated/types-returns/my_psp34_enumerable'

describe('MY_PSP34_ENUMERABLE', () => {
  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP34(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
    const contract = new ContractPSP34(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      bob,
      contract,
      query: contract.query,
      tx: contract.tx,
      close: async () => {
        await api.disconnect()
      }
    }
  }

  it('Enumerable should fail', async () => {
    const {
      contract,
      defaultSigner: sender,
      alice,
      query,
      close
    } = await setup()

    expect((await query.ownersTokenByIndex(sender.address, 0)).value.ok).to.be.undefined
    expect((await query.ownersTokenByIndex(alice.address, 0)).value.ok).to.be.undefined
  })

  it('Enumerable works', async () => {
    const {
      contract,
      defaultSigner: sender,
      alice,
      query,
      close
    } = await setup()

    await expect(contract.tx.ownersTokenByIndex(sender.address, 0)).to.eventually.be.rejected
    await expect(contract.tx.ownersTokenByIndex(alice.address, 0)).to.eventually.be.rejected

    const psp34_id1 = IdBuilder.U8(1)
    const psp34_id2 = IdBuilder.U8(2)

    await contract.tx.mint(alice.address, psp34_id1)
    await contract.tx.mint(alice.address, psp34_id2)

    expect((await query.tokenByIndex(0)).value.ok).to.be.deep.equal(IdBuilderReturns.U8(1))
    expect((await query.tokenByIndex(1)).value.ok).to.be.deep.equal(IdBuilderReturns.U8(2))
    expect((await query.ownersTokenByIndex(alice.address, 0)).value.ok).to.be.deep.equal(IdBuilderReturns.U8(1))
    expect((await query.ownersTokenByIndex(alice.address, 1)).value.ok).to.be.deep.equal(IdBuilderReturns.U8(2))

    await close()
  })

  it('Enumerable works after burn', async () => {
    const {
      contract,
      defaultSigner: sender,
      alice,
      query,
      close
    } = await setup()

    await expect(contract.tx.ownersTokenByIndex(sender.address, 0)).to.eventually.be.rejected
    await expect(contract.tx.ownersTokenByIndex(alice.address, 0)).to.eventually.be.rejected

    const psp34_id1 = IdBuilder.U8(1)
    const psp34_id2 = IdBuilder.U8(2)

    await contract.tx.mint(alice.address, psp34_id1)
    await contract.tx.mint(alice.address, psp34_id2)

    expect((await query.tokenByIndex(0)).value.ok).to.be.deep.equal(IdBuilderReturns.U8(1))
    expect((await query.tokenByIndex(1)).value.ok).to.be.deep.equal(IdBuilderReturns.U8(2))

    await contract.tx.burn(alice.address, psp34_id2)

    await contract.tx.ownersTokenByIndex(alice.address, 0)
    await expect(contract.tx.ownersTokenByIndex(alice.address, 1)).to.eventually.be.rejected

    await close()
  })
})
