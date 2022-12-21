import {expect, getSigners} from './../../helpers'
import {ApiPromise} from '@polkadot/api'
import Constructors from '../../../../typechain-generated/constructors/my_psp22_pausable'
import Contract from '../../../../typechain-generated/contracts/my_psp22_pausable'

describe('MY_PSP22_PAUSABLE', () => {
  async function setup() {
    // return await setupContract('my_psp22_pausable', 'new', '1000')
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new Constructors(api, defaultSigner)
    const contractAddress = (await contractFactory.new(1000)).address
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

  it('Can transfer when not paused', async () => {
    const { api, contract: pausable, query, defaultSigner: sender, alice: receiver } = await setup()

    // sender has 1000 tokens
    await expect(query.balanceOf(sender.address)).to.have.bnToNumber(1000)
    // receiver has 0 tokens
    await expect(query.balanceOf(receiver.address)).to.have.bnToNumber(0)

    // sender sends tokens to the receiver
    await pausable.tx.transfer(receiver.address, 100, [])

    // sender has 900 tokens
    await expect(query.balanceOf(sender.address)).to.have.bnToNumber(900)
    // receiver has 100 tokens
    await expect(query.balanceOf(receiver.address)).to.have.bnToNumber(100)

    await api.disconnect()
  })

  it('Can not transfer when paused', async () => {
    const { api, contract: pausable, query, defaultSigner: sender, alice: receiver } = await setup()

    // sender has 1000 tokens
    await expect(query.balanceOf(sender.address)).to.have.bnToNumber(1000)
    // receiver has 0 tokens
    await expect(query.balanceOf(receiver.address)).to.have.bnToNumber(0)
    // we pause the contract
    await pausable.tx.changeState()

    // sender sends tokens to the receiver
    await expect(pausable.tx.transfer(receiver.address, 100, [])).to.eventually.be.rejected

    // sender has 1000 tokens
    await expect(query.balanceOf(sender.address)).to.have.bnToNumber(1000)
    // receiver has 0 tokens
    await expect(query.balanceOf(receiver.address)).to.have.bnToNumber(0)

    await api.disconnect()
  })

})
