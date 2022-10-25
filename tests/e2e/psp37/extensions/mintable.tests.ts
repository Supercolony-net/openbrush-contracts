import {expect, getSigners} from '../../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP35 from '../../../../typechain-generated/constructors/my_psp37_mintable'
import ContractPSP35 from '../../../../typechain-generated/contracts/my_psp37_mintable'
import {IdBuilder} from '../../../../typechain-generated/types-arguments/my_psp37_mintable'

describe('MY_PSP37_MINTABLE', () => {
  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP35(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
    const contract = new ContractPSP35(contractAddress, defaultSigner, api)

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

  it('Mint works', async () => {
    const { contract, defaultSigner: sender, query, alice, close } = await setup()

    const token1 = IdBuilder.U8(0)
    const token2 = IdBuilder.U8(1)

    const amount1 = 1
    const amount2 = 10

    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(0)

    await expect(contract.tx.mint(sender.address, [[token1, amount1]])).to.eventually.be.fulfilled
    await expect(contract.tx.mint(sender.address, [[token2, amount2]])).to.eventually.be.fulfilled
    await expect(contract.tx.mint(alice.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(amount2)
    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(amount1)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(amount2)

    await close()
  })
})
