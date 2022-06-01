import { expect, setupContract } from './helpers'

describe('MY_PAUSABLE', () => {
  async function setup() {
    return setupContract('my_pausable', 'new')
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
