import { expect, getSigners } from './helpers'
import { network, patract } from 'redspot'
import Constructors from '../../typechain-generated/constructors/my_payment_splitter';
import Contract from '../../typechain-generated/contracts/my_payment_splitter';
import {ApiPromise} from '@polkadot/api';
const { api } = network

const { getRandomSigner } = patract
const KAYNE_SHARE = 40
const IAN_SHARE = 60

describe('MY_PAYMENT_SPLITTER', () => {
  async function setup() {
    const api = await ApiPromise.create()
    const ian = getSigners()[0]
    const kayne = getSigners()[1]
    const contractFactory = new Constructors(api, ian)
    const { address: contractAddress } = await contractFactory.new([[kayne.address, KAYNE_SHARE], [ian.address, IAN_SHARE]])
    const contract = new Contract(contractAddress, ian, api)

    return { contract, kayne, ian }
  }

  it('PAYMENT SPLITTER - init values', async () => {
    // Arrange - Create a contract
    const { contract, kayne, ian } = await setup()

    // Assert - Init with shares
    await expect(contract.query.shares(kayne.address)).to.have.output(40)
    await expect(contract.query.shares(ian.address)).to.have.output(60)
    await expect(contract.query.totalShares()).to.have.output(100)
    await expect(contract.query.payee(0)).to.have.output(kayne.address)
    await expect(contract.query.payee(1)).to.have.output(ian.address)
  })

  it('PAYMENT SPLITTER - release a native token', async () => {
    // Arrange - Create a contract
    const { contract, kayne, ian } = await setup()

    // Act - Send native token and release them
    await expect(contract.query.totalReleased()).to.have.output(0)
    await expect(contract.tx.receive({ value: 1000000000000 })).to.eventually.be.fulfilled
    await expect(contract.tx.release(kayne.address)).to.eventually.be.fulfilled
    await expect(contract.tx.release(ian.address)).to.eventually.be.fulfilled

    // Assert - Ian must hold more tokens than kayne
    const totalReleased = ((await contract.query.totalReleased()).value.toNumber())
    const kayneReleased = ((await contract.query.released(kayne.address)).value).toNumber()
    const ianReleased = ((await contract.query.released(ian.address)).value).toNumber()
    expect(ianReleased > kayneReleased).to.true
    expect(kayneReleased).to.equal(totalReleased * KAYNE_SHARE / (KAYNE_SHARE + IAN_SHARE))
    expect(ianReleased).to.equal(totalReleased * IAN_SHARE / (KAYNE_SHARE + IAN_SHARE))
    expect(ianReleased + kayneReleased).to.equal(totalReleased)
  })

  it('PAYMENT SPLITTER - release a native token using releaseAll function', async () => {
    // Arrange - Create a contract
    const { contract, kayne, ian } = await setup()

    // Act - Send native token and release them
    await expect(contract.query.totalReleased()).to.have.output(0)
    await expect(contract.tx.receive({ value: 1000000000000 })).to.eventually.be.fulfilled
    await expect(contract.tx.releaseAll()).to.eventually.be.fulfilled

    // Assert - Ian must hold more tokens than kayne
    const totalReleased = ((await contract.query.totalReleased()).value).toNumber()
    const kayneReleased = ((await contract.query.released(kayne.address)).value).toNumber()
    const ianReleased = ((await contract.query.released(ian.address)).value).toNumber()
    expect(ianReleased > kayneReleased).to.true
    expect(kayneReleased).to.equal(totalReleased * KAYNE_SHARE / (KAYNE_SHARE + IAN_SHARE))
    expect(ianReleased).to.equal(totalReleased * IAN_SHARE / (KAYNE_SHARE + IAN_SHARE))
    expect(ianReleased + kayneReleased).to.equal(totalReleased)
  })
})
