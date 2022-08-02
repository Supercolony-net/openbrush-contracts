import {expect, getSigners} from './helpers'
import Constructors from '../../typechain-generated/constructors/my_pausable'
import {ApiPromise} from '@polkadot/api'
import Contract from '../../typechain-generated/contracts/my_pausable'

describe('MY_PAUSABLE', () => {
  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()

    const contractFactories = new Constructors(api, signers[0])

    const contractAddress = (await contractFactories.new()).address

    const contract = new Contract(contractAddress, signers[0], api)

    return {
      contract
    }
  }

  it('Success flip when not paused', async () => {
    const { contract } = await setup()

    await expect(contract.tx.flip()).to.eventually.be.fulfilled
  })

  it('Success pause when not paused', async () => {
    const { contract } = await setup()

    await expect(contract.tx.pause()).to.eventually.be.fulfilled
  })

  it('Failed double pause', async () => {
    const { contract } = await setup()

    await expect(contract.tx.pause()).to.eventually.be.fulfilled
    await expect(contract.tx.pause()).to.eventually.be.rejected
  })

  it('Success pause and unpause', async () => {
    const { contract } = await setup()

    await expect(contract.tx.pause()).to.eventually.be.fulfilled
    await expect(contract.tx.unpause()).to.eventually.be.fulfilled
  })

  it('Failed unpause', async () => {
    const { contract } = await setup()

    await expect(contract.tx.unpause()).to.eventually.be.rejected
  })

  it('Failed flip when paused', async () => {
    const { contract } = await setup()

    await expect(contract.tx.pause()).to.eventually.be.fulfilled
    await expect(contract.tx.flip()).to.eventually.be.rejected
  })
})
