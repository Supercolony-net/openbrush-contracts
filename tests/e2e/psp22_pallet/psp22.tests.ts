import { consts } from '../constants'
import {expect, getSigners} from '../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP22 from '../../../typechain-generated/constructors/my_psp22_pallet'
import ContractPSP22 from '../../../typechain-generated/contracts/my_psp22_pallet'
describe('MY_PSP22_PALLET', () => {
  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP22(api, defaultSigner)
    const contractAddress = (await contractFactory.new(Math.floor(Math.random() * 1000) + 1, 1, 1000, {value: '10000000000000000'})).address
    const contract = new ContractPSP22(contractAddress, defaultSigner, api)

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

  it('Assigns initial balance', async () => {
    const { api, query, defaultSigner: sender } = await setup()

    expect((await query.balanceOf(sender.address)).value).to.be.bnToNumber(1000)

    await api.disconnect()
  })

  it('Transfer adds amount to destination account', async () => {
    const {
      api,
      contract,
      alice: receiver
    } = await setup()

    await contract.tx.transfer(receiver.address, 7, [])
    await expect((await contract.query.balanceOf(receiver.address)).value.unwrapRecursively().toNumber()).to.be.equal(7)
    await expect((await contract.query.balanceOf(contract.signer.address)).value.unwrapRecursively().toNumber()).to.be.equal(1000 - 7) // =)

    await api.disconnect()
  })

  it('Can not transfer above the amount', async () => {
    const {
      api,
      contract,
      alice: receiver
    } = await setup()

    await expect(contract.tx.transfer(receiver.address, 1007, [])).to.eventually.be.rejected

    await api.disconnect()
  })
})
