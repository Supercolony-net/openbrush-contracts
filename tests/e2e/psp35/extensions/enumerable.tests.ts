import { expect, setupContract } from '../../helpers'

interface Result {
  ok: Ok;
}

interface Ok {
  u8: number;
}

describe('MY_PSP35_ENUMERABLE', () => {
  async function setup() {
    return setupContract('my_psp35_enumerable', 'new')
  }

  function result(s: string | undefined) {
    const result: Result = s != null ? JSON.parse(s) : null
    return result
  }

  it('Enumerable should fail', async () => {
    const {
      defaultSigner: sender,
      accounts: [alice],
      query
    } = await setup()

    await expect(query.ownersTokenByIndex(sender.address, 0)).to.eventually.be.rejected
    await expect(query.ownersTokenByIndex(alice.address, 0)).to.eventually.be.rejected
  })

  it('Enumerable works', async () => {
    const {
      contract,
      defaultSigner: sender,
      accounts: [alice],
      query
    } = await setup()

    await expect(contract.tx.ownersTokenByIndex(sender.address, 0)).to.eventually.be.rejected
    await expect(contract.tx.ownersTokenByIndex(alice.address, 0)).to.eventually.be.rejected

    const token1 = {'u8': 1 }
    const token2 = {'u8': 2 }
    const amount1 = 1
    const amount2 = 20

    await expect(contract.tx.mint(alice.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    expect(result((await query.tokenByIndex(0)).output?.toString()).ok.u8).equal(1)
    expect(result((await query.tokenByIndex(1)).output?.toString()).ok.u8).equal(2)

    expect(result((await query.ownersTokenByIndex(alice.address, 0)).output?.toString()).ok.u8).equal(1)
    expect(result((await query.ownersTokenByIndex(alice.address, 1)).output?.toString()).ok.u8).equal(2)
  })

  it('Enumerable works after burn', async () => {
    const {
      contract,
      defaultSigner: sender,
      accounts: [alice],
      query
    } = await setup()

    await expect(contract.tx.ownersTokenByIndex(sender.address, 0)).to.eventually.be.rejected
    await expect(contract.tx.ownersTokenByIndex(alice.address, 0)).to.eventually.be.rejected

    const token1 = {'u8': 1}
    const token2 = {'u8': 2}
    const amount1 = 1
    const amount2 = 20

    await expect(contract.tx.mint(alice.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    expect(result((await query.tokenByIndex(0)).output?.toString()).ok.u8).equal(1)
    expect(result((await query.tokenByIndex(1)).output?.toString()).ok.u8).equal(2)

    await expect(contract.tx.burn(alice.address, [[token2, amount2]])).to.eventually.be.fulfilled

    expect(result((await query.ownersTokenByIndex(alice.address, 0)).output?.toString()).ok.u8).equal(1)
    await expect(contract.query.ownersTokenByIndex(alice.address, 1)).to.eventually.be.rejected
  })

  it('Enumerable transfer works', async () => {
    const {
      contract,
      defaultSigner: sender,
      accounts: [alice],
      query
    } = await setup()

    await expect(contract.tx.ownersTokenByIndex(sender.address, 0)).to.eventually.be.rejected
    await expect(contract.tx.ownersTokenByIndex(alice.address, 0)).to.eventually.be.rejected

    const token1 = {'u8': 1}
    const token2 = {'u8': 2}
    const amount1 = 1
    const amount2 = 20

    await expect(contract.tx.mint(sender.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    expect(result((await query.ownersTokenByIndex(sender.address, 0)).output?.toString()).ok.u8).equal(1)
    await expect(contract.tx.transfer(alice.address, token1, amount1, [])).to.eventually.be.fulfilled

    expect(result((await query.ownersTokenByIndex(alice.address, 0)).output?.toString()).ok.u8).equal(1)
    expect(result((await query.ownersTokenByIndex(sender.address, 0)).output?.toString()).ok.u8).equal(2)

    await expect(contract.tx.transfer(alice.address, token2, 10, [])).to.eventually.be.fulfilled

    expect(result((await query.ownersTokenByIndex(sender.address, 0)).output?.toString()).ok.u8).equal(2)
    expect(result((await query.ownersTokenByIndex(alice.address, 1)).output?.toString()).ok.u8).equal(2)
  })
})
