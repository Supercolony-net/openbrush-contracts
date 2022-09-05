import {expect, getSigners} from '../../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP35 from '../../../../typechain-generated/constructors/my_psp35_batch'
import ContractPSP35 from '../../../../typechain-generated/contracts/my_psp35_batch'
import {IdBuilder} from '../../../../typechain-generated/types-arguments/my_psp35_batch'

describe('MY_PSP37_BATCH', () => {
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

  it('Batch Transfer should work', async () =>{
    const { defaultSigner: sender, alice, query, tx, close } = await setup()

    const amount1 = 1
    const amount2 = 20

    await expect(tx.mint(sender.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    await expect(tx.batchTransfer(alice.address, [[token1, amount1], [token2, amount2]], [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(amount1)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(amount2)
    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(0)

    await close()
  })

  it(' Batch transfer from should work', async () => {
    const { defaultSigner: sender, alice, query, tx, close } = await setup()

    const amount1 = 1
    const amount2 = 20

    await expect(tx.mint(sender.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(0)
    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(amount2)

    await expect(tx.batchTransferFrom(sender.address, alice.address, [[token1, amount1], [token2, amount2]], [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(amount1)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(amount2)

    await close()
  })

  it('Batch transfer from with insufficient balance should fail', async () => {
    const { defaultSigner: sender, alice, tx, close } = await setup()
    
    const amount1 = 1
    const amount2 = 20

    await expect(tx.mint(sender.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled

    await expect(tx.batchTransferFrom(sender.address, alice.address, [[token1, amount1 + 1], [token2, amount2 + 1]], [])).to.eventually.be.rejected

    await close()
  })

  it('Batch transfer from with no approve should fail', async () => {
    const { defaultSigner: sender, alice, tx, close } = await setup()
    
    const amount1 = 1
    const amount2 = 20

    await expect(tx.mint(alice.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled
    await expect(tx.batchTransferFrom(alice.address, sender.address, [[token1, amount1], [token2, amount2]], [])).to.eventually.be.rejected

    await close()
  })

  it('Batch transfer from with approve should work', async () => {
    const { contract, defaultSigner: sender, alice, query, tx, close } = await setup()
    
    const amount1 = 1
    const amount2 = 20

    await expect(tx.mint(sender.address, [[token1, amount1], [token2, amount2]])).to.eventually.be.fulfilled
    await expect(tx.approve(alice.address, null, 1)).to.eventually.be.fulfilled

    await expect(contract.withSigner(alice).tx.batchTransferFrom(sender.address, alice.address, [[token1, amount1], [token2, amount2]], [])).to.eventually.be.fulfilled

    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(amount1)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(amount2)
    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(0)

    await close()
  })
})