import { expect, setupContract } from '../../helpers'

interface Id {
  u8: number;
}

describe('MY_PSP35_ENUMERABLE', () => {
  async function setup() {
    return setupContract('my_psp35_enumerable', 'new')
  }

  function id(s: string | undefined) {
    const id: Id = s != null ? JSON.parse(s) : null
    return id
  }

  it('Enumerable should fail', async () => {
    const {
      defaultSigner: sender,
      accounts: [alice],
      query
    } = await setup()

    await expect(query.ownersTokenByIndex(sender.address, 0)).to.have.output(null)
    await expect(query.ownersTokenByIndex(alice.address, 0)).to.have.output(null)
  })

  it('Enumerable works', async () => {
    const {
      contract,
      defaultSigner: sender,
      accounts: [alice],
      query
    } = await setup()

    await expect(query.ownersTokenByIndex(sender.address, 0)).to.have.output(null)
    await expect(query.ownersTokenByIndex(alice.address, 0)).to.have.output(null)

    const token1 = {'u8': 1 }
    const token2 = {'u8': 2 }
    const amount1 = 1
    const amount2 = 20

    await expect(contract.tx.mint(alice.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    expect(id((await query.tokenByIndex(0)).output?.toString()).u8).equal(1)
    expect(id((await query.tokenByIndex(1)).output?.toString()).u8).equal(2)

    expect(id((await query.ownersTokenByIndex(alice.address, 0)).output?.toString()).u8).equal(1)
    expect(id((await query.ownersTokenByIndex(alice.address, 1)).output?.toString()).u8).equal(2)
  })

  it('Enumerable works after burn', async () => {
    const {
      contract,
      defaultSigner: sender,
      accounts: [alice],
      query
    } = await setup()

    await expect(query.ownersTokenByIndex(sender.address, 0)).to.have.output(null)
    await expect(query.ownersTokenByIndex(alice.address, 0)).to.have.output(null)

    const token1 = {'u8': 1}
    const token2 = {'u8': 2}
    const amount1 = 1
    const amount2 = 20

    await expect(contract.tx.mint(alice.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    expect(id((await query.tokenByIndex(0)).output?.toString()).u8).equal(1)
    expect(id((await query.tokenByIndex(1)).output?.toString()).u8).equal(2)

    await expect(contract.tx.burn(alice.address, [[token2, amount2]])).to.eventually.be.fulfilled

    expect(id((await query.ownersTokenByIndex(alice.address, 0)).output?.toString()).u8).equal(1)
    await expect(query.ownersTokenByIndex(alice.address, 1)).to.have.output(null)
  })

  it('Enumerable transfer works', async () => {
    const {
      contract,
      defaultSigner: sender,
      accounts: [alice],
      query
    } = await setup()

    await expect(query.ownersTokenByIndex(sender.address, 0)).to.have.output(null)
    await expect(query.ownersTokenByIndex(alice.address, 0)).to.have.output(null)

    const token1 = {'u8': 1}
    const token2 = {'u8': 2}
    const amount1 = 1
    const amount2 = 20

    await expect(contract.tx.mint(sender.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    expect(id((await query.ownersTokenByIndex(sender.address, 0)).output?.toString()).u8).equal(1)

    await expect(contract.tx.transfer(alice.address, token1, amount1, [])).to.eventually.be.fulfilled

    expect(id((await query.ownersTokenByIndex(alice.address, 0)).output?.toString()).u8).equal(1)
    expect(id((await query.ownersTokenByIndex(sender.address, 0)).output?.toString()).u8).equal(2)

    await expect(contract.tx.transfer(alice.address, token2, 10, [])).to.eventually.be.fulfilled

    expect(id((await query.ownersTokenByIndex(sender.address, 0)).output?.toString()).u8).equal(2)
    expect(id((await query.ownersTokenByIndex(alice.address, 1)).output?.toString()).u8).equal(2)
  })
})
