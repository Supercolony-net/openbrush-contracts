import { bnArg, expect, fromSigner, setupContract } from './helpers'
import { network, patract } from 'redspot'
const { api } = network

const { getRandomSigner } = patract
const KAYNE_SHARE = 40
const IAN_SHARE = 60

describe('MY_PAYMENT_SPLITTER', () => {
  async function setup() {
    const ian = await getRandomSigner()
    const kayne = await getRandomSigner()
    const contract = await setupContract('my_payment_splitter', 'new', [[kayne.address, KAYNE_SHARE], [ian.address, IAN_SHARE]])

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
    await expect(contract.contract.query.totalReleased()).to.have.output(0)
    await expect(contract.contract.tx.receive({ value: 1000000000000 })).to.eventually.be.fulfilled
    await expect(contract.contract.tx.release(kayne.address)).to.eventually.be.fulfilled
    await expect(contract.contract.tx.release(ian.address)).to.eventually.be.fulfilled

    // Assert - Ian must hold more tokens than kayne
    // @ts-ignore
    const totalReleased = Number.parseInt((await contract.contract.query.totalReleased()).output)
    // @ts-ignore
    const kayneReleased = Number.parseInt((await contract.contract.query.released(kayne.address)).output)
    // @ts-ignore
    const ianReleased = Number.parseInt((await contract.contract.query.released(ian.address)).output)
    expect(ianReleased > kayneReleased).to.true
    expect(kayneReleased).to.equal(totalReleased * KAYNE_SHARE / (KAYNE_SHARE + IAN_SHARE))
    expect(ianReleased).to.equal(totalReleased * IAN_SHARE / (KAYNE_SHARE + IAN_SHARE))
    expect(ianReleased + kayneReleased).to.equal(totalReleased)
  })

  it('PAYMENT SPLITTER - release a native token using releaseAll function', async () => {
    // Arrange - Create a contract
    const { contract, kayne, ian } = await setup()

    // Act - Send native token and release them
    await expect(contract.contract.query.totalReleased()).to.have.output(0)
    await expect(contract.contract.tx.receive({ value: 1000000000000 })).to.eventually.be.fulfilled
    await expect(contract.contract.tx.releaseAll()).to.eventually.be.fulfilled

    // Assert - Ian must hold more tokens than kayne
    // @ts-ignore
    const totalReleased = Number.parseInt((await contract.contract.query.totalReleased()).output)
    // @ts-ignore
    const kayneReleased = Number.parseInt((await contract.contract.query.released(kayne.address)).output)
    // @ts-ignore
    const ianReleased = Number.parseInt((await contract.contract.query.released(ian.address)).output)
    expect(ianReleased > kayneReleased).to.true
    expect(kayneReleased).to.equal(totalReleased * KAYNE_SHARE / (KAYNE_SHARE + IAN_SHARE))
    expect(ianReleased).to.equal(totalReleased * IAN_SHARE / (KAYNE_SHARE + IAN_SHARE))
    expect(ianReleased + kayneReleased).to.equal(totalReleased)
  })
})
