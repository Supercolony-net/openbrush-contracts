import { expect, getSigners } from './helpers'
import Constructors from '../../typechain-generated/constructors/my_payment_splitter'
import Contract from '../../typechain-generated/contracts/my_payment_splitter'
import {ApiPromise} from '@polkadot/api'

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

    return { api, contract, kayne, ian }
  }

  it('PAYMENT SPLITTER - init values', async () => {
    // Arrange - Create a contract
    const { api, contract, kayne, ian } = await setup()

    // Assert - Init with shares
    const kayneShares = (await contract.query.shares(kayne.address)).value.toNumber()
    await expect(kayneShares).to.be.eq(40)
    const ianShares = (await contract.query.shares(ian.address)).value.toNumber()
    await expect(ianShares).to.be.eq(60)
    const totalShares = (await contract.query.totalShares()).value.toNumber()
    await expect(totalShares).to.be.eq(100)
    await expect(contract.query.payee(0)).to.have.output(kayne.address)
    await expect(contract.query.payee(1)).to.have.output(ian.address)

    await api.disconnect()
  })

  it('PAYMENT SPLITTER - release a native token', async () => {
    // Arrange - Create a contract
    const { api, contract, kayne, ian } = await setup()

    // Act - Send native token and release them
    const totalReleasedBefore = (await contract.query.totalReleased()).value.toNumber()
    await expect(totalReleasedBefore).to.be.eq(0)
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

    await api.disconnect()
  })

  it('PAYMENT SPLITTER - release a native token using releaseAll function', async () => {
    // Arrange - Create a contract
    const { api, contract, kayne, ian } = await setup()

    // Act - Send native token and release them
    const totalReleasedBefore = (await contract.query.totalReleased()).value.toNumber()
    await expect(totalReleasedBefore).to.be.eq(0)
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

    await api.disconnect()
  })
})
