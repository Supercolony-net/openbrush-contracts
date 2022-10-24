import {expect, getSigners} from '../../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP35 from '../../../../typechain-generated/constructors/my_psp35_burnable'
import ContractPSP35 from '../../../../typechain-generated/contracts/my_psp35_burnable'
import {IdBuilder} from '../../../../typechain-generated/types-arguments/my_psp35_burnable'

describe('MY_PSP37_BURNABLE', () => {
  const token1 = IdBuilder.U8(0)
  const token2 = IdBuilder.U8(1)

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

  it('Burn works', async () => {
    const { contract, query, defaultSigner: sender, alice, close } = await setup()
    
    const amount1 = 1
    const amount2 = 20
    
    await expect(contract.tx.mintTo(sender.address, [[token1, amount1], [token2, amount2]])).to.be.fulfilled
    await expect(contract.tx.transferFrom(sender.address, alice.address, token1, amount1, [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, null)).to.have.bnToNumber(1)
    await expect(query.balanceOf(alice.address, null)).to.have.bnToNumber(1)

    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(amount2)
    await expect(query.totalSupply(null)).to.have.bnToNumber(2)

    await expect(contract.tx.burn(sender.address, [[token2, amount2]])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, null)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, null)).to.have.bnToNumber(1)
    await expect(query.totalSupply(null)).to.have.bnToNumber(1)
    await expect(query.totalSupply(token2)).to.have.bnToNumber(0)
    await expect(query.totalSupply(token1)).to.have.bnToNumber(amount1)

    await expect(contract.tx.burn(alice.address, [[token1, amount1]])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(0)
    await expect(query.totalSupply(null)).to.have.bnToNumber(0)
    await expect(query.totalSupply(token1)).to.have.bnToNumber(0)
    await expect(query.totalSupply(token2)).to.have.bnToNumber(0)

    await close()
  })

  it('Burn batch works', async () => {
    const { contract, query, defaultSigner: sender, alice, close } = await setup()
    
    const amount1 = 1
    const amount2 = 10

    await expect(contract.tx.mintTo(sender.address, [[token1, amount1], [token2, 20]])).to.eventually.be.fulfilled

    await expect(contract.tx.transferFrom(sender.address, alice.address, token2, amount2, [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(amount2)
    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(amount2)

    await expect(contract.tx.burn(sender.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled
    await expect(contract.tx.burn(alice.address, [[token1, 0], [token2, amount2]])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(0)

    await close()
  })

  it('Burn insufficient balance should fail', async () => {
    const { contract, defaultSigner: sender, query, alice, close } = await setup()
    
    const amount1 = 1
    const amount2 = 20

    await expect(contract.tx.mintTo(sender.address, [[token1, amount1], [token2, amount2]])).to.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(amount2)
    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(0)

    await expect(contract.tx.burn(sender.address, [[token1, amount1 + 1], [token2, amount2]]))
      .to.eventually.be.rejected
    await expect(contract.tx.burn(sender.address, [[token1, amount1 + 1]]))
      .to.eventually.be.rejected

    await expect(contract.tx.burn(alice.address, [[token1, amount1 + 1], [token2, amount2]]))
      .to.eventually.be.rejected
    await expect(contract.tx.burn(alice.address, [[token1, amount1 + 1]]))
      .to.eventually.be.rejected

    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(amount2)
    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(0)

    await close()
  })
})
