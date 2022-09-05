import { consts } from '../constants'
import {expect, getSigners} from '../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP22 from '../../../typechain-generated/constructors/my_psp22_pallet'
import ContractPSP22 from '../../../typechain-generated/contracts/my_psp22_pallet'
import ConstructorsPSP22Receiver from '../../../typechain-generated/constructors/psp22_receiver'
import ContractPSP22Receiver from '../../../typechain-generated/contracts/psp22_receiver'

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

  async function setup_receiver() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP22Receiver(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
    const contract = new ContractPSP22Receiver(contractAddress, defaultSigner, api)

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

    expect((await query.balanceOf(sender.address)).value.toNumber()).to.have.output(1000)

    await api.disconnect()
  })

  it('Transfer adds amount to destination account', async () => {
    const {
      api,
      contract,
      alice: receiver
    } = await setup()

    await contract.tx.transfer(receiver.address, 7, [])
    await expect((await contract.query.balanceOf(receiver.address)).value.toNumber()).to.be.equal(7)
    await expect((await contract.query.balanceOf(contract.signer.address)).value.toNumber()).to.be.equal(1000 - 7) // =)

    await api.disconnect()
  })

  it('Transfers funds successfully if destination account is a receiver and supports transfers', async () => {
    const { api: api1, tx } = await setup()

    const { api: api2, contract } = await setup_receiver()

    await expect(tx.transfer(contract.address, 7, [])).to.eventually.be.fulfilled

    await api1.disconnect()
    await api2.disconnect()
  })

  it('Transfers funds successfully if destination account is a receiver a contract but not PSP22Receiver', async () => {
    const { api: api1, tx } = await setup()

    const { api: api2, contract } = await setup()

    await expect(tx.transfer(contract.address, 7, [])).to.eventually.be.fulfilled

    await api1.disconnect()
    await api2.disconnect()
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
