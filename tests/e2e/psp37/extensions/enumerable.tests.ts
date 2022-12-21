import {expect, getSigners} from '../../helpers'
import {IdBuilder as IdBuilderArgs} from '../../../../typechain-generated/types-arguments/my_psp37_enumerable'
import {IdBuilder} from '../../../../typechain-generated/types-returns/my_psp37_enumerable'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP37 from '../../../../typechain-generated/constructors/my_psp37_enumerable'
import ContractPSP37 from '../../../../typechain-generated/contracts/my_psp37_enumerable'


describe('MY_PSP37_ENUMERABLE', () => {
  const token1 = IdBuilderArgs.U8(1)
  const token2 = IdBuilderArgs.U8(2)
  const token1Return = IdBuilder.U8(1)
  const token2Return = IdBuilder.U8(2)

  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP37(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
    const contract = new ContractPSP37(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      bob,
      contract,
      query: contract.query,
      tx: contract.tx,
      close: async () => {
        await api.disconnect()
      }
    }
  }

  it('Enumerable should fail', async () => {
    const {
      defaultSigner: sender,
      alice,
      query,
      close
    } = await setup()

    await expect(query.ownersTokenByIndex(sender.address, 0)).to.have.output(null)
    await expect(query.ownersTokenByIndex(alice.address, 0)).to.have.output(null)

    await close()
  })

  it('Enumerable works', async () => {
    const {
      contract,
      defaultSigner: sender,
      alice,
      query,
      close
    } = await setup()

    await expect(query.ownersTokenByIndex(sender.address, 0)).to.have.output(null)
    await expect(query.ownersTokenByIndex(alice.address, 0)).to.have.output(null)

    const amount1 = 1
    const amount2 = 20

    await contract.tx.mint(alice.address, [[token1, amount1], [token2, amount2]])

    // expect(id((await query.tokenByIndex(0)).output?.toString()).u8).equal(1)
    expect((await query.tokenByIndex(0)).value.unwrapRecursively()).to.be.deep.equal(token1Return)
    // expect(id((await query.tokenByIndex(1)).output?.toString()).u8).equal(2)
    expect((await query.tokenByIndex(1)).value.unwrapRecursively()).to.be.deep.equal(token2Return)

    // expect(id((await query.ownersTokenByIndex(alice.address, 0)).output?.toString()).u8).equal(1)
    expect((await query.ownersTokenByIndex(alice.address, 0)).value.unwrapRecursively()).to.be.deep.equal(token1Return)
    // expect(id((await query.ownersTokenByIndex(alice.address, 1)).output?.toString()).u8).equal(2)
    expect((await query.ownersTokenByIndex(alice.address, 1)).value.unwrapRecursively()).to.be.deep.equal(token2Return)

    await close()
  })

  it('Enumerable works after burn', async () => {
    const {
      contract,
      defaultSigner: sender,
      alice,
      query,
      close
    } = await setup()

    await expect(query.ownersTokenByIndex(sender.address, 0)).to.have.output(null)
    await expect(query.ownersTokenByIndex(alice.address, 0)).to.have.output(null)

    const amount1 = 1
    const amount2 = 20

    await contract.tx.mint(alice.address, [[token1, amount1], [token2, amount2]])

    // expect(id((await query.tokenByIndex(0)).output?.toString()).u8).equal(1)
    expect((await query.tokenByIndex(0)).value.unwrapRecursively()).to.be.deep.equal(token1Return)
    // expect(id((await query.tokenByIndex(1)).output?.toString()).u8).equal(2)
    expect((await query.tokenByIndex(1)).value.unwrapRecursively()).to.be.deep.equal(token2Return)

    await contract.tx.burn(alice.address, [[token2, amount2]])

    // expect(id((await query.ownersTokenByIndex(alice.address, 0)).output?.toString()).u8).equal(1)
    expect((await query.ownersTokenByIndex(alice.address, 0)).value.unwrapRecursively()).to.be.deep.equal(token1Return)
    await expect(query.ownersTokenByIndex(alice.address, 1)).to.have.output(null)

    await close()
  })

  it('Enumerable transfer works', async () => {
    const {
      contract,
      defaultSigner: sender,
      alice,
      query,
      close
    } = await setup()

    await expect(query.ownersTokenByIndex(sender.address, 0)).to.have.output(null)
    await expect(query.ownersTokenByIndex(alice.address, 0)).to.have.output(null)

    const amount1 = 1
    const amount2 = 20

    await contract.tx.mint(sender.address, [[token1, amount1], [token2, amount2]])

    // expect(id((await query.ownersTokenByIndex(sender.address, 0)).output?.toString()).u8).equal(1)
    expect((await query.ownersTokenByIndex(sender.address, 0)).value.unwrapRecursively()).to.be.deep.equal(token1Return)

    await contract.tx.transfer(alice.address, token1, amount1, [])

    // expect(id((await query.ownersTokenByIndex(alice.address, 0)).output?.toString()).u8).equal(1)
    expect((await query.ownersTokenByIndex(alice.address, 0)).value.unwrapRecursively()).to.be.deep.equal(token1Return)
    // expect(id((await query.ownersTokenByIndex(sender.address, 0)).output?.toString()).u8).equal(2)
    expect((await query.ownersTokenByIndex(sender.address, 0)).value.unwrapRecursively()).to.be.deep.equal(token2Return)

    await contract.tx.transfer(alice.address, token2, 10, [])

    // expect(id((await query.ownersTokenByIndex(sender.address, 0)).output?.toString()).u8).equal(2)
    expect((await query.ownersTokenByIndex(sender.address, 0)).value.unwrapRecursively()).to.be.deep.equal(token2Return)
    // expect(id((await query.ownersTokenByIndex(alice.address, 1)).output?.toString()).u8).equal(2)
    expect((await query.ownersTokenByIndex(alice.address, 1)).value.unwrapRecursively()).to.be.deep.equal(token2Return)

    await close()
  })
})
