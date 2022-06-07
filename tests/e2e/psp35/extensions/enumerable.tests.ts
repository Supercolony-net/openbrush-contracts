import { bnArg, expect, setupContract } from '../../helpers'
import { bnFromHex } from '@polkadot/util'
import BN from 'bn.js'

interface Result {
  ok: string;
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

    const token1 = bnArg(1)
    const token2 = bnArg(2)
    const amount1 = 1
    const amount2 = 20

    await expect(contract.tx.mint(alice.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    expect(bnFromHex(result((await query.tokenByIndex(0)).output?.toString()).ok)).equal(new BN(token1))
    expect(bnFromHex(result((await query.tokenByIndex(1)).output?.toString()).ok)).equal(new BN(token2))

    expect(bnFromHex(result((await query.ownersTokenByIndex(alice.address, 0)).output?.toString()).ok)).equal(new BN(token1))
    expect(bnFromHex(result((await query.ownersTokenByIndex(alice.address, 1)).output?.toString()).ok)).equal(new BN(token2))
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

    const token1 = bnArg(1)
    const token2 = bnArg(2)
    const amount1 = 1
    const amount2 = 20

    await expect(contract.tx.mint(alice.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    expect(bnFromHex(result((await query.tokenByIndex(0)).output?.toString()).ok)).equal(new BN(token1))
    expect(bnFromHex(result((await query.tokenByIndex(1)).output?.toString()).ok)).equal(new BN(token2))

    await expect(contract.tx.burn(alice.address, [[token2, amount2]])).to.eventually.be.fulfilled

    expect(bnFromHex(result((await query.ownersTokenByIndex(alice.address, 0)).output?.toString()).ok)).equal(new BN(token1))
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

    const token1 = bnArg(1)
    const token2 = bnArg(2)
    const amount1 = 1
    const amount2 = 20

    await expect(contract.tx.mint(sender.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    expect(bnFromHex(result((await query.ownersTokenByIndex(sender.address, 0)).output?.toString()).ok)).equal(new BN(token1))
    await expect(contract.tx.transfer(alice.address, token1, amount1, [])).to.eventually.be.fulfilled

    expect(bnFromHex(result((await query.ownersTokenByIndex(alice.address, 0)).output?.toString()).ok)).equal(new BN(token1))
    expect(bnFromHex(result((await query.ownersTokenByIndex(sender.address, 0)).output?.toString()).ok)).equal(new BN(token2))

    await expect(contract.tx.transfer(alice.address, token2, 10, [])).to.eventually.be.fulfilled

    expect(bnFromHex(result((await query.ownersTokenByIndex(sender.address, 0)).output?.toString()).ok)).equal(new BN(token2))
    expect(bnFromHex(result((await query.ownersTokenByIndex(alice.address, 1)).output?.toString()).ok)).equal(new BN(token2))
  })
})
