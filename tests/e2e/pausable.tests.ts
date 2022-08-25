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
      api,
      contract
    }
  }

  it('Success flip when not paused', async () => {
    const { api, contract } = await setup()

    await expect(contract.tx.flip()).to.eventually.be.fulfilled

    await api.disconnect()
  })

  it('Success pause when not paused', async () => {
    const { api, contract } = await setup()

    await expect(contract.tx.pause()).to.eventually.be.fulfilled

    await api.disconnect()
  })

  it('Failed double pause', async () => {
    const { api, contract } = await setup()

    await expect(contract.tx.pause()).to.eventually.be.fulfilled
    await expect(contract.tx.pause()).to.eventually.be.rejected

    await api.disconnect()
  })

  it('Success pause and unpause', async () => {
    const { api, contract } = await setup()

    await expect(contract.tx.pause()).to.eventually.be.fulfilled
    await expect(contract.tx.unpause()).to.eventually.be.fulfilled

    await api.disconnect()
  })

  it('Failed unpause', async () => {
    const { api, contract } = await setup()

    await expect(contract.tx.unpause()).to.eventually.be.rejected

    await api.disconnect()
  })

  it('Failed flip when paused', async () => {
    const { api, contract } = await setup()

    await expect(contract.tx.pause()).to.eventually.be.fulfilled
    await expect(contract.tx.flip()).to.eventually.be.rejected

    await api.disconnect()
  })
})
